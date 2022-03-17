use super::run::RunOptions;
use crate::gm_artifacts;
use std::{path::Path, process::Child};

pub fn invoke_release(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &RunOptions,
) -> Child {
    let mut igor = std::process::Command::new(macros.igor_path.clone());
    igor.arg("-j=8")
        .arg(format!("-options={}", build_bff.display()));

    // add the verbosity
    if sub_command.task.verbosity > 1 {
        igor.arg("-v");
    }

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg("PackageZip")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn invoke_run(
    macros: &gm_artifacts::GmMacros,
    build_bff: &Path,
    sub_command: &RunCommand,
) -> Child {
    let mut igor = std::process::Command::new(macros.igor_path.clone());
    igor.arg("-j=8")
        .arg(format!("-options={}", build_bff.display()));

    // add the verbosity
    if sub_command.task.verbosity > 1 {
        igor.arg("-v");
    }

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg("Run")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn invoke_rerun(
    mut use_x64_override: bool,
    gm_build: &gm_artifacts::GmBuild,
    macros: &gm_artifacts::GmMacros,
) -> Child {
    // figure out if we're x64 or not
    if use_x64_override == false {
        let options = std::fs::read_to_string(
            gm_build
                .project_dir
                .join("options/windows/options_windows.yy"),
        )
        .unwrap();

        let options =
            crate::trailing_comma_util::TRAILING_COMMA_UTIL.clear_trailing_comma(&options);
        let options: serde_json::Value = serde_json::from_str(&options).unwrap();
        let options_map = options.as_object().unwrap();

        use_x64_override = options_map
            .get("option_windows_use_x64")
            .unwrap()
            .as_bool()
            .unwrap();
    }

    if use_x64_override {
        std::process::Command::new(gm_build.runtime_location.join(&macros.x64_runner_path))
            .arg("-game")
            .arg(gm_build.compile_output_file_name.clone())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap()
    } else {
        std::process::Command::new(gm_build.runtime_location.join(&macros.runner_path))
            .arg("-game")
            .arg(gm_build.compile_output_file_name.clone())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap()
    }
}
