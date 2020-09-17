use crate::igor::BuildData;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmBuild {
    pub target_file: String,
    pub asset_compiler: String,
    pub debug: String,
    #[serde(rename = "compile_output_file_name")]
    pub compile_output_file_name: PathBuf,
    pub user_shaders: String,
    pub steam_options: PathBuf,
    pub config: String,
    pub output_folder: PathBuf,
    pub project_name: String,
    pub macros: PathBuf,
    pub project_dir: PathBuf,
    pub preferences: PathBuf,
    pub project_path: PathBuf,
    pub temp_folder: PathBuf,
    pub temp_folder_unmapped: PathBuf,
    pub user_dir: PathBuf,
    pub runtime_location: PathBuf,
    pub target_options: PathBuf,
    pub target_mask: String,
    pub application_path: PathBuf,
    pub verbose: String,
    #[serde(rename = "SteamIDE")]
    pub steam_ide: String,
    pub help_port: String,
    pub debugger_port: String,
}

impl GmBuild {
    pub fn new(build_data: &BuildData) -> Self {
        let build = build_data
            .output_folder
            .join(build_data.output_kind.to_string());
        let cache = build.join("cache");

        Self {
            compile_output_file_name: build_data
                .output_folder
                .join(build_data.output_kind.to_string())
                .join(&build_data.project_name)
                .with_extension("win"),
            steam_options: cache.join("steam_options.yy"),
            project_name: build_data.project_name.clone(),
            macros: cache.join("macros.json"),
            project_dir: build_data.project_directory.clone(),
            preferences: cache.join("preferences.json"),
            project_path: build_data
                .project_directory
                .join(&build_data.project_name)
                .with_extension("yyp"),
            temp_folder: cache.clone(),
            temp_folder_unmapped: cache.clone(),
            user_dir: build_data.user_dir.join(&build_data.user_string),
            runtime_location: build_data.runtime_location.clone(),
            target_options: cache.join("targetoptions.json"),
            target_mask: build_data.target_mask.to_string(),
            application_path: build_data.application_path.clone(),
            output_folder: build,

            ..Default::default()
        }
    }
}

impl Default for GmBuild {
    fn default() -> Self {
        Self {
            compile_output_file_name: PathBuf::new(),
            steam_options: PathBuf::new(),
            project_name: String::new(),
            macros: PathBuf::new(),
            project_dir: PathBuf::new(),
            preferences: PathBuf::new(),
            project_path: PathBuf::new(),
            temp_folder: PathBuf::new(),
            temp_folder_unmapped: PathBuf::new(),
            user_dir: PathBuf::new(),
            runtime_location: PathBuf::new(),
            target_options: PathBuf::new(),
            target_mask: String::new(),
            application_path: PathBuf::new(),
            output_folder: PathBuf::new(),

            target_file: String::new(),
            asset_compiler: String::new(),
            debug: "False".to_string(),
            user_shaders: "True".to_string(),
            config: "Default".to_string(),
            verbose: "False".to_string(),
            steam_ide: "False".to_string(),
            help_port: "51290".to_string(),
            debugger_port: "6509".to_string(),
        }
    }
}

/*
    pub fn output_build_bff(&self) -> serde_json::Value {
        let json_str = format!(
            r#"{{
    "compile_output_file_name": "{OUTPUT_FOLDER}/{OUTPUT_KIND}/{PROJECT_NAME}.win",
    "steamOptions": "{CACHE}/steam_options.yy",
    "outputFolder": "{OUTPUT_FOLDER}/{OUTPUT_KIND}",
    "projectName": "{PROJECT_NAME}",
    "macros": "{CACHE}/macros.json",
    "projectDir": "{CURRENT_DIRECTORY}",
    "preferences": "{CACHE}/preferences.json",
    "projectPath": "{CURRENT_DIRECTORY}/{PROJECT_NAME}.yyp",
    "tempFolder": "{CACHE}",
    "tempFolderUnmapped": "{CACHE}",
    "userDir": "{USER_DIR}/{USER_STRING}",
    "runtimeLocation": "{RUNTIME_LOCATION}",
    "targetOptions": "{CACHE}/targetoptions.json",
    "targetMask": "{TARGET_MASK}",
    "applicationPath": "{APPLICATION_PATH}",
}}"#,
            OUTPUT_FOLDER = self.output_folder.display(),
            OUTPUT_KIND = self.output_kind,
            PROJECT_NAME = self.project_name,
            CACHE = format!(
                "{}/{}/cache",
                self.output_folder.display(),
                self.output_kind
            ),
            CURRENT_DIRECTORY = self.project_directory.display(),
            USER_DIR = self.user_dir.display(),
            USER_STRING = self.user_string,
            RUNTIME_LOCATION = self.runtime_location.display(),
            TARGET_MASK = self.target_mask,
            APPLICATION_PATH = self.application_path.display(),
        );

        serde_json::from_str(&json_str).unwrap()
    }
*/
