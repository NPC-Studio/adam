use super::{
    compiler_handler::CompilerHandler, compiler_handler::CompilerOutput, invoke, invoke_rerun,
    printer::Printer,
};
use crate::{gm_artifacts, input::Input, input::RunData};
use std::{
    io::Lines,
    io::{BufRead, BufReader},
    path::Path,
};

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
        let compiler_handler = match sub_command.0 {
            RunKind::Run => CompilerHandler::new_run(),
            RunKind::Build => CompilerHandler::new_build(),
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

pub fn rerun_old(gm_build: gm_artifacts::GmBuild, run_data: RunData) {
    let mut child = invoke_rerun(&gm_build);
    // startup the printer in a separate thread...
    let project_dir = gm_build.project_dir.clone();
    let printer_handler = std::thread::spawn(move || Printer::new(&project_dir.join("scripts")));

    if run_data.verbosity > 0 {
        let reader = BufReader::new(child.stdout.unwrap()).lines();
        for line in reader {
            if let Ok(l) = line {
                println!("{}", l.trim());
            }
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
    for line in lines {
        if let Ok(l) = line {
            let message = l.to_string();

            if message == "Igor complete." {
                println!("adam complete");
                break;
            }

            printer.print_line(message);
        }
    }
}
