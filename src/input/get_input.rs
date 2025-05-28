use std::fmt;

use camino::{Utf8Path, Utf8PathBuf};
use color_eyre::Help;

use crate::{AnyResult, RunOptions};

use super::cli::ClapOperation;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Operation {
    Run(RunKind),
    Check,
    Clean,
}

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum RunKind {
    Run,
    Build,
    Release,
    Test(String),
}

impl RunKind {
    /// Returns `true` if the run kind is [`Test`].
    ///
    /// [`Test`]: RunKind::Test
    pub fn is_test(&self) -> bool {
        matches!(self, Self::Test(_))
    }
}

impl fmt::Display for RunKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = match self {
            RunKind::Run | RunKind::Build => "compile",
            RunKind::Release => "release",
            RunKind::Test(test_word) => {
                if test_word.is_empty() {
                    "test"
                } else {
                    return write!(f, "test:{}", test_word);
                }
            }
        };

        f.pad(word)
    }
}

pub fn parse_inputs(
    clap_op: ClapOperation,
    mut runtime_options: RunOptions,
    check_options: &mut Option<Utf8PathBuf>,
) -> AnyResult<(RunOptions, Operation)> {
    let (build_options, operation) = match clap_op {
        ClapOperation::Run(b) => (b, Operation::Run(RunKind::Run)),
        #[cfg(target_os = "windows")]
        ClapOperation::Build(b) => (b, Operation::Run(RunKind::Build)),
        ClapOperation::Release(b) => (b, Operation::Run(RunKind::Release)),
        ClapOperation::Test {
            adam_test,
            build_options,
        } => {
            // we need to concatenate these back into a single string...
            let mut concat = adam_test
                .into_iter()
                .fold(String::new(), |mut accum, element| {
                    accum.push_str(&element);
                    accum.push(' ');

                    accum
                });
            // and then pop off the extra space
            concat.pop();

            (build_options, Operation::Run(RunKind::Test(concat)))
        }
        ClapOperation::Check {
            path_to_run,
            build_options,
        } => {
            if let Some(path_to_run) = path_to_run {
                *check_options = Some(path_to_run);
            }
            (build_options, Operation::Check)
        }
        ClapOperation::Clean(b) => (b, Operation::Clean),

        // we won't get here for these
        ClapOperation::UserConfig(_)
        | ClapOperation::Edit(_)
        | ClapOperation::Folder { .. }
        | ClapOperation::Script(_)
        | ClapOperation::Object(_)
        | ClapOperation::Shader(_)
        | ClapOperation::Remove { .. }
        | ClapOperation::Reserialize
        | ClapOperation::Rename { .. } => {
            unimplemented!()
        }
    };

    // write them cli_options down!
    build_options.write_to_options(&mut runtime_options);

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

    let user_id: usize = match um_json.get("userID").unwrap().as_str().unwrap().parse() {
        Ok(v) => v,
        Err(_) => {
            println!(
                "{}: invalid `userID` found. are you logged in?",
                console::style("adam error").bright().red(),
            );

            std::process::exit(1);
        }
    };

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
                .join(format!("{}_{}/local_settings.json", user_name, user_id)),
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
