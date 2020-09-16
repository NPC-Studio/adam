use std::{env, path::PathBuf};

const OUTPUT_DIR: &str = "target";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ApplicationData {
    pub current_directory: PathBuf,
    pub project_name: String,
    pub output_folder: PathBuf,
}

impl ApplicationData {
    pub fn new() -> Self {
        let current_directory = env::current_dir().expect("cannot work in current directory");

        let mut project_name = None;

        for file in current_directory.read_dir().unwrap() {
            let file = file.unwrap().path();

            if let Some(ext) = file.extension() {
                if ext == "yyp" {
                    if project_name.is_some() {
                        panic!("specify project name");
                    } else {
                        let st = std::fs::read_to_string(file).unwrap();
                        let yyp: yy_typings::Yyp = serde_json::from_str(
                            &yy_typings::utils::TrailingCommaUtility::clear_trailing_comma_once(
                                &st,
                            ),
                        )
                        .expect("yyp wasn't a real yyp");
                        project_name = Some(yyp.name);
                    }
                }
            }
        }

        let project_name = project_name.unwrap();

        Self {
            output_folder: current_directory.join(OUTPUT_DIR),
            current_directory,
            project_name,
        }
    }
}
