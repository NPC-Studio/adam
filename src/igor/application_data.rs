use std::env;

use camino::Utf8PathBuf;

use crate::AnyResult;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ApplicationData {
    pub current_directory: Utf8PathBuf,
    pub project_name: String,
}

impl ApplicationData {
    pub fn new() -> AnyResult<Self> {
        let current_directory = Utf8PathBuf::from_path_buf(
            env::current_dir().map_err(|_| color_eyre::eyre::anyhow!("cannot read directory"))?,
        )
        .map_err(|_| color_eyre::eyre::anyhow!("current dir isn't utf8"))?;

        let mut project_name = None;

        for file in current_directory.read_dir()?.flatten() {
            let file = file.path();

            if let Some(ext) = file.extension() {
                if ext == "yyp" {
                    let stem = file.file_stem().unwrap().to_string_lossy().to_string();
                    if project_name.is_some() {
                        color_eyre::eyre::bail!("multiple yyps discovered",);
                    }

                    project_name = Some(stem);
                }
            }
        }

        if let Some(project_name) = project_name {
            Ok(Self {
                current_directory,
                project_name,
            })
        } else {
            color_eyre::eyre::bail!("could not find a .yyp in current directory")
        }
    }
}
