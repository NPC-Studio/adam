use super::{invoke, invoke_rerun};
use crate::{gm_artifacts, input::Input, input::RunData};
use heck::TitleCase;
use indicatif::ProgressBar;
use std::{
    io::Lines,
    io::{BufRead, BufReader},
    path::Path,
    process::Child,
};

const FINAL_EMITS: [&str; 7] = [
    "MainOptions.json",
    "gamepadcount",
    "hardware device",
    "Collision Event time",
    "Entering main loop.",
    "Total memory used",
    "********",
];

pub struct RunCommand(pub(super) RunKind, pub(super) RunData);
impl From<Input> for RunCommand {
    fn from(o: Input) -> Self {
        match o {
            Input::Run(b) => RunCommand(RunKind::Run, b),
            Input::Build(b) => RunCommand(RunKind::Build, b),
            Input::Clean(_, _) => unimplemented!(),
        }
    }
}

impl std::fmt::Display for RunCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = match self.0 {
            RunKind::Run | RunKind::Build => "compile",
            // RunKind::Release => "release",
        };

        write!(f, "{} {}", if self.1.yyc { "yyc" } else { "vm" }, word)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum RunKind {
    Run,
    Build,
    // Release,
}

pub fn run_command(
    build_bff: &Path,
    macros: gm_artifacts::GmMacros,
    sub_command: impl Into<RunCommand>,
) {
    let sub_command: RunCommand = sub_command.into();
    let mut child = invoke(&macros, build_bff, &sub_command);

    if sub_command.1.verbosity > 0 {
        let reader = BufReader::new(child.stdout.unwrap()).lines();
        for line in reader {
            if let Ok(l) = line {
                println!("{}", l.trim());
            }
        }
    } else {
        let output = run_initial(
            &mut child,
            &macros.project_name,
            &macros.project_full_filename,
            sub_command,
            false,
        );

        if let Some(success) = output {
            let mut reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();

            // skip the ****
            reader.next();

            // skip the annoying ass "controller"
            reader.next();

            for msg in success {
                println!("{}", msg);
            }

            run_game(&mut reader);
        }
    }
}

pub fn rerun_old(gm_build: gm_artifacts::GmBuild, run_data: RunData) {
    let mut child = invoke_rerun(&gm_build);

    if run_data.verbosity > 0 {
        let reader = BufReader::new(child.stdout.unwrap()).lines();
        for line in reader {
            if let Ok(l) = line {
                println!("{}", l.trim());
            }
        }
        return;
    }

    let output = run_initial(
        &mut child,
        &gm_build.project_name,
        &gm_build.project_path,
        RunCommand(RunKind::Build, run_data),
        true,
    );

    if let Some(success) = output {
        let mut reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();

        // skip the ****
        reader.next();

        // skip the annoying ass "controller"
        reader.next();

        for msg in success {
            println!("{}", msg);
        }

        run_game(&mut reader);
    }
}

fn run_initial(
    child: &mut Child,
    project_name: &str,
    project_path: &Path,
    run_command: RunCommand,
    in_final_stage: bool,
) -> Option<Vec<String>> {
    const RUN_INDICATORS: [&str; 2] = ["[Run]", "Run_Start"];
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

    let mut in_final_stage = in_final_stage;
    let mut startup_messages = vec![];

    let start_time = std::time::Instant::now();

    let lines = BufReader::new(child.stdout.as_mut().unwrap()).lines();

    for line in lines {
        if let Ok(l) = line {
            if l.contains("Error: ") {
                progress_bar.finish_with_message(l.trim());
                return None;
            }

            progress_bar.inc(10);
            let message = l.trim();

            if message.is_empty() {
                continue;
            }

            if in_final_stage == false {
                let max_size = message.len().min(30);

                progress_bar.set_message(&message[..max_size]);

                if RUN_INDICATORS.iter().any(|v| message.contains(v)) {
                    // messy messy
                    if run_command.0 == RunKind::Build {
                        child.kill().unwrap();
                        break;
                    }
                    in_final_stage = true;
                }
            } else {
                // we're in the final stage...
                if FINAL_EMITS.iter().any(|&v| message.contains(v)) == false {
                    startup_messages.push(message.to_owned());
                }

                if l == "Entering main loop." {
                    progress_bar.finish_and_clear();
                    break;
                }
            }
        }
    }

    progress_bar.finish_and_clear();
    println!(
        "{} {} {}:{} in {}",
        console::style("Completed").green().bright(),
        gm_artifacts::PLATFORM.to_string(),
        run_command,
        console::style(&run_command.1.config).yellow().bright(),
        indicatif::HumanDuration(std::time::Instant::now() - start_time)
    );

    Some(startup_messages)
}

fn run_game(lines: &mut Lines<impl BufRead>) {
    const SHUTDOWN: [&str; 3] = [
        "Attempting to set gamepadcount to",
        "Not shutting down steam as it is not initialised",
        "Script_Free called",
    ];

    let stylers = vec![
        ColorStyler {
            matchers: vec!["error", "ERROR"],
            style: console::Style::new().red().bright(),
        },
        ColorStyler {
            matchers: vec!["warning", "WARNING"],
            style: console::Style::new().yellow().bright(),
        },
        ColorStyler {
            matchers: vec!["info", "INFO", "debug", "DEBUG"],
            style: console::Style::new().green().bright(),
        },
        ColorStyler {
            matchers: vec!["trace", "TRACE"],
            style: console::Style::new().dim(),
        },
    ];

    for line in lines {
        if let Ok(l) = line {
            let message = l.trim();
            if message.is_empty() {
                continue;
            }

            let mut message = message.to_string();

            for styler in stylers.iter() {
                styler.style(&mut message);
            }

            if message == "Igor complete." {
                println!("adam complete");
                break;
            }

            if SHUTDOWN.iter().any(|v| l.contains(v)) == false {
                println!("{}", message);
            }
        }
    }
}

pub struct ColorStyler {
    pub matchers: Vec<&'static str>,
    pub style: console::Style,
}

impl ColorStyler {
    pub fn style(&self, input: &mut String) {
        for m in self.matchers.iter() {
            if input.contains(m) {
                *input = input.replace(m, &self.style.apply_to(m).to_string());
            }
        }
    }
}
// // gml_Script_target_window_gui_Camera_gml_GlobalScript_CameraClass:77
// pub fn script_styler(input: &mut String) {

// }

/*
gml_Object_Game_Create_0:46
gml_Script_deserialize_Configuration_gml_GlobalScript_Configuration:35
gml_Object_Game_Create_0:256
gml_Script_Camera:370
gml_Script_Camera:373
gml_Script_set_view_size_Camera_gml_GlobalScript_CameraClass:110
gml_Script_target_mistria_gui_Camera_gml_GlobalScript_CameraClass:45
gml_Script_target_window_gui_Camera_gml_GlobalScript_CameraClass:77
gml_GlobalScript_Boombox_3740_play_track_Boombox_gml_GlobalScript_Boombox:141
*/
