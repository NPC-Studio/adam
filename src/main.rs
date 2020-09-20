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
    let mut user_data = match igor::UserData::new() {
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
        Input::Clean => {
            match std::fs::remove_dir_all(application_data.output_folder) {
                Ok(()) => {}
                Err(e) => {
                    println!("{}: {}", console::style("error").bright().red(), e);
                }
            }
            return;
        }
    };

    // check if we have a valid yyc bat
    if run_data.yyc {
        if let Some(data) = run_data.visual_studio_path.clone() {
            if data.exists() {
                println!("{:?}", data);
                user_data.visual_studio_path = Some(data);
            }
        }

        if user_data.visual_studio_path.is_none() {
            println!(
            "{}: no valid path to visual studio .bat build file. to use \
        yyc, we must have a visual studio .bat file.\n\
        To specify path, do one of the following:\n\
        \tAdd it in the Gms2 IDE\n\
        \tSpecify it in a .adam config file with `visual_studio_path` as a key\n\
        \tPass it in as a flag with --visual_studio_path\n\
        The best option is to set it in the IDE directly, since this \
        path is local and, therefore, not Git safe.\n\
        For more information, see https://help.yoyogames.com/hc/en-us/articles/227860547-GMS2-Required-SDKs",
            console::style("error").bright().red(),
        );

            return;
        }
    }

    let build_data = igor::BuildData {
        output_folder: application_data.output_folder,
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
        gm_artifacts::GmPreferences::new(user_data.visual_studio_path.unwrap())
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
