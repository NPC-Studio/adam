use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    /// The configuration to use. If blank, will use "Default".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,

    /// the target yyp. If blank, will use a Yyp if found in the directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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
