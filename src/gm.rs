use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BuildData {
    pub output_folder: PathBuf,
    pub output_kind: OutputKind,
    pub project_name: String,
    pub current_directory: PathBuf,
    pub user_dir: PathBuf,
    pub user_name: String,
    pub user_id: usize,
    pub runtime_location: PathBuf,
    pub target_mask: usize,
    pub application_path: PathBuf,
}

impl BuildData {
    #[allow(dead_code)]
    pub fn output_build_bff(self) -> serde_json::Value {
        let json_str = format!(
            r#"{{
    "targetFile": "",
    "assetCompiler": "",
    "debug": "False",
    "compile_output_file_name": "{OUTPUT_FOLDER}/{OUTPUT_KIND}/{PROJECT_NAME}.win",
    "useShaders": "True",
    "steamOptions": "{CACHE}/steam_options.yy",
    "config": "Default",
    "outputFolder": "{OUTPUT_FOLDER}/{OUTPUT_KIND}",
    "projectName": "{PROJECT_NAME}",
    "macros": "{CACHE}/macros.json",
    "projectDir": "{CURRENT_DIRECTORY}",
    "preferences": "{CACHE}/preferences.json",
    "projectPath": "{CURRENT_DIRECTORY}/{PROJECT_NAME}.yyp",
    "tempFolder": "{CACHE}",
    "tempFolderUnmapped": "{CACHE}",
    "userDir": "{USER_DIR}/{USER_NAME}_{USER_ID}",
    "runtimeLocation": "{RUNTIME_LOCATION}",
    "targetOptions": "{CACHE}/targetoptions.json",
    "targetMask": "{TARGET_MASK}",
    "applicationPath": "{APPLICATION_PATH}",
    "verbose": "False",
    "SteamIDE": "False",
    "helpPort": "51290",
    "debuggerPort": "6509"
}}"#,
            OUTPUT_FOLDER = self.output_folder.display(),
            OUTPUT_KIND = self.output_kind,
            PROJECT_NAME = self.project_name,
            CACHE = format!(
                "{}/{}/cache",
                self.output_folder.display(),
                self.output_kind
            ),
            CURRENT_DIRECTORY = self.current_directory.display(),
            USER_DIR = self.user_dir.display(),
            USER_NAME = self.user_name,
            USER_ID = self.user_id,
            RUNTIME_LOCATION = self.runtime_location.display(),
            TARGET_MASK = self.target_mask,
            APPLICATION_PATH = self.application_path.display(),
        );

        serde_json::from_str(&json_str).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OutputKind {
    Vm,
    Yyc,
}

impl Default for OutputKind {
    fn default() -> Self {
        Self::Vm
    }
}

impl std::fmt::Display for OutputKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputKind::Vm => write!(f, "vm"),
            OutputKind::Yyc => write!(f, "yyc"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    Windows,
    Darwin,
}
