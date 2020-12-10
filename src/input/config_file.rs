use serde::Deserialize;

use super::cli::RunOptions;

#[derive(Debug, PartialEq, Eq, Deserialize, Default)]
pub struct ConfigFile {
    /// the Gms2 configuration to use. If blank, will use "Default".
    pub configuration: Option<String>,

    /// the target yyp project name. If blank, will use a Yyp if found in the directory.
    pub yyp: Option<String>,

    /// the verbosity to use in the compiler.
    /// >0 disable the pretty compile widget
    /// >1 adds verbose logging for the initial stages of compilation
    /// >2 enables all verbosity
    pub verbosity: Option<usize>,

    /// The output folder, relative to the current working directory. Defaults to `target`
    pub output_folder: Option<std::path::PathBuf>,

    /// Ignore cache.
    /// >0 disables the quick run when no files have been changed.
    /// >1 disables caching entirely.
    pub ignore_cache: Option<usize>,

    /// An absolute path to the Gms2 install location on the system.
    ///
    /// On Windows, this defaults to `C:\Program Files\GameMaker Studio 2\GameMakerStudio.exe`.
    /// On macOS, this default to `/Applications/GameMaker Studio 2.app`. (For macOS, you can point to just
    /// the .app -- internally, we will search inside the app bundle for the executable)
    pub gms2_install_location: Option<std::path::PathBuf>,

    /// Option to switch to using the Gms2 Beta. By default, this will use the `C:/Program Files/GameMaker Studio 2 Beta/GameMakerStudio-Beta.exe`
    /// filepath, but can be overriden with `gms2_install_location` for beta Steam builds.
    #[serde(default)]
    pub beta: bool,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.536` on stable and beta.
    pub runtime: Option<String>,
}

impl From<ConfigFile> for RunOptions {
    fn from(o: ConfigFile) -> Self {
        Self {
            yyc: false,
            config: o.configuration,
            yyp: o.yyp,
            verbosity: o.verbosity.unwrap_or_default(),
            output_folder: o.output_folder,
            gms2_install_location: o.gms2_install_location,
            ignore_cache: o.ignore_cache.unwrap_or_default(),
            beta: o.beta,
            runtime: o.runtime,
        }
    }
}

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
