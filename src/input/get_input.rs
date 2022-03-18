use std::fmt;

use camino::Utf8Path;
use clap::Parser;
use color_eyre::Help;

use crate::{AnyResult, PlatformOptions, RunOptions, TaskOptions, DEFAULT_PLATFORM_DATA};

use super::{
    cli::{self, ClapOperation},
    config_file,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum Operation {
    Run(RunKind),
    Clean,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum RunKind {
    Run,
    Build,
    Release,
    Test,
}

impl RunKind {
    /// Returns `true` if the run kind is [`Test`].
    ///
    /// [`Test`]: RunKind::Test
    pub fn is_test(&self) -> bool {
        matches!(self, Self::Test)
    }
}

impl fmt::Display for RunKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self {
            RunKind::Run | RunKind::Build => "compile",
            RunKind::Release => "release",
            RunKind::Test => "test",
        };

        f.pad(word)
    }
}

pub fn parse_inputs() -> AnyResult<(RunOptions, Operation)> {
    let mut runtime_options = {
        let platform: PlatformOptions = PlatformOptions {
            gms2_application_location: DEFAULT_PLATFORM_DATA.stable_application_path.into(),
            runtime_location: DEFAULT_PLATFORM_DATA.stable_runtime_location.into(),
            visual_studio_path: Default::default(),
            user_license_folder: Default::default(),
            home_dir: DEFAULT_PLATFORM_DATA.home_dir.clone(),
            compiler_cache: DEFAULT_PLATFORM_DATA.stable_cached_data.clone(),
        };
        let task = TaskOptions::default();

        RunOptions { task, platform }
    };
    config_file::ConfigFile::find_config()
        .unwrap_or_default()
        .write_to_options(&mut runtime_options);

    let value: cli::InputOpts = cli::InputOpts::parse();
    let (cli_options, operation) = match value.subcmd {
        ClapOperation::Run(b) => (b, Operation::Run(RunKind::Run)),
        ClapOperation::Build(b) => (b, Operation::Run(RunKind::Build)),
        ClapOperation::Release(b) => (b, Operation::Run(RunKind::Release)),
        ClapOperation::Test(b) => (b, Operation::Run(RunKind::Test)),
        ClapOperation::Clean(co) => (
            cli::CliOptions {
                output_folder: co.output_folder,
                ..Default::default()
            },
            Operation::Clean,
        ),
    };

    // write them cli_options down!
    cli_options.write_to_options(&mut runtime_options);

    // check if we can make a user data raw...
    load_user_data(&mut runtime_options)?;

    Ok((runtime_options, operation))
}

/// Loads in the license folder path and the visual studio path.
pub fn load_user_data(options: &mut RunOptions) -> AnyResult {
    // user has loaded all of these!
    if options.platform.user_license_folder.exists()
        && options.platform.user_license_folder.exists()
    {
        return Ok(());
    }

    // check for early exit...
    if options.task.no_user_folder {
        let msg = if cfg!(target_os = "windows") {
            "`no-user-folder` is set, but either `user-license-folder` or `visual-studio-path` is not set."
        } else {
            "`no-user-folder` is set, but `user-license-folder` is not set."
        };

        println!(
            "{}: {}",
            console::style("adam error").bright().red(),
            console::style(msg).bold()
        );

        std::process::exit(1);
    }

    let um_json_path = options.platform.compiler_cache.join("um.json");
    let um_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&um_json_path)
            .with_note(|| format!("Could not read path {}", um_json_path))?,
    )
    .with_note(|| "Couldn't parse `um.json` file.")?;

    let user_id: usize = um_json.get("userID").unwrap().as_str().unwrap().parse()?;
    let user_name = um_json
        .get("login")
        .unwrap()
        .as_str()
        .unwrap()
        .split('@')
        .next()
        .unwrap()
        .to_owned();

    if options.platform.user_license_folder.exists() == false {
        options.platform.user_license_folder = options
            .platform
            .compiler_cache
            .join(format!("{}_{}", user_name, user_id));
    }

    // we need a visual studio path...
    if cfg!(target_os = "windows") && options.platform.visual_studio_path.exists() == false {
        // the ide can give us one...
        let new_path = std::fs::read_to_string(
            options
                .platform
                .compiler_cache
                .join(&format!("{}_{}/local_settings.json", user_name, user_id)),
        )
        .ok()
        .and_then(|data| {
            let local_settings: serde_json::Value = serde_json::from_str(&data).unwrap();

            local_settings
                .get("machine.Platform Settings.Windows.visual_studio_path")
                .map(|v| {
                    let v = v.as_str().unwrap();
                    Utf8Path::new(v).to_owned()
                })
        })
        .unwrap_or_else(|| {
            Utf8Path::new("C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/vcvars32.bat")
                .to_owned()
        });

        options.platform.visual_studio_path = new_path;
    }

    Ok(())
}
