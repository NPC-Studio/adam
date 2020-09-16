use std::io::{BufRead, BufReader};

mod input;
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
}

#[cfg(target_os = "macos")]
const TARGET_MASK: usize = 2;

#[cfg(target_os = "windows")]
const TARGET_MASK: usize = 64;

fn main() {
    let user_data = igor::UserData::new();
    let application_data = igor::ApplicationData::new();

    let build_bff = igor::BuildData {
        output_folder: application_data.output_folder,
        output_kind: igor::OutputKind::Vm,
        project_name: application_data.project_name,
        current_directory: application_data.current_directory,
        user_dir: user_data.user_dir,
        user_string: user_data.user_string,
        runtime_location: std::path::Path::new(
            "/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401",
        )
        .to_owned(),
        target_mask: TARGET_MASK,
        application_path: std::path::Path::new(
            "/Applications/GameMaker Studio 2.app/Contents/MonoBundle/GameMaker Studio 2.exe",
        )
        .to_owned(),
    };

    // make our dir
    let cache_folder = build_bff
        .output_folder
        .join(&format!("{}/cache", build_bff.output_kind));
    std::fs::create_dir_all(&cache_folder).unwrap();

    // write in the build.bff
    std::fs::write(
        cache_folder.join("build.bff"),
        serde_json::to_string_pretty(&build_bff.output_build_bff()).unwrap(),
    )
    .unwrap();

    // write in the preferences
    std::fs::write(
        cache_folder.join("preferences.json"),
        serde_json::to_string_pretty(&gm_artifacts::GmPreferences::default()).unwrap(),
    )
    .unwrap();

    // write in the targetoptions
    std::fs::write(
        cache_folder.join("targetoptions.json"),
        serde_json::to_string_pretty(&gm_artifacts::GmTargetOptions {
            runtime: build_bff.output_kind,
        })
        .unwrap(),
    )
    .unwrap();

    // write in the steamoptions
    std::fs::write(
        cache_folder.join("steam_options.yy"),
        serde_json::to_string_pretty(&gm_artifacts::GmSteamOptions::default()).unwrap(),
    )
    .unwrap();

    // and the macros...
    std::fs::write(
        cache_folder.join("macros.json"),
        serde_json::to_string_pretty(&gm_artifacts::build_macros(&build_bff)).unwrap(),
    )
    .unwrap();

    let igor_output = std::process::Command::new(
        "/Library/Frameworks/Mono.framework/Versions/Current/Commands/mono",
    )
    .arg("/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401/bin/Igor.exe")
    .arg("-j=8")
    .arg(format!(
        "-options={}",
        cache_folder.join("build.bff").display()
    ))
    .arg("--")
    .arg("Mac")
    .arg("Run")
    .stdout(std::process::Stdio::piped())
    .spawn()
    .unwrap();

    let reader = BufReader::new(igor_output.stdout.unwrap());

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
}
