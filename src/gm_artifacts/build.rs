use crate::igor::BuildData;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::Platform;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmBuild {
    pub target_file: PathBuf,
    pub asset_compiler: String,
    pub debug: String,
    #[serde(rename = "compile_output_file_name")]
    pub compile_output_file_name: PathBuf,
    pub use_shaders: String,
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
    pub fn new(build_data: &BuildData, platform: &Platform) -> Self {
        let build = build_data
            .output_folder
            .join(build_data.output_kind.to_string());
        let cache = build.join("cache");
        let tmp = build.join("tmp");

        Self {
            target_file: build_data
                .target_file
                .clone()
                .map(|v| build.parent().unwrap().join(v))
                .unwrap_or_default(),
            compile_output_file_name: build_data
                .output_folder
                .join(build_data.output_kind.to_string())
                .join(&build_data.project_filename)
                .with_extension("win"),
            steam_options: cache.join("steam_options.yy"),
            project_name: build_data.project_filename.clone(),
            macros: cache.join("macros.json"),
            project_dir: build_data.project_directory.clone(),
            preferences: cache.join("preferences.json"),
            project_path: build_data
                .project_directory
                .join(&build_data.project_filename)
                .with_extension("yyp"),
            temp_folder: tmp.clone(),
            temp_folder_unmapped: tmp,
            user_dir: platform.gms2_data.join(&build_data.user_string),
            runtime_location: build_data.runtime_location.clone(),
            target_options: cache.join("targetoptions.json"),
            target_mask: build_data.target_mask.to_string(),
            application_path: build_data.application_path.clone(),
            output_folder: build,
            config: build_data.config.clone(),

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

            target_file: PathBuf::new(),
            asset_compiler: String::new(),
            debug: "False".to_string(),
            use_shaders: "True".to_string(),
            config: "Default".to_string(),
            verbose: "False".to_string(),
            steam_ide: "False".to_string(),
            help_port: "51290".to_string(),
            debugger_port: "6509".to_string(),
        }
    }
}
