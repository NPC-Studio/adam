use super::{
    compiler_handler::CompilerHandler, compiler_handler::CompilerOutput, invoke_release,
    invoke_rerun, invoke_run, printer::Printer,
};
use crate::{gm_artifacts, input::RunKind, manifest, RunOptions};
use std::{
    io::Lines,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run_command(
    build_bff: &Path,
    macros: gm_artifacts::GmMacros,
    run_options: RunOptions,
    run_kind: RunKind,
) -> bool {
    let mut child = match run_kind {
        RunKind::Run | RunKind::Build | RunKind::Test => {
            invoke_run(&macros, build_bff, &run_options)
        }
        RunKind::Release => invoke_release(&macros, build_bff, &run_options),
    };

    if run_options.task.verbosity > 0 || run_kind == RunKind::Release {
        let reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();
        for line in reader.flatten() {
            println!("{}", line.trim());
        }

        match child.wait() {
            Ok(e) => e.success(),
            Err(_) => false,
        }
    } else {
        let compiler_handler = if run_kind == RunKind::Build {
            CompilerHandler::new_build()
        } else {
            CompilerHandler::new_run()
        };
        // startup the printer in a separate thread...
        let project_dir = macros.project_dir.clone();
        let printer_handler =
            std::thread::spawn(move || Printer::new(&project_dir.join("scripts")));

        let output = compiler_handler.compile(
            &mut child,
            &macros.project_name,
            &macros.project_full_filename,
            run_kind,
            &run_options,
        );

        let mut printer = printer_handler.join().unwrap();

        match output {
            CompilerOutput::Errors(e) => {
                for error in e {
                    printer.print_line(error);
                }
                let cache_folder = build_bff.parent().unwrap();
                manifest::invalidate_manifest(cache_folder);

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
            }
            CompilerOutput::SuccessAndBuild => true,
        }
    }
}

pub fn rerun_old(
    gm_build: gm_artifacts::GmBuild,
    macros: &gm_artifacts::GmMacros,
    run_data: RunOptions,
) -> bool {
    #[cfg(target_os = "windows")]
    let mut child = invoke_rerun(run_data.task.x64_windows, &gm_build, macros);

    #[cfg(not(target_os = "windows"))]
    let mut child = invoke_rerun(&gm_build);
    #[cfg(not(target_os = "windows"))]
    // very good code!
    let _ = macros;

    // startup the printer in a separate thread...
    let project_dir = gm_build.project_dir.clone();
    let printer_handler = std::thread::spawn(move || Printer::new(&project_dir.join("scripts")));

    if run_data.task.verbosity > 0 {
        let reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();
        for line in reader.flatten() {
            println!("{}", line.trim());
        }

        return match child.wait() {
            Ok(e) => e.success(),
            Err(_) => false,
        };
    }

    let compile_handler = CompilerHandler::new_rerun();
    let output = compile_handler.compile(
        &mut child,
        &gm_build.project_name,
        &gm_build.project_path,
        RunKind::Build,
        &run_data,
    );

    let mut printer = printer_handler.join().unwrap();

    match output {
        CompilerOutput::Errors(e) => {
            for error in e {
                printer.print_line(error);
            }
            let cache_folder = gm_build.output_folder;
            manifest::invalidate_manifest(&cache_folder);

            false
        }
        CompilerOutput::SuccessAndRun(msgs) => {
            let mut reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();

            // skip the ****
            reader.next();

            // skip the annoying ass "controller"
            reader.next();

            // startup the printer...
            let mut printer = Printer::new(&gm_build.project_dir.join("scripts"));

            for msg in msgs {
                printer.print_line(msg);
            }

            run_game(&mut reader, &mut printer, RunKind::Build, &run_data)
        }
        CompilerOutput::SuccessAndBuild => unimplemented!(),
    }
}

fn run_game(
    lines: &mut Lines<impl BufRead>,
    printer: &mut Printer,
    run_kind: RunKind,
    run_options: &RunOptions,
) -> bool {
    let mut found = false;

    let kill_word = if run_kind == RunKind::Test {
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
