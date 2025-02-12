use crate::igor::BuildData;
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmBuild {
    pub target_file: Utf8PathBuf,
    pub asset_compiler: String,
    pub debug: String,
    #[serde(rename = "compile_output_file_name")]
    pub compile_output_file_name: Utf8PathBuf,
    pub use_shaders: String,
    pub steam_options: Utf8PathBuf,
    pub config: String,
    pub output_folder: Utf8PathBuf,
    pub project_name: String,
    pub macros: Utf8PathBuf,
    pub project_dir: Utf8PathBuf,
    pub preferences: Utf8PathBuf,
    pub project_path: Utf8PathBuf,
    pub temp_folder: Utf8PathBuf,
    pub temp_folder_unmapped: Utf8PathBuf,
    #[serde(rename = "userDir")]
    pub license_dir: Utf8PathBuf,
    pub runtime_location: Utf8PathBuf,
    pub target_options: Utf8PathBuf,
    pub target_mask: String,
    pub application_path: Utf8PathBuf,
    pub verbose: String,
    #[serde(rename = "SteamIDE")]
    pub steam_ide: String,
    pub help_port: String,
    pub debugger_port: String,
}

impl GmBuild {
    pub fn new(build_data: &BuildData) -> Self {
        let cache = &build_data.folders.cache;
        let tmp = &build_data.folders.tmp;

        Self {
            target_file: build_data.folders.target_file.clone(),
            compile_output_file_name: build_data
                .folders
                .output
                .join(&build_data.project_filename)
                .with_extension("win"),
            steam_options: build_data.folders.cache.join("steam_options.yy"),
            project_name: build_data.project_filename.clone(),
            macros: cache.join("macros.json"),
            project_dir: build_data.project_directory.clone(),
            preferences: cache.join("preferences.json"),
            project_path: build_data
                .project_directory
                .join(&build_data.project_filename)
                .with_extension("yyp"),
            temp_folder: tmp.clone(),
            temp_folder_unmapped: tmp.clone(),
            license_dir: build_data.license_folder.clone(),
            runtime_location: build_data.runtime_location.clone(),
            target_options: cache.join("targetoptions.json"),
            target_mask: build_data.target_mask.to_string(),
            application_path: build_data.application_path.clone(),
            output_folder: build_data.folders.output.clone(),
            config: build_data.config.clone(),

            ..Default::default()
        }
    }
}

impl Default for GmBuild {
    fn default() -> Self {
        Self {
            compile_output_file_name: Utf8PathBuf::new(),
            steam_options: Utf8PathBuf::new(),
            project_name: String::new(),
            macros: Utf8PathBuf::new(),
            project_dir: Utf8PathBuf::new(),
            preferences: Utf8PathBuf::new(),
            project_path: Utf8PathBuf::new(),
            temp_folder: Utf8PathBuf::new(),
            temp_folder_unmapped: Utf8PathBuf::new(),
            license_dir: Utf8PathBuf::new(),
            runtime_location: Utf8PathBuf::new(),
            target_options: Utf8PathBuf::new(),
            target_mask: String::new(),
            application_path: Utf8PathBuf::new(),
            output_folder: Utf8PathBuf::new(),

            target_file: Utf8PathBuf::new(),
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
