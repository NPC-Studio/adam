#![allow(clippy::bool_comparison)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]

use gm_artifacts::PlatformBuilder;

mod input {
    mod cli;
    mod config_file;
    mod get_input;
    pub use cli::RunOptions;
    pub use get_input::{parse_inputs, Operation, RunKind};
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

#[cfg(target_os = "windows")]
mod trailing_comma_util;

fn main() {
    let (options, operation) = input::parse_inputs();

    // build our platform handle here
    let platform = {
        let mut builder = PlatformBuilder::new();
        if options.beta {
            builder.set_beta();
        }
        if let Some(install) = &options.gms2_install_location {
            builder.set_app_override(Some(install.to_owned()));
        }
        if let Some(runtime) = &options.runtime {
            builder.set_runtime_name(runtime.to_owned());
        }

        builder.generate()
    };

    let application_data = match igor::ApplicationData::new(&options.yyp) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}\naborting",
                console::style("adam error").bright().red(),
                console::style(format!("reading application data {}", e)).bold()
            );
            return;
        }
    };
    let user_data = match igor::UserData::new(&platform) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}\naborting",
                console::style("adam error").bright().red(),
                console::style(format!("reading user data {}", e)).bold()
            );
            return;
        }
    };

    // handle a clean, extract the build_data
    let run_kind = match operation {
        input::Operation::Run(inner) => inner,
        input::Operation::Clean => {
            match std::fs::remove_dir_all(
                application_data.current_directory.join(
                    options
                        .output_folder
                        .unwrap_or_else(|| std::path::Path::new("target").to_owned()),
                ),
            ) {
                Ok(()) => {}
                Err(e) => {
                    println!("{} on clean: {}", console::style("error").bright().red(), e);
                }
            }
            return;
        }
    };

    // check if we have a valid yyc bat
    if options.yyc {
        if cfg!(not(target_os = "windows")) {
            println!(
                "{}: {}\nPlease log a feature request at https://github.com/NPC-Studio/adam/issues",
                console::style("adam error",).bright().red(),
                console::style("adam does not support macOS YYC compilation, yet.").bold(),
            );
            return;
        }

        if user_data.visual_studio_path.exists() == false {
            println!(
                "{}: {}.\n\
            Supplied path in preferences was \"{}\" but it did not exist.\n\
            To use yyc, we must have a visual studio .bat file.\n\
        Please specify a path in the Gms2 IDE. \n\
        For more information, see \
        https://help.yoyogames.com/hc/en-us/articles/227860547-GMS2-Required-SDKs",
                console::style("error").bright().red(),
                console::style("no valid path to visual studio .bat build file").bold(),
                user_data.visual_studio_path.display(),
            );

            return;
        }
    }

    let build_data = igor::BuildData {
        output_folder: application_data.current_directory.join(
            &options
                .output_folder
                .as_deref()
                .unwrap_or_else(|| std::path::Path::new("target"))
                .to_owned(),
        ),
        output_kind: if options.yyc {
            igor::OutputKind::Yyc
        } else {
            igor::OutputKind::Vm
        },
        project_directory: application_data.current_directory,
        user_dir: user_data.user_dir,
        user_string: user_data.user_string,
        runtime_location: platform.runtime_location.clone(),
        target_mask: platform.target_mask,
        application_path: platform.application_path.clone(),
        config: options.config.as_deref().unwrap_or("Default").to_owned(),
        target_file: None,
        project_filename: application_data.project_name,
    };

    let gm_build = gm_artifacts::GmBuild::new(&build_data, &platform);

    // make our dirs:
    let cache_folder = build_data
        .output_folder
        .join(&format!("{}/cache", build_data.output_kind));
    std::fs::create_dir_all(&cache_folder).unwrap();

    let macros = gm_artifacts::GmMacros::new(&build_data);

    // check if we need to make a new build at all, or can go straight to the runner
    if options.ignore_cache == 0
        && cfg!(target_os = "windows")
        && manifest::check_manifest(
            build_data.config.clone(),
            &build_data.project_directory,
            &cache_folder,
            &build_data.output_folder,
        )
    {
        runner::rerun_old(gm_build, &macros, options);
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
    let preferences = if options.yyc {
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
    std::fs::write(
        &gm_build.macros,
        serde_json::to_string_pretty(&macros).unwrap(),
    )
    .unwrap();

    runner::run_command(&build_location, macros, options, run_kind);
}
