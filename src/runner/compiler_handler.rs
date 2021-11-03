use crate::gm_artifacts;

use super::run::RunCommand;
use heck::TitleCase;
use indicatif::ProgressBar;
use std::{io::BufRead, io::BufReader, path::Path, process::Child};

pub struct CompilerHandler(CompilerState, bool);

enum CompilerState {
    Initialize,
    Compile(Vec<String>),
    ChunkBuilder,
    PreRunToMainLoop(Vec<String>),
}

impl CompilerHandler {
    pub fn new_run() -> Self {
        Self(CompilerState::Initialize, false)
    }

    pub fn new_build() -> Self {
        Self(CompilerState::Initialize, true)
    }

    pub fn new_release() -> Self {
        Self(CompilerState::Initialize, false)
    }

    #[cfg(target_os = "windows")]
    pub fn new_rerun() -> Self {
        Self(CompilerState::PreRunToMainLoop(vec![]), false)
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new_rerun() -> Self {
        Self(CompilerState::ChunkBuilder, false)
    }

    pub fn compile(
        mut self,
        child: &mut Child,
        project_name: &str,
        project_path: &Path,
        run_command: RunCommand,
    ) -> CompilerOutput {
        let progress_bar = ProgressBar::new(1000);
        progress_bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                .progress_chars("#> "),
        );
        progress_bar.enable_steady_tick(100);
        progress_bar.println(format!(
            "{} {} ({})",
            console::style("Compiling").green().bright(),
            project_name.to_title_case(),
            project_path.display()
        ));

        let start_time = std::time::Instant::now();
        let lines = BufReader::new(child.stdout.as_mut().unwrap()).lines();

        for line in lines.filter_map(|v| v.ok()) {
            let max_size = line.len().min(30);

            match &mut self.0 {
                CompilerState::Initialize => {
                    progress_bar.set_message(&line[..max_size]);

                    if line.contains("[Compile]") {
                        self.0 = CompilerState::Compile(vec![]);
                        progress_bar.set_position(progress_bar.position().max(250));
                    } else {
                        progress_bar.inc(20);
                    }
                }
                CompilerState::Compile(e_msgs) => {
                    if line.contains("Error") {
                        e_msgs.push(line);
                        progress_bar.set_message("Collecting errors...");
                    } else if line.contains("Final Compile...finished") {
                        progress_bar.set_position(progress_bar.position().max(500));
                        if e_msgs.is_empty() {
                            self.0 = CompilerState::ChunkBuilder;
                        } else {
                            return CompilerOutput::Errors(e_msgs.clone());
                        }
                    } else if e_msgs.is_empty() {
                        progress_bar.set_message(&line[..max_size]);
                    } else {
                        progress_bar.inc(20);
                    }
                }
                CompilerState::ChunkBuilder => {
                    #[cfg(target_os = "windows")]
                    const CHUNK_ENDER: &str = "Igor complete";

                    #[cfg(not(target_os = "windows"))]
                    const CHUNK_ENDER: &str = "Finished PrepareGame()";

                    // we're in the final stage...
                    if line.contains(CHUNK_ENDER) {
                        progress_bar.set_message("adam compile complete");
                        if self.1 {
                            progress_bar.finish_and_clear();
                            if let Err(e) = child.kill() {
                                println!(
                                    "{}: could not kill the compiler process, {}",
                                    console::style("error").red().bright(),
                                    e
                                );
                            }
                            progress_bar.finish_and_clear();
                            println!(
                                "{} {} {}:{} in {}",
                                console::style("Completed").green().bright(),
                                gm_artifacts::PLATFORM_KIND.to_string(),
                                run_command,
                                console::style(
                                    &run_command.1.config.as_deref().unwrap_or("Default")
                                )
                                .yellow()
                                .bright(),
                                indicatif::HumanDuration(std::time::Instant::now() - start_time)
                            );

                            return CompilerOutput::SuccessAndBuild;
                        } else {
                            progress_bar.set_position(progress_bar.position().max(750));
                            self.0 = CompilerState::PreRunToMainLoop(vec![]);
                        }
                    } else {
                        progress_bar.set_message(&line[..max_size]);
                        progress_bar.inc(10);
                    }
                }
                CompilerState::PreRunToMainLoop(startup_msgs) => {
                    const FINAL_EMITS: [&str; 10] = [
                        "Run_Start",
                        "[Run]",
                        "MainOptions.json",
                        "gamepadcount",
                        "hardware device",
                        "Collision Event time",
                        "Entering main loop.",
                        "Total memory used",
                        "Texture #",
                        "********",
                    ];

                    if line == "Entering main loop." || line == "Igor complete." {
                        progress_bar.finish_and_clear();
                        println!(
                            "{} {} {}:{} in {}",
                            console::style("Completed").green().bright(),
                            gm_artifacts::PLATFORM_KIND.to_string(),
                            run_command,
                            console::style(&run_command.1.config.as_deref().unwrap_or("Default"))
                                .yellow()
                                .bright(),
                            indicatif::HumanDuration(std::time::Instant::now() - start_time)
                        );

                        return CompilerOutput::SuccessAndRun(startup_msgs.clone());
                    } else {
                        // we're in the final stage...
                        if FINAL_EMITS.iter().any(|&v| line.contains(v)) == false {
                            startup_msgs.push(line);
                        } else {
                            progress_bar.set_message(&line);
                            progress_bar.inc(25);
                        }
                    }
                }
            }
        }

        match self.0 {
            CompilerState::Compile(msgs) | CompilerState::PreRunToMainLoop(msgs) => {
                CompilerOutput::Errors(msgs)
            }
            _ => CompilerOutput::Errors(vec!["adam error: unexpected end of compiler messages. \
            Are you on an unsupported platform?"
                .to_string()]),
        }
    }
}

pub enum CompilerOutput {
    Errors(Vec<String>),
    SuccessAndBuild,
    SuccessAndRun(Vec<String>),
}
