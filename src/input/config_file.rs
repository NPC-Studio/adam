use camino::Utf8PathBuf;
use serde::Deserialize;

// use super::cli::RunOptions;

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
    /// On Windows, this defaults to `C:\Program Files\GameMaker Studio 2\GameMakerStudio.exe`.
    /// On macOS, this default to `/Applications/GameMaker Studio 2.app`. (For macOS, you can point to just
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
    pub test_env_variables: Option<Vec<String>>,
}

impl ConfigFile {
    pub fn write_to_options(self, run_options: &mut crate::RunOptions) {
        if let Some(o) = self.configuration {
            run_options.task.config = o;
        }

        if let Some(verb) = self.verbosity {
            run_options.task.verbosity = verb;
        }

        if let Some(output_folder) = self.output_folder {
            run_options.task.output_folder = output_folder;
        }

        if let Some(gms2_install_location) = self.gms2_install_location {
            run_options.platform.gms2_application_location = gms2_install_location;
        }

        if let Some(ignore_cache) = self.ignore_cache {
            run_options.task.ignore_cache = ignore_cache;
        }

        if self.beta {
            run_options.platform.gms2_application_location =
                crate::DEFAULT_PLATFORM_DATA.beta_application_path.into();

            run_options.platform.runtime_location =
                crate::DEFAULT_PLATFORM_DATA.beta_runtime_location.into();

            run_options.platform.compiler_cache = crate::BETA_CACHED_DATA.clone();
        }

        if let Some(runtime) = self.runtime {
            let path = run_options
                .platform
                .runtime_location
                .parent()
                .unwrap()
                .join(format!("runtime-{}", runtime));
            run_options.platform.runtime_location = path;
        }

        run_options.task.x64_windows = self.x64_windows;

        if let Some(o) = self.runtime_location_override {
            run_options.platform.runtime_location = o;
        }

        if let Some(o) = self.visual_studio_path {
            run_options.platform.visual_studio_path = o;
        }

        if let Some(o) = self.user_license_folder {
            run_options.platform.user_license_folder = o;
        }
        run_options.task.no_user_folder = self.no_user_folder;

        if let Some(test_env_variables) = self.test_env_variables {
            run_options.task.test_env_variables = test_env_variables;
        }
    }
}

// impl From<ConfigFile> for crate::RunOptions {
//     fn from(o: ConfigFile) -> Self {
//         Self {
//             yyc: false,
//             config: o.configuration.unwrap_or_else(|| "Default".to_string()),
//             verbosity: o.verbosity.unwrap_or_default(),
//             output_folder: o
//                 .output_folder
//                 .unwrap_or_else(|| camino::Utf8Path::new("target").to_owned()),
//             gms2_install_location: o
//                 .gms2_install_location
//                 .unwrap_or_else(|| crate::PlatformBuilder::generate()),
//             ignore_cache: o.ignore_cache.unwrap_or_default(),
//             beta: o.beta,
//             runtime: o.runtime,
//             x64_windows: o.x64_windows,
//             runtime_location_override: o.runtime_location_override,
//             visual_studio_path: o.visual_studio_path,
//             user_license_folder: o.user_license_folder,
//             no_user_folder: o.no_user_folder,
//         }
//     }
// }

impl ConfigFile {
    pub fn find_config() -> Option<ConfigFile> {
        let current_directory = std::env::current_dir().expect("cannot work in current directory");
        let iterator = current_directory.read_dir().ok()?;

        let mut config = None;

        for file in iterator {
            let file = file.ok()?.path();

            if let Some(fname) = file.file_name() {
                let lossy = fname.to_string_lossy();

                if lossy == ".adam" {
                    if let Ok(txt) = std::fs::read_to_string(&file) {
                        let mut output = toml::from_str(&txt).ok();
                        if output.is_none() {
                            output = serde_json::from_str(&txt).ok();
                        }

                        if let Some(output) = output {
                            if config.is_some() {
                                println!(
                                    "{}: two config files present. \
                                please specify with `--config`. Ignoring both...",
                                    console::style("configuration error").red()
                                );
                            } else {
                                config = Some(output);
                            }
                        } else {
                            println!(
                                "{}: could not deserialize configuration file. Ignoring...",
                                console::style("configuration error").red()
                            );
                        }
                    }
                }
                if lossy == "adam.json" || lossy == ".adam.json" {
                    if let Ok(txt) = std::fs::read_to_string(&file) {
                        match serde_json::from_str(&txt) {
                            Ok(v) => {
                                if config.is_some() {
                                    println!(
                                        "{}: two config files present. \
                                        please specify with `--config`. Ignoring both...",
                                        console::style("configuration error").red()
                                    );
                                } else {
                                    config = Some(v);
                                }
                            }
                            Err(v) => {
                                println!(
                                    "{}: could not deserialize configuration file, {}. Ignoring...",
                                    console::style("configuration error").red(),
                                    v
                                );
                            }
                        }
                    }
                }
                if lossy == "adam.toml" || lossy == ".adam.toml" {
                    if let Ok(txt) = std::fs::read_to_string(&file) {
                        match toml::from_str(&txt) {
                            Ok(v) => {
                                if config.is_some() {
                                    println!(
                                        "{}: two config files present. \
                                        please specify with `--config`. Ignoring both...",
                                        console::style("configuration error").red()
                                    );
                                } else {
                                    config = Some(v);
                                }
                            }
                            Err(v) => {
                                println!(
                                    "{}: could not deserialize configuration file, {}. Ignoring...",
                                    console::style("configuration error").red(),
                                    v
                                );
                            }
                        }
                    }
                }
            }
        }

        config
    }
}
