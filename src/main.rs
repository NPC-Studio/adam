#![allow(clippy::bool_comparison)]

use input::Input;

mod input {
    mod cli;
    mod config_file;
    mod inputs;

    pub use inputs::{get_input, Input, RunData};
}
mod igor {
    mod application_data;
    pub use application_data::*;

    mod build;
    pub use build::*;

    mod user_data;
    pub use user_data::*;
}

mod gm_artifacts {
    mod steam_options;
    pub use steam_options::*;

    mod macros;
    pub use macros::*;

    mod preferences;
    pub use preferences::*;

    mod target_options;
    pub use target_options::*;

    mod platform;
    pub use platform::*;

    mod build;
    pub use build::*;
}
mod manifest;

mod runner {
    mod run;
    pub use run::{rerun_old, run_command};

    #[cfg(not(target_os = "windows"))]
    mod invoke_nix;
    #[cfg(not(target_os = "windows"))]
    pub(super) use invoke_nix::{invoke, invoke_rerun};

    #[cfg(target_os = "windows")]
    mod invoke_win;
    #[cfg(target_os = "windows")]
    pub(super) use invoke_win::{invoke, invoke_rerun};

    mod compiler_handler;
    mod gm_uri_parse;
    mod printer;
}

fn main() {
    let options = input::get_input();
    let application_data = match igor::ApplicationData::new(options.yyp_name()) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}\naborting",
                console::style("error").bright().red(),
                e
            );
            return;
        }
    };
    let user_data = match igor::UserData::new() {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}\naborting",
                console::style("error").bright().red(),
                e
            );
            return;
        }
    };

    // handle a clean, extract the build_data
    let run_data = match &options {
        Input::Run(b) | Input::Build(b) => b,
        Input::Clean(v, _) => {
            match std::fs::remove_dir_all(application_data.current_directory.join(v)) {
                Ok(()) => {}
                Err(e) => {
                    println!(
                        "{} on clean {}: {}",
                        console::style("error").bright().red(),
                        application_data.current_directory.join(v).display(),
                        e
                    );
                }
            }
            return;
        }
    };

    // check if we have a valid yyc bat
    if run_data.yyc && user_data.visual_studio_path.exists() == false {
        println!(
            "{}: no valid path to visual studio .bat build file. Supplied path in preferences was\n\
        \"{}\", but it did not exist. \n\
        To use yyc, we must have a visual studio .bat file.\n\
        Please specify a path in the Gms2 IDE. \n\
        For more information, see https://help.yoyogames.com/hc/en-us/articles/227860547-GMS2-Required-SDKs",
            console::style("error").bright().red(), user_data.visual_studio_path.display(),
        );

        return;
    }

    let build_data = igor::BuildData {
        output_folder: application_data
            .current_directory
            .join(&run_data.output_folder),
        output_kind: if run_data.yyc {
            igor::OutputKind::Yyc
        } else {
            igor::OutputKind::Vm
        },
        project_directory: application_data.current_directory,
        user_dir: user_data.user_dir,
        user_string: user_data.user_string,
        runtime_location: std::path::Path::new(gm_artifacts::RUNTIME_LOCATION).to_owned(),
        target_mask: gm_artifacts::TARGET_MASK,
        application_path: std::path::Path::new(gm_artifacts::APPLICATION_PATH).to_owned(),
        config: run_data.config.clone(),
        target_file: None,
        project_name: application_data.project_name,
    };
    let gm_build = gm_artifacts::GmBuild::new(&build_data);

    // make our dirs:
    let cache_folder = build_data
        .output_folder
        .join(&format!("{}/cache", build_data.output_kind));
    std::fs::create_dir_all(&cache_folder).unwrap();

    // check if we need to make a new build at all, or can go straight to the runner
    if run_data.ignore_cache == false
        && manifest::check_manifest(
            build_data.config.clone(),
            &build_data.project_directory,
            &cache_folder,
            &build_data.output_folder,
        )
    {
        runner::rerun_old(
            gm_build,
            match options {
                Input::Run(bd) => bd,
                Input::Build(bd) => bd,
                Input::Clean(_, _) => unimplemented!(),
            },
        );
        return;
    }

    let build_location = cache_folder.join("build.bff");

    // make and clear our tmps
    let tmp = build_data
        .output_folder
        .join(&format!("{}/tmp", build_data.output_kind));
    // clear the tmp
    if tmp.exists() {
        std::fs::remove_dir_all(&tmp).unwrap();
    }
    std::fs::create_dir_all(&tmp).unwrap();

    // write in the build.bff
    std::fs::write(
        &build_location,
        serde_json::to_string_pretty(&gm_build).unwrap(),
    )
    .unwrap();

    // write in the preferences
    let preferences = if run_data.yyc {
        gm_artifacts::GmPreferences::new(user_data.visual_studio_path)
    } else {
        gm_artifacts::GmPreferences::default()
    };
    std::fs::write(
        &gm_build.preferences,
        serde_json::to_string_pretty(&preferences).unwrap(),
    )
    .unwrap();

    // write in the targetoptions
    std::fs::write(
        &gm_build.target_options,
        serde_json::to_string_pretty(&gm_artifacts::GmTargetOptions {
            runtime: build_data.output_kind,
        })
        .unwrap(),
    )
    .unwrap();

    // write in the steamoptions
    std::fs::write(
        &gm_build.steam_options,
        serde_json::to_string_pretty(&gm_artifacts::GmSteamOptions::default()).unwrap(),
    )
    .unwrap();

    // and we write the macros finally
    let macros = gm_artifacts::GmMacros::new(&build_data);
    std::fs::write(
        &gm_build.macros,
        serde_json::to_string_pretty(&macros).unwrap(),
    )
    .unwrap();

    runner::run_command(&build_location, macros, options);
}
