#![allow(clippy::bool_comparison)]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]

use gm_artifacts::PlatformBuilder;

use crate::igor::{OutputKind, TargetFolders};

type AnyResult<T = ()> = color_eyre::eyre::Result<T>;

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
    pub(super) use invoke_nix::{invoke_release, invoke_rerun, invoke_run};

    #[cfg(target_os = "windows")]
    mod invoke_win;
    #[cfg(target_os = "windows")]
    pub(super) use invoke_win::{invoke_release, invoke_rerun, invoke_run};

    mod compiler_handler;
    mod gm_uri_parse;
    mod printer;
}

#[cfg(target_os = "windows")]
mod trailing_comma_util;

fn main() -> AnyResult {
    let (mut options, operation) = input::parse_inputs();

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
        if let Some(runtime_full_path) = &options.runtime_location_override {
            builder.set_runtime_override(Some(runtime_full_path.to_owned()));
        }

        builder.generate()
    };

    let application_data = match igor::ApplicationData::new(&options.yyp) {
        Ok(v) => v,
        Err(e) => {
            println!(
                "{}: {}",
                console::style("adam error").bright().red(),
                console::style(e).bold()
            );

            return Ok(());
        }
    };

    // check if we can make a user data raw...
    if let Err(e) = igor::load_user_data(
        &platform,
        &mut options.user_license_folder,
        &mut options.visual_studio_path,
    ) {
        println!(
            "{}: {}\naborting",
            console::style("adam error").bright().red(),
            console::style(e).bold()
        );
        return Ok(());
    };

    // handle a clean, extract the build_data
    let run_kind = match operation {
        input::Operation::Run(inner) => inner,
        input::Operation::Clean => {
            // clean up the output folder...
            if let Err(e) = std::fs::remove_dir_all(
                application_data
                    .current_directory
                    .join(options.output_folder()),
            ) {
                println!("{} on clean: {}", console::style("error").bright().red(), e);
            }
            return Ok(());
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
            return Ok(());
        }

        if let Some(visual_studio_path) = options.visual_studio_path {
            println!(
                "{}: {}.\n\
            Supplied path in preferences was \"{}\" but it did not exist.\n\
            To use yyc, we must have a visual studio .bat file.\n\
        Please specify a path in the Gms2 IDE. \n\
        For more information, see \
        https://help.yoyogames.com/hc/en-us/articles/227860547-GMS2-Required-SDKs",
                console::style("error").bright().red(),
                console::style("no valid path to visual studio .bat build file").bold(),
                visual_studio_path.display(),
            );

            return Ok(());
        }
    }

    let output_kind = if options.yyc {
        igor::OutputKind::Yyc
    } else {
        igor::OutputKind::Vm
    };

    let build_data = igor::BuildData {
        folders: TargetFolders::new(
            &application_data.current_directory,
            options.output_folder.as_deref(),
            output_kind,
            &application_data.project_name,
        )?,
        output_kind,
        project_filename: application_data.project_name,
        project_directory: application_data.current_directory,
        user_dir: platform.user_data.clone(),
        license_folder: options.user_license_folder.clone().unwrap(),
        runtime_location: platform.runtime_location.clone(),
        target_mask: platform.target_mask,
        application_path: platform.application_path.clone(),
        config: options.config.as_deref().unwrap_or("Default").to_owned(),
    };

    let gm_build = gm_artifacts::GmBuild::new(&build_data);
    let macros = gm_artifacts::GmMacros::new(&build_data);
    let visual_studio_path = options.visual_studio_path.clone();

    // check if we need to make a new build at all, or can go straight to the runner
    if options.ignore_cache == 0
        && cfg!(target_os = "windows")
        && manifest::check_manifest(
            build_data.config.clone(),
            &build_data.project_directory,
            &build_data.folders.cache,
            &build_data.folders.main,
        )
    {
        runner::rerun_old(gm_build, &macros, options);
        return Ok(());
    }

    // clear the temp files...
    build_data.folders.clear_tmp()?;

    let build_location = build_data.folders.cache.join("build.bff");

    // write in the build.bff
    std::fs::write(
        &build_location,
        serde_json::to_string_pretty(&gm_build).unwrap(),
    )
    .unwrap();

    // write in the preferences
    let preferences = if build_data.output_kind == OutputKind::Yyc {
        gm_artifacts::GmPreferences::new(visual_studio_path.unwrap())
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

    // write in the steamoptions -- we just use defaults here...
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

    Ok(())
}
