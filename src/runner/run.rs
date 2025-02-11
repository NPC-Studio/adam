use camino::Utf8Path;

use super::{
    compiler_handler::CompilerHandler, compiler_handler::CompilerOutput, invoke_igor,
    printer::Printer,
};
use crate::{
    gm_artifacts::{self, PLATFORM_KIND},
    input::RunKind,
    runner::cache::Cache,
    RunOptions,
};
use std::{
    io::Lines,
    io::{BufRead, BufReader},
};

pub fn run_command(
    build_bff: &Utf8Path,
    macros: gm_artifacts::GmMacros,
    run_options: RunOptions,
    run_kind: &RunKind,
) -> bool {
    // and now let's set our kill cmd
    if run_options.task.close_on_sig_kill {
        ctrlc::set_handler(move || {
            use sysinfo::{ProcessExt, SystemExt};

            let mut system = sysinfo::System::new_all();
            system.refresh_processes();

            let needle = if cfg!(target_os = "windows") {
                "Runner.exe"
            } else {
                "Mac_Runner"
            };

            for process in system.processes_by_name(needle) {
                process.kill();
            }
        })
        .unwrap();
    }

    // read the cache if it doesn't exist...
    let final_output = {
        let sub_str = if run_options.task.yyc { "yyc" } else { "vm" };
        format!("./{}/{}", run_options.task.output_folder, sub_str)
    };

    let time = std::time::Instant::now();
    let mut child = if run_options.no_compile {
        let mut igor = std::process::Command::new(format!(
            "{}/{}/x64/Runner.exe  ",
            run_options.platform.runtime_location, PLATFORM_KIND,
        ));

        igor.arg("-game")
            .arg(format!("{}/output/data.win", final_output))
            .stdout(std::process::Stdio::piped());

        if run_options.task.verbosity > 0 {
            println!("{:?}", igor);
        }

        igor.spawn().unwrap()
    } else {
        invoke_igor(run_kind, &macros, build_bff, run_options.task.verbosity)
    };

    if run_options.task.verbosity > 0
        || *run_kind == RunKind::Release
        || *run_kind == RunKind::Build
    {
        let reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();
        for line in reader.map_while(Result::ok) {
            println!("{}", line.trim());
        }

        match child.wait() {
            Ok(e) => e.success(),
            Err(_) => false,
        }
    } else {
        let compiler_handler = if run_options.no_compile {
            CompilerHandler::new_re_run()
        } else {
            CompilerHandler::new_run()
        };
        // startup the printer in a separate thread...
        let project_dir = macros.project_dir.clone();
        let printer_handler =
            std::thread::spawn(move || Printer::new(&project_dir.join("scripts")));

        let cache_path = format!("{}/cache.toml", final_output);
        let mut cache: Cache = std::fs::read_to_string(&cache_path)
            .ok()
            .and_then(|txt| toml::from_str(&txt).ok())
            .unwrap_or_default();

        let output = compiler_handler.compile(
            &mut child,
            &macros.project_name,
            &macros.project_full_filename,
            run_kind,
            &run_options,
            &cache,
        );

        if !matches!(output, CompilerOutput::Errors(_)) {
            cache.time = time.elapsed();
            let cache = toml::to_string_pretty(&cache).unwrap();

            // not our business if this fails
            std::fs::write(cache_path, cache).unwrap();
        }

        let mut printer = printer_handler.join().unwrap();

        match output {
            CompilerOutput::Errors(e) => {
                for error in e {
                    printer.print_line(error);
                }

                false
            }
            CompilerOutput::SuccessAndRun(msgs) => {
                let mut reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();

                // skip the ****
                reader.next();

                // skip the annoying ass "controller"
                reader.next();

                // otherwise, print out some early messages...
                for msg in msgs {
                    printer.print_line(msg);
                }

                run_game(&mut reader, &mut printer, run_kind, &run_options)
                    || run_options.no_compile
            }
        }
    }
}

fn run_game(
    lines: &mut Lines<impl BufRead>,
    printer: &mut Printer,
    run_kind: &RunKind,
    run_options: &RunOptions,
) -> bool {
    let mut found = false;

    let kill_word = if matches!(run_kind, RunKind::Test(_)) {
        &run_options.task.test_success_needle
    } else {
        "Igor complete"
    };

    for line in lines.flatten() {
        if line.contains(kill_word) {
            found = true;
            break;
        }

        printer.print_line(line);
    }

    found
}
