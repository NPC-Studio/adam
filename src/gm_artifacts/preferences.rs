use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// #[cfg(target_os = "windows")]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub struct GmPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_packaging_choice: Option<usize>,
    visual_studio_path: PathBuf,
}

impl GmPreferences {
    pub fn new(visual_studio_path: PathBuf) -> Self {
        Self {
            default_packaging_choice: Some(1),
            visual_studio_path,
        }
    }
}

impl Default for GmPreferences {
    fn default() -> Self {
        Self {
            default_packaging_choice: Some(1),
            visual_studio_path: Path::new(
                "C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/vcvars32.bat",
            )
            .to_owned(),
        }
    }
}
