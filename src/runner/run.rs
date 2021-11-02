use super::{
    compiler_handler::CompilerHandler, compiler_handler::CompilerOutput, invoke_release,
    invoke_rerun, invoke_run, printer::Printer,
};
use crate::{
    gm_artifacts,
    input::{RunKind, RunOptions},
    manifest,
};
use std::{
    io::Lines,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct RunCommand(pub(super) RunKind, pub(super) RunOptions);

impl std::fmt::Display for RunCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = match self.0 {
            RunKind::Run | RunKind::Build => "compile",
            RunKind::Release => "release",
        };

        write!(f, "{} {}", if self.1.yyc { "yyc" } else { "vm" }, word)
    }
}

pub fn run_command(
    build_bff: &Path,
    macros: gm_artifacts::GmMacros,
    sub_command: RunOptions,
    run_kind: RunKind,
) {
    let sub_command = RunCommand(run_kind, sub_command);
    let mut child = match sub_command.0 {
        RunKind::Run | RunKind::Build => invoke_run(&macros, build_bff, &sub_command),
        RunKind::Release => invoke_release(&macros, build_bff, &sub_command),
    };

    if sub_command.1.verbosity > 0 {
        let reader = BufReader::new(child.stdout.unwrap()).lines();
        for line in reader.flatten() {
            println!("{}", line.trim());
        }
    } else {
        let compiler_handler = match sub_command.0 {
            RunKind::Run => CompilerHandler::new_run(),
            RunKind::Build => CompilerHandler::new_build(),
            RunKind::Release => CompilerHandler::new_release(),
        };
        // startup the printer in a separate thread...
        let project_dir = macros.project_dir.clone();
        let printer_handler =
            std::thread::spawn(move || Printer::new(&project_dir.join("scripts")));

        let output = compiler_handler.compile(
            &mut child,
            &macros.project_name,
            &macros.project_full_filename,
            sub_command,
        );

        let mut printer = printer_handler.join().unwrap();

        match output {
            CompilerOutput::Errors(e) => {
                for error in e {
                    printer.print_line(error);
                }
                let cache_folder = build_bff.parent().unwrap();
                manifest::invalidate_manifest(cache_folder);
            }
            CompilerOutput::SuccessAndRun(msgs) => {
                let mut reader = BufReader::new(child.stdout.as_mut().unwrap()).lines();

                // skip the ****
                reader.next();

                // skip the annoying ass "controller"
                reader.next();

                for msg in msgs {
                    printer.print_line(msg);
                }

                run_game(&mut reader, &mut printer);
            }
            CompilerOutput::SuccessAndBuild => {}
        }
    }
}

pub fn rerun_old(
    gm_build: gm_artifacts::GmBuild,
    macros: &gm_artifacts::GmMacros,
    run_data: RunOptions,
) {
    #[cfg(target_os = "windows")]
    let mut child = invoke_rerun(run_data.x64_windows, &gm_build, macros);

    #[cfg(not(target_os = "windows"))]
    let mut child = invoke_rerun(&gm_build);
    #[cfg(not(target_os = "windows"))]
    // very good code!
    let _ = macros;

    // startup the printer in a separate thread...
    let project_dir = gm_build.project_dir.clone();
    let printer_handler = std::thread::spawn(move || Printer::new(&project_dir.join("scripts")));

    if run_data.verbosity > 0 {
        let reader = BufReader::new(child.stdout.unwrap()).lines();
        for line in reader.flatten() {
            println!("{}", line.trim());
        }
        return;
    }

    let compile_handler = CompilerHandler::new_rerun();
    let output = compile_handler.compile(
        &mut child,
        &gm_build.project_name,
        &gm_build.project_path,
        RunCommand(RunKind::Build, run_data),
    );

    let mut printer = printer_handler.join().unwrap();

    match output {
        CompilerOutput::Errors(e) => {
            for error in e {
                printer.print_line(error);
            }
            let cache_folder = gm_build.output_folder;
            manifest::invalidate_manifest(&cache_folder);
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

            run_game(&mut reader, &mut printer);
        }
        CompilerOutput::SuccessAndBuild => unimplemented!(),
    }
}

fn run_game(lines: &mut Lines<impl BufRead>, printer: &mut Printer) {
    for line in lines.flatten() {
        let message = line.to_string();

        if message == "Igor complete." {
            println!("adam complete");
            break;
        }

        printer.print_line(message);
    }
}
