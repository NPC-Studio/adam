use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BuildData {
    pub output_folder: PathBuf,
    pub output_kind: OutputKind,
    pub project_filename: String,
    pub project_directory: PathBuf,
    /// This is the home directory for the user. Ie, on Mac Os, this will be `~`.
    pub user_dir: PathBuf,
    /// This is the folder of the license directory.
    pub license_folder: PathBuf,
    pub runtime_location: PathBuf,
    pub target_mask: usize,
    pub application_path: PathBuf,
    pub config: String,
    pub target_file: Option<PathBuf>,
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
pub enum PlatformKind {
    Windows,
    Darwin,
}

impl std::fmt::Display for PlatformKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformKind::Windows => write!(f, "windows"),
            PlatformKind::Darwin => write!(f, "mac"),
        }
    }
}
