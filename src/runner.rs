use crate::{gm_artifacts, input::Input};
use heck::TitleCase;
use indicatif::ProgressBar;
use std::{
    io::Lines,
    io::{BufRead, BufReader},
    path::Path,
    process::ChildStdout,
};

pub fn run_command(
    build_bff: &Path,
    macros: gm_artifacts::GmMacros,
    verbose: usize,
    sub_command: Input,
) {
    let mut reader = invoke(&macros, build_bff, sub_command);
    if verbose > 0 {
        for line in reader {
            if let Ok(l) = line {
                println!("{}", l.trim());
            }
        }
    } else {
        let output = run_initial(
            &mut reader,
            &macros.project_name,
            &macros.project_full_filename,
            &gm_artifacts::PLATFORM.to_string(),
            build_kind,
        );

        if let Some(success) = output {
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

#[cfg(target_os = "windows")]
fn invoke(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &str,
) -> Lines<BufReader<ChildStdout>> {
    let igor_output = std::process::Command::new(macros.igor_path.clone())
        .arg("-j=8")
        .arg(format!("-options={}", build_bff.display()))
        .arg("--")
        .arg(gm_artifacts::PLATFORM.to_string())
        .arg(sub_command)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    BufReader::new(igor_output.stdout.unwrap()).lines()
}

#[cfg(not(target_os = "windows"))]
fn invoke(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &str,
) -> Lines<BufReader<ChildStdout>> {
    let igor_output = std::process::Command::new(gm_artifacts::MONO_LOCATION)
        .arg(macros.igor_path.clone())
        .arg("-j=8")
        .arg(format!("-options={}", build_bff.display()))
        .arg("--")
        .arg(gm_artifacts::PLATFORM.to_string())
        .arg(sub_command)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    BufReader::new(igor_output.stdout.unwrap()).lines()
}

fn run_initial(
    lines: &mut Lines<BufReader<ChildStdout>>,
    project_name: &str,
    project_path: &Path,
    platform: &str,
    build_kind: &str,
) -> Option<Vec<String>> {
    const RUN_INDICATOR: &str = "[Run]";
    const FINAL_EMITS: [&str; 7] = [
        "MainOptions.json",
        "Attempting to set gamepadcount",
        "hardware device",
        "Collision Event time",
        "Entering main loop.",
        "Total memory used",
        "********",
    ];

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
        console::style("Compiling").green(),
        project_name.to_title_case(),
        project_path.display()
    ));

    let mut in_final_stage = false;
    let mut startup_messages = vec![];

    let start_time = std::time::Instant::now();

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

                if message.contains(RUN_INDICATOR) {
                    in_final_stage = true;
                }
            } else {
                // we're in the final stage...
                if FINAL_EMITS.iter().any(|&v| message.contains(v)) == false {
                    startup_messages.push(message.to_owned());
                }

                if l == "Entering main loop." {
                    progress_bar.finish_and_clear();
                    println!(
                        "{} {} {} in {}",
                        console::style("Compiled").green(),
                        platform,
                        build_kind,
                        indicatif::HumanDuration(std::time::Instant::now() - start_time)
                    );
                    break;
                }
            }
        }
    }

    Some(startup_messages)
}

fn run_game(lines: &mut Lines<BufReader<ChildStdout>>) {
    const SHUTDOWN: [&str; 3] = [
        "Attempting to set gamepadcount to",
        "Not shutting down steam as it is not initialised",
        "Script_Free called",
    ];

    for line in lines {
        if let Ok(l) = line {
            let message = l.trim();
            if message.is_empty() {
                continue;
            }

            if SHUTDOWN.iter().any(|v| l.contains(v)) == false {
                println!("{}", message);
            }
        }
    }
}
