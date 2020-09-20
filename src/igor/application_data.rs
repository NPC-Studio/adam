use std::{env, path::PathBuf};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ApplicationData {
    pub current_directory: PathBuf,
    pub project_name: String,
}

impl ApplicationData {
    pub fn new(destination_yyp: &Option<String>) -> anyhow::Result<Self> {
        let current_directory =
            env::current_dir().map_err(|_| anyhow::anyhow!("cannot read directory"))?;

        let mut project_name = None;
        let mut too_many_projects = vec![];

        for file in current_directory.read_dir()? {
            if let Ok(file) = file {
                let file = file.path();

                if let Some(ext) = file.extension() {
                    if ext == "yyp" {
                        let st = std::fs::read_to_string(file).unwrap();

                        let yyp: yy_typings::Yyp = serde_json::from_str(
                            &yy_typings::utils::TrailingCommaUtility::clear_trailing_comma_once(
                                &st,
                            ),
                        )?;

                        if too_many_projects.is_empty() {
                            if let Some(dest) = destination_yyp {
                                if yyp.name == *dest {
                                    project_name = Some(yyp.name);
                                }
                            } else if let Some(project_name) = project_name.take() {
                                too_many_projects.push(project_name);
                                too_many_projects.push(yyp.name.clone());
                            } else {
                                project_name = Some(yyp.name);
                            }
                        } else {
                            too_many_projects.push(yyp.name.clone());
                        }
                    }
                }
            }
        }

        if too_many_projects.is_empty() == false {
            anyhow::bail!(
                "multiple yyps discovered. specify target with --target. options: \n\
            \t{}",
                too_many_projects
                    .into_iter()
                    .fold(String::new(), |mut accum, v| {
                        if accum.is_empty() == false {
                            accum.push(',');
                            accum.push(' ');
                        }
                        accum.push_str(&v);

                        accum
                    })
            );
        }

        if let Some(project_name) = project_name {
            Ok(Self {
                current_directory,
                project_name,
            })
        } else {
            anyhow::bail!(
                "could not find a valid yyp in {}",
                current_directory.display()
            )
        }
    }
}
