use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::AnyResult;

/// This is our core truthy data format. All data comes
/// directly from this build data in some form or another.
#[derive(Debug, Clone)]
pub struct BuildData {
    /// The target folders for this build.
    pub folders: TargetFolders,

    /// This is the kind out output.
    pub output_kind: OutputKind,

    /// The name of the project.
    pub project_filename: String,
    pub project_directory: Utf8PathBuf,

    /// This is the home directory for the user. Ie, on Mac Os, this will be `~`.
    pub user_dir: Utf8PathBuf,

    /// This is the folder of the license directory.
    pub license_folder: Utf8PathBuf,

    /// This is the runtime location folder
    pub runtime_location: Utf8PathBuf,

    /// The target mask is nearly meaningless, but it appears to be `64`.
    pub target_mask: usize,

    /// The path to the application exe. On macOS, this is to the `.MacOS` folder
    /// within the application.
    pub application_path: Utf8PathBuf,

    /// The config to pass to the EXE. By default, this is `Default`.
    pub config: String,
}

#[derive(Debug, Clone)]
pub struct TargetFolders {
    /// This is the parent folder, such as `PROJECT/target/vm`.
    pub main: Utf8PathBuf,
    /// This is the folder we dump the important stuff inside.
    pub output: Utf8PathBuf,
    /// This is the cache folder.
    pub cache: Utf8PathBuf,
    /// This is the temp folder.
    pub tmp: Utf8PathBuf,
    /// This is the end output of a zip in a build. It is meaningless otherwise.
    pub target_file: Utf8PathBuf,
}

impl TargetFolders {
    pub fn new(
        current_directory: &Utf8Path,
        output_folder: &Utf8Path,
        output_kind: OutputKind,
        project_name: &str,
    ) -> AnyResult<Self> {
        let dir = current_directory
            .join(output_folder)
            .join(output_kind.to_string());

        let me = TargetFolders {
            output: dir.join("output"),
            cache: dir.join("cache"),
            tmp: dir.join("tmp"),
            target_file: dir.join(project_name).with_extension("zip"),
            main: dir,
        };

        // generate all our folders...
        std::fs::create_dir_all(&me.output)?;
        std::fs::create_dir_all(&me.cache)?;
        std::fs::create_dir_all(&me.tmp)?;

        Ok(me)
    }

    pub fn clear_tmp(&self) -> AnyResult {
        // remove it!
        if self.tmp.exists() {
            std::fs::remove_dir_all(&self.tmp).unwrap();
        }
        std::fs::create_dir_all(&self.tmp).unwrap();

        Ok(())
    }
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
