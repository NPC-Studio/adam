#![allow(clippy::bool_comparison)]

use input::Input;

mod input {
    mod cli;
    mod config_file;
    mod inputs;

    pub use inputs::{get_input, Input};
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
            println!("{}: {}\naborting", console::style("error").bright().red(), e);
            return;
        }
    };
    let user_data = igor::UserData::new();

    let build_data = match &options {
        Input::Run(b) | Input::Build(b) | Input::Release(b) => b,
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

    let build_data = igor::BuildData {
        output_folder: application_data.output_folder,
        output_kind: igor::OutputKind::Vm,
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
    std::fs::write(
        &gm_build.preferences,
        serde_json::to_string_pretty(&gm_artifacts::GmPreferences::default()).unwrap(),
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

    runner::run_command(
        &build_location,
        macros,
        false,
        "Run",
        &igor::OutputKind::Vm.to_string(),
    );
}
