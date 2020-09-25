use super::run::RunCommand;
use crate::gm_artifacts;
use std::{path::Path, process::Child};

pub fn invoke(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &RunCommand,
) -> Child {
    let mut igor = std::process::Command::new(macros.igor_path.clone());
    igor.arg("-j=8")
        .arg(format!("-options={}", build_bff.display()));

    // add the verbosity
    if sub_command.1.verbosity > 1 {
        igor.arg("-v");
    }

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM.to_string())
        .arg("Run")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn invoke_rerun(gm_build: &gm_artifacts::GmBuild) -> Child {
    std::process::Command::new(gm_build.runtime_location.join("windows/Runner.exe"))
        .arg("-game")
        .arg(gm_build.compile_output_file_name.clone())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}
