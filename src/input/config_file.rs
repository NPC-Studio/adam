use serde::Deserialize;

use super::cli::RunOptions;

#[derive(Debug, PartialEq, Eq, Deserialize, Default)]
pub struct ConfigFile {
    /// The configuration to use. If blank, will use "Default".
    pub configuration: Option<String>,

    /// the target yyp. If blank, will use a Yyp if found in the directory.
    pub yyp: Option<String>,

    /// the verbosity to use in the compiler.
    /// >1 disable the pretty compile widget
    /// >2 adds verbose logging for the initial stages of compilation
    /// >3 enables all verbosity
    pub verbosity: Option<usize>,

    /// The output folder, relative to the current working directory. Defaults to `target`
    pub output_folder: Option<std::path::PathBuf>,
}

impl From<ConfigFile> for RunOptions {
    fn from(o: ConfigFile) -> Self {
        Self {
            yyc: false,
            config: o.configuration,
            yyp: o.yyp,
            verbosity: o.verbosity.unwrap_or_default(),
            output_folder: o.output_folder,
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
