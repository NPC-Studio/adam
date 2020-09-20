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
mod runner;

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
        Input::Run(b) | Input::Release(b) => b,
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
        project_name: application_data.project_name,
        project_directory: application_data.current_directory,
        user_dir: user_data.user_dir,
        user_string: user_data.user_string,
        runtime_location: std::path::Path::new(gm_artifacts::RUNTIME_LOCATION).to_owned(),
        target_mask: gm_artifacts::TARGET_MASK,
        application_path: std::path::Path::new(gm_artifacts::APPLICATION_PATH).to_owned(),
        config: "debug".to_string(),
    };

    // make our dir
    let cache_folder = build_data
        .output_folder
        .join(&format!("{}/cache", build_data.output_kind));
    std::fs::create_dir_all(&cache_folder).unwrap();

    let gm_build = gm_artifacts::GmBuild::new(&build_data);
    let build_location = cache_folder.join("build.bff");

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

    // and the macros...
    let macros = gm_artifacts::GmMacros::new(&build_data);

    std::fs::write(
        &gm_build.macros,
        serde_json::to_string_pretty(&macros).unwrap(),
    )
    .unwrap();

    runner::run_command(&build_location, macros, options);
}
