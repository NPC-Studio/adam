use crate::{gm_artifacts, RunOptions};
use std::{path::Path, process::Child};

pub fn invoke_run(
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
    let o = igor
        .arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg("Run")
        .stdout(std::process::Stdio::piped());

    // add the verbosity
    if sub_command.task.verbosity > 1 {
        println!("igor: {:?}", o);
    }

    o.spawn().unwrap()
}

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

    igor.arg(format!(
        "--lf={}",
        macros.user_directory.join("licence.plist").display()
    ));

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg("PackageZip");

    igor.stdout(std::process::Stdio::piped()).spawn().unwrap()
}
