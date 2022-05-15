use std::path::PathBuf;
use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::DEFAULT_PLATFORM_DATA;

#[derive(Debug, PartialEq, Eq, Deserialize, Default)]
pub struct ConfigFile {
    /// the Gms2 configuration to use. If blank, will use "Default".
    pub configuration: Option<String>,

    /// the verbosity to use in the compiler.
    /// >0 disable the pretty compile widget
    /// >1 adds verbose logging for the initial stages of compilation
    /// >2 enables all verbosity
    pub verbosity: Option<usize>,

    /// The output folder, relative to the current working directory. Defaults to `target`
    pub output_folder: Option<Utf8PathBuf>,

    /// Ignore cache.
    /// >0 disables the quick run when no files have been changed.
    /// >1 disables caching entirely.
    pub ignore_cache: Option<usize>,

    /// An absolute path to the Gms2 install location on the system.
    ///
    /// On Windows, this defaults to `C:\Program Files\GameMaker Studio 2\GameMaker.exe`.
    /// On macOS, this default to `/Applications/GameMaker.app`. (For macOS, you can point to just
    /// the .app -- internally, we will search inside the app bundle for the executable)
    pub gms2_install_location: Option<Utf8PathBuf>,

    /// Option to switch to using the Gms2 Beta. By default, this will use the `C:/Program Files/GameMaker Studio 2 Beta/GameMakerStudio-Beta.exe`
    /// filepath, but can be overriden with `gms2_install_location` for beta Steam builds.
    #[serde(default)]
    pub beta: bool,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.409` on stable and beta.
    pub runtime: Option<String>,

    /// Whether or not to use the x64 variant on windows.
    /// On non-Windows platforms, this option is meaningless. We do a best effort to detect x64 usage by reading
    /// your options.yy, but we don't currently parse configs deeply, which means that a special config set up
    /// to use x64 won't be discovered. For such a circumstance, use this flag to build correctly.
    ///
    /// In general, it's easiest if you don't use override x64 with certain configs in Gms2.
    #[serde(default)]
    pub x64_windows: bool,

    /// If this option is set, then we will not read your `~/.config/GameMakerStudio2` or `%APPDATA%/GameMakerStudio2` folders
    /// at all. If you pass this, then you MUST pass in a `user-license-folder` and (on Windows) a `visual-studio-path`. Otherwise,
    /// adam will exit out with an error.
    #[serde(default)]
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
    pub test_env_variables: Vec<String>,

    /// A keyword in the output to look for in `adam test` which indicates that a run completed. Ideally,
    /// this should be printed right before the program completes.
    #[serde(default)]
    pub test_success_keyword: Option<String>,
}

impl ConfigFile {
    pub fn write_to_options(self, run_options: &mut crate::RunOptions) {
        let Self {
            configuration,
            verbosity,
            output_folder,
            ignore_cache,
            gms2_install_location,
            beta,
            runtime,
            x64_windows,
            no_user_folder,
            runtime_location_override,
            visual_studio_path,
            user_license_folder,
            test_env_variables,
            test_success_keyword: test_success_code,
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

        run_options.task.x64_windows = x64_windows;

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
        if let Some(o) = test_success_code {
            run_options.task.test_success_needle = o;
        }
    }
}

impl ConfigFile {
    pub fn find_config(user_supplied_path: Option<&PathBuf>) -> Option<ConfigFile> {
        let config_path = match user_supplied_path {
            Some(path) => path.to_path_buf(),
            None => {
                let current_directory =
                    std::env::current_dir().expect("cannot work in current directory");
                let mut iterator = current_directory.read_dir().ok()?.flatten();
                iterator
                    .find(|entry| {
                        entry.file_name().to_str().map_or(false, |file| {
                            matches!(
                                file,
                                ".adam" | "adam.toml" | ".adam.toml" | "adam.json" | ".adam.json"
                            )
                        })
                    })
                    .map(|entry| entry.path())?
            }
        };

        let config = std::fs::read_to_string(&config_path).ok().and_then(|txt| {
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
