use crate::gm_artifacts;
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
    verbose: bool,
    sub_command: &str,
    build_kind: &str,
) {
    let mut reader = invoke(&macros.igor_path, build_bff, sub_command);
    if verbose {
        for line in reader {
            if let Ok(l) = line {
                println!("{}", l.trim());
            }
        }
    } else {
        let success = run_initial(
            &mut reader,
            &macros.project_name,
            &gm_artifacts::PLATFORM.to_string(),
            build_kind,
        );

        if success {
            // skip the ****
            reader.next();

            // skip the annoying ass "controller"
            reader.next();

            run_game(&mut reader);
        }
    }
}

#[cfg(target_os = "windows")]
fn invoke(igor_path: &Path, build_bff: &Path, sub_command: &str) -> Lines<BufReader<ChildStdout>> {
    let igor_output = std::process::Command::new(igor_path)
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
    platform: &str,
    build_kind: &str,
) -> bool {
    const RUN_INDICATOR: &str = "[Run]";
    const FINAL_EMITS: [&str; 4] = [
        "MainOptions.json",
        "Attempting to set gamepadcount",
        "hardware device",
        "Collision Event time",
    ];

    let progress_bar = ProgressBar::new(1000);
    progress_bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
            .progress_chars("#> "),
    );
    progress_bar.println(format!(
        "compiling {} on {} {}",
        project_name.to_title_case(),
        platform,
        build_kind
    ));

    for line in lines {
        if let Ok(l) = line {
            if l.contains("Error") {
                progress_bar.finish_with_message(l.trim());
                return false;
            }

            progress_bar.inc(10);
            let message = l.trim();
            

            if message.is_empty() == false {
                let max_size = message.len().min(30);

                progress_bar.set_message(&message[..max_size]);
            }

            if l == "Entering main loop." {
                progress_bar.finish_with_message("success");
                break;
            }
        }
    }

    println!("done with that shit");

    true
}

fn run_game(lines: &mut Lines<BufReader<ChildStdout>>) {
    for line in lines {
        if let Ok(l) = line {
            println!("{}", l);
        }
    }
}
