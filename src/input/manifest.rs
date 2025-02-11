use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::DEFAULT_PLATFORM_DATA;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// the Gms2 configuration to use. If blank, will use "Default".
    pub configuration: Option<String>,

    /// the verbosity to use in the compiler.
    /// >0 disable the pretty compile widget
    /// >1 adds verbose logging for the initial stages of compilation
    /// >2 enables all verbosity
    pub verbosity: Option<u8>,

    /// The output folder, relative to the current working directory. Defaults to `target`
    pub output_folder: Option<Utf8PathBuf>,

    /// Ignore cache.
    /// >0 disables the quick run when no files have been changed.
    /// >1 disables caching entirely.
    pub ignore_cache: Option<u8>,

    /// An absolute path to the Gms2 install location on the system.
    ///
    /// On Windows, this defaults to `C:\Program Files\GameMaker Studio 2\GameMaker.exe`.
    /// On macOS, this default to `/Applications/GameMaker.app`. (For macOS, you can point to just
    /// the .app -- internally, we will search inside the app bundle for the executable)
    pub gms2_install_location: Option<Utf8PathBuf>,

    /// Option to switch to using the Gms2 Beta. By default, this will use the `C:/Program Files/GameMaker Studio 2 Beta/GameMakerStudio-Beta.exe`
    /// filepath, but can be overriden with `gms2_install_location` for beta Steam builds.
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub beta: bool,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.409` on stable and beta.
    pub runtime: Option<String>,

    #[deprecated]
    #[serde(default)]
    #[serde(skip_serializing)]
    pub x64_windows: bool,

    /// If this option is set, then we will not read your `~/.config/GameMakerStudio2` or `%APPDATA%/GameMakerStudio2` folders
    /// at all. If you pass this, then you MUST pass in a `user-license-folder` and (on Windows) a `visual-studio-path`. Otherwise,
    /// adam will exit out with an error.
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_user_folder: bool,

    /// This sets a complete path to the runtime location.
    #[serde(default)]
    pub runtime_location_override: Option<Utf8PathBuf>,

    /// Use this visual studio path, instead of the visual studio path within the `user_folder`
    /// at `~/.config`. This is only relevant on Windows.
    ///
    /// This should be a path to the `.bat` file, such as:
    ///
    /// ```zsh
    /// C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Auxiliary/Build/vcvars32.bat
    /// ```
    ///
    /// For more info on this path, see https://help.yoyogames.com/hc/en-us/articles/235186048-Setting-Up-For-Windows
    ///
    /// If this field and `user_license_folder` are both set, then we will not look in your
    /// `user_folder` at all. To ensure we don't do that, pass `-no-user-folder`.
    #[serde(default)]
    pub visual_studio_path: Option<Utf8PathBuf>,

    /// Use this folder for the user_license, instead of the path within the `user_folder`
    /// at `~/.config`.
    ///
    /// If this field and `visual_studio_path` are both set, then we will not look in your
    /// `user_folder` at all.
    #[serde(default)]
    pub user_license_folder: Option<Utf8PathBuf>,

    /// A list of environment variable names that will be set to "1" if running `adam test`.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub test_env_variables: Vec<String>,

    /// A keyword in the output to look for in `adam test` which indicates that a run completed. Ideally,
    /// this should be printed right before the program completes.
    #[serde(default)]
    pub test_success_keyword: Option<String>,

    /// This is the shell script which we will run on Windows.
    ///
    /// This path is relative to the current working directory.
    #[serde(default)]
    pub path_to_run_windows: Option<Utf8PathBuf>,

    /// This is the shell script which we will run on *Nix platforms (macOS only currently).
    ///
    /// This path is relative to the current working directory.
    #[serde(default)]
    pub path_to_run_nix: Option<Utf8PathBuf>,
}

impl Manifest {
    pub fn write_to_options(
        self,
        run_options: &mut crate::RunOptions,
        check_options: &mut Option<Utf8PathBuf>,
    ) {
        #[allow(deprecated)]
        let Self {
            configuration,
            verbosity,
            output_folder,
            ignore_cache,
            gms2_install_location,
            beta,
            runtime,
            no_user_folder,
            runtime_location_override,
            visual_studio_path,
            user_license_folder,
            test_env_variables,
            test_success_keyword,
            path_to_run_windows,
            path_to_run_nix,
            x64_windows: _,
        } = self;

        if let Some(o) = configuration {
            run_options.task.config = o;
        }

        if let Some(verb) = verbosity {
            run_options.task.verbosity = verb;
        }

        if let Some(output_folder) = output_folder {
            run_options.task.output_folder = output_folder;
        }

        if let Some(gms2_install_location) = gms2_install_location {
            run_options.platform.gms2_application_location = gms2_install_location;
        }

        if let Some(ignore_cache) = ignore_cache {
            run_options.task.ignore_cache = ignore_cache;
        }

        if beta {
            run_options.platform.gms2_application_location =
                DEFAULT_PLATFORM_DATA.beta_application_path.into();

            run_options.platform.runtime_location =
                DEFAULT_PLATFORM_DATA.beta_runtime_location.into();

            run_options.platform.compiler_cache = DEFAULT_PLATFORM_DATA.beta_cached_data.clone();
        }

        if let Some(runtime) = runtime {
            let path = run_options
                .platform
                .runtime_location
                .parent()
                .unwrap()
                .join(format!("runtime-{}", runtime));
            run_options.platform.runtime_location = path;
        }

        if let Some(o) = runtime_location_override {
            run_options.platform.runtime_location = o;
        }

        if let Some(o) = visual_studio_path {
            run_options.platform.visual_studio_path = o;
        }

        if let Some(o) = user_license_folder {
            run_options.platform.user_license_folder = o;
        }
        run_options.task.no_user_folder = no_user_folder;
        run_options.task.test_env_variables = test_env_variables;
        if let Some(o) = test_success_keyword {
            run_options.task.test_success_needle = o;
        }
        let target = if cfg!(target_os = "windows") {
            path_to_run_windows
        } else {
            path_to_run_nix
        };
        if let Some(target) = target {
            *check_options = Some(target);
        }
    }

    /// Applies personal config onto another config
    pub fn apply_on(self, target_config: &mut Self) {
        #[allow(deprecated)]
        let Self {
            configuration,
            verbosity,
            output_folder,
            ignore_cache,
            gms2_install_location,
            beta,
            runtime,
            no_user_folder,
            runtime_location_override,
            visual_studio_path,
            user_license_folder,
            mut test_env_variables,
            test_success_keyword,
            path_to_run_windows,
            path_to_run_nix,
            x64_windows: _,
        } = self;

        if let Some(o) = configuration {
            target_config.configuration = Some(o);
        }

        if let Some(verb) = verbosity {
            target_config.verbosity = Some(verb);
        }

        if let Some(output_folder) = output_folder {
            target_config.output_folder = Some(output_folder);
        }

        if let Some(gms2_install_location) = gms2_install_location {
            target_config.gms2_install_location = Some(gms2_install_location);
        }

        if let Some(ignore_cache) = ignore_cache {
            target_config.ignore_cache = Some(ignore_cache);
        }

        if beta {
            target_config.beta = true;
        }

        if let Some(runtime) = runtime {
            target_config.runtime = Some(runtime);
        }

        if let Some(o) = runtime_location_override {
            target_config.runtime_location_override = Some(o);
        }

        if let Some(o) = visual_studio_path {
            target_config.visual_studio_path = Some(o);
        }

        if let Some(o) = user_license_folder {
            target_config.user_license_folder = Some(o);
        }
        if no_user_folder {
            target_config.no_user_folder = true;
        }
        target_config
            .test_env_variables
            .append(&mut test_env_variables);

        if let Some(o) = test_success_keyword {
            target_config.test_success_keyword = Some(o);
        }

        if let Some(windows_path) = path_to_run_windows {
            target_config.path_to_run_windows = Some(windows_path);
        }

        if let Some(nix_path) = path_to_run_nix {
            target_config.path_to_run_nix = Some(nix_path);
        }
    }
}

impl Manifest {
    pub fn find_manifest(user_supplied_path: Option<&PathBuf>) -> Option<Manifest> {
        let config_path = match user_supplied_path {
            Some(path) => path.to_path_buf(),
            None => {
                let current_directory =
                    std::env::current_dir().expect("cannot work in current directory");
                let mut iterator = current_directory.read_dir().ok()?.flatten();
                iterator
                    .find(|entry| {
                        entry.file_name().to_str().is_some_and(|file| {
                            matches!(
                                file,
                                ".adam" | "adam.toml" | ".adam.toml" | "adam.json" | ".adam.json"
                            )
                        })
                    })
                    .map(|entry| entry.path())?
            }
        };

        let config = std::fs::read_to_string(config_path).ok().and_then(|txt| {
            toml::from_str(&txt)
                .or_else(|_| serde_json::from_str(&txt))
                .ok()
        });

        if config.is_none() {
            println!(
                "{}: could not deserialize configuration file. Ignoring...",
                console::style("configuration error").red()
            );
        }

        config
    }
}
