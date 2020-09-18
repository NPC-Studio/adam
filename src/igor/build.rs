use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BuildData {
    pub output_folder: PathBuf,
    pub output_kind: OutputKind,
    pub project_name: String,
    pub project_directory: PathBuf,
    pub user_dir: PathBuf,
    pub user_string: String,
    pub runtime_location: PathBuf,
    pub target_mask: usize,
    pub application_path: PathBuf,
    pub config: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OutputKind {
    #[serde(rename = "VM")]
    Vm,
    #[allow(dead_code)]
    #[serde(rename = "YYC")]
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

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    Windows,
    Darwin,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Windows => write!(f, "windows"),
            Platform::Darwin => write!(f, "mac"),
        }
    }
}
