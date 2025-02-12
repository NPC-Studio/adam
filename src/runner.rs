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
    run_options: &RunOptions,
) -> Child {
    // we do all build operations directly with the Gmac
    if *run_kind == RunKind::Build {
        #[cfg(target_os = "windows")]
        {
            let cache = macros.asset_compiler_cache_directory.join("cache");

            let mut gmac = std::process::Command::new(macros.asset_compiler_path.clone());
            gmac.raw_arg("-c")
                .raw_arg("--mv=1")
                .raw_arg("--zpex")
                .raw_arg("--iv=0")
                .raw_arg("--rv=0")
                .raw_arg("-j=8")
                .raw_arg(format!("--gn=\"{}\"", macros.project_name))
                .raw_arg(format!("--td=\"{}\"", macros.temp_directory))
                .raw_arg(format!("--cd=\"{}\"", cache))
                .raw_arg(format!("--rtp=\"{}\"", macros.runtime_location))
                .raw_arg(format!("--zpuf=\"{}\"", macros.user_directory))
                .raw_arg("--prefabs=\"\"")
                .raw_arg("/ffe=\"fm+Cfg==\"")
                .raw_arg("-m=windows")
                .raw_arg("--tgt=64")
                .raw_arg("--nodnd")
                .raw_arg(format!("--cfg=\"{}\"", run_options.task.config))
                .raw_arg(format!(
                    "-o=\"{}\"",
                    macros.asset_compiler_cache_directory.join("output")
                ))
                .raw_arg("-sh=True")
                .raw_arg("--cvm")
                .raw_arg(format!("--baseproject=\"{}\"", macros.base_project))
                .raw_arg(format!("\"{}\"", macros.project_full_filename))
                .raw_arg("--debug")
                .raw_arg("--bt=compile")
                .raw_arg("--rt=vm")
                .raw_arg("--64bitgame=true");

            return gmac
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("failed to spawn gmac process");
        }

        #[cfg(target_family = "unix")]
        panic!("Unix targets cannot execute a bare build!");
    }

    let word = match run_kind {
        RunKind::Run | RunKind::Test(_) => "Run",
        RunKind::Build => "PackageZip", // we do this as a BS option basically
        RunKind::Release => "PackageZip",
    };

    let mut igor = std::process::Command::new(macros.igor_path.clone());
    igor.arg("-j=8").arg(format!("-options={}", build_bff));

    // add the verbosity
    if run_options.task.verbosity > 1 {
        igor.arg("-v");
    }

    // add the platform
    igor.arg("--")
        .arg(gm_artifacts::PLATFORM_KIND.to_string())
        .arg(word)
        .stdout(std::process::Stdio::piped());

    if run_options.task.verbosity > 1 {
        println!("{:?}", igor);
    }

    igor.spawn().unwrap()
}
