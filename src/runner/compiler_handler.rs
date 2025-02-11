use crate::{gm_artifacts, input::RunKind, RunOptions};

use camino::Utf8Path;
use heck::ToTitleCase;
use indicatif::ProgressBar;
use std::{io::BufRead, io::BufReader, process::Child};

use super::Cache;

pub struct CompilerHandler {
    state: CompilerState,
}

enum CompilerState {
    Initialize,
    Compile(Vec<String>),
    ChunkBuilder,
    PreRunToMainLoop(Vec<String>),
}

impl CompilerHandler {
    pub fn new_run() -> Self {
        Self {
            state: CompilerState::Initialize,
        }
    }

    pub fn new_re_run() -> Self {
        Self {
            state: CompilerState::PreRunToMainLoop(vec![]),
        }
    }

    pub fn compile(
        mut self,
        child: &mut Child,
        project_name: &str,
        project_path: &Utf8Path,
        run_kind: &RunKind,
        run_options: &RunOptions,
        cache: &Cache,
    ) -> CompilerOutput {
        let progress_bar = ProgressBar::new(cache.time.as_secs());
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                .unwrap()
                .progress_chars("#> "),
        );
        progress_bar.println(format!(
            "{} {} ({})",
            console::style("Compiling").green().bright(),
            project_name.to_title_case(),
            project_path,
        ));

        // make a lil thread guy!
        let our_progress_bar = progress_bar.clone();
        std::thread::spawn(move || {
            let t = std::time::Instant::now();
            loop {
                if our_progress_bar.is_finished() {
                    return;
                }

                our_progress_bar.tick();
                our_progress_bar.set_position(t.elapsed().as_secs());

                std::thread::sleep(std::time::Duration::new(0, 16666666));
            }
        });

        let start_time = std::time::Instant::now();
        let lines = BufReader::new(child.stdout.as_mut().unwrap()).lines();

        for line in lines.map_while(|v| v.ok()) {
            let max_size = line.len().min(30);

            match &mut self.state {
                CompilerState::Initialize => {
                    progress_bar.set_message(line[..max_size].to_string());

                    if line.contains("[Compile]") {
                        self.state = CompilerState::Compile(vec![]);
                    }
                }
                CompilerState::Compile(e_msgs) => {
                    if line.contains("Error") {
                        e_msgs.push(line);
                        progress_bar.set_message("Collecting errors...");
                    } else if line.contains("Final Compile...finished") {
                        if e_msgs.is_empty() {
                            self.state = CompilerState::ChunkBuilder;
                        } else {
                            return CompilerOutput::Errors(e_msgs.clone());
                        }
                    } else if e_msgs.is_empty() {
                        progress_bar.set_message(line[..max_size].to_string());
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

                        self.state = CompilerState::PreRunToMainLoop(vec![]);
                    } else {
                        progress_bar.set_message(line[..max_size].to_string());
                    }
                }
                CompilerState::PreRunToMainLoop(startup_msgs) => {
                    const FINAL_EMITS: [&str; 11] = [
                        "Run_Start",
                        "CreateColPairs",
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
                            "{} {} {} {}:{} in {}",
                            console::style("Completed").green().bright(),
                            gm_artifacts::PLATFORM_KIND,
                            if run_options.task.yyc { "yyc" } else { "vm" },
                            run_kind,
                            console::style(&run_options.task.config).yellow().bright(),
                            indicatif::HumanDuration(std::time::Instant::now() - start_time)
                        );

                        return CompilerOutput::SuccessAndRun(startup_msgs.clone());
                    } else {
                        // we're in the final stage...
                        if FINAL_EMITS.iter().any(|&v| line.contains(v)) == false {
                            startup_msgs.push(line);
                        }
                    }
                }
            }
        }

        match self.state {
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
    SuccessAndRun(Vec<String>),
}
