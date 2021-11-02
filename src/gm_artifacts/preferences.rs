use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub struct GmPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_packaging_choice: Option<usize>,
    #[cfg(target_os = "windows")]
    visual_studio_path: PathBuf,
    #[cfg(target_os = "macos")]
    default_team_id: String,
    #[cfg(target_os = "macos")]
    suppress_build: bool,
}

impl GmPreferences {
    #[cfg(target_os = "windows")]
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
            #[cfg(target_os = "windows")]
            visual_studio_path: Path::new(
                "C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/vcvars32.bat",
            )
            .to_owned(),
            #[cfg(target_os = "macos")]
            default_team_id: String::new(),
            #[cfg(target_os = "macos")]
            suppress_build: false,
        }
    }
}
