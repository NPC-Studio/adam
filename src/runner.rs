use crate::{gm_artifacts, input::RunKind};
use std::process::Child;

mod run;
pub use run::run_command;

mod check_options;
pub use check_options::run_check;

mod compiler_handler;
mod printer;
mod run_options;

pub use run_options::*;

mod cache;
pub use cache::Cache;

use camino::Utf8Path;

pub fn invoke_igor(
    run_kind: &RunKind,
    macros: &gm_artifacts::GmMacros,
    build_bff: &Utf8Path,
    verbosity: u8,
) -> Child {
    let word = match run_kind {
        RunKind::Run | RunKind::Test(_) => "Run",
        RunKind::Build => "PackageNsis", // we do this as a BS option basically
        RunKind::Release => "PackageZip",
    };

    let mut igor = std::process::Command::new(macros.igor_path.clone());
    igor.arg("-j=8").arg(format!("-options={}", build_bff));

    // add the verbosity
    if verbosity > 1 {
        igor.arg("-v");
    }

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg(word)
        .stdout(std::process::Stdio::piped());

    if verbosity > 1 {
        println!("{:?}", igor);
    }

    igor.spawn().unwrap()
}
