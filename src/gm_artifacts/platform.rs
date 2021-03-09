use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
pub const PLATFORM_KIND: crate::igor::PlatformKind = crate::igor::PlatformKind::Windows;

#[cfg(not(target_os = "windows"))]
pub const PLATFORM_KIND: crate::igor::PlatformKind = crate::igor::PlatformKind::Darwin;

#[derive(Debug, Clone)]
pub struct Platform {
    pub runtime_location: PathBuf,
    pub target_mask: usize,
    pub application_path: PathBuf,
    pub gms2_data: PathBuf,
    pub user_data: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PlatformBuilder {
    runtime: String,
    short_name: String,
    long_name: String,
    appdata_name: String,
    app_override: Option<PathBuf>,
}

impl PlatformBuilder {
    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        Self {
            runtime: "2.3.1.409".to_string(),
            short_name: "GameMakerStudio".to_string(),
            long_name: "GameMaker Studio 2".to_string(),
            appdata_name: "GameMakerStudio2".to_string(),
            app_override: None,
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new(&self) -> Self {
        // Self {
        //     runtime: "2.3.1.536".to_string(),
        //     platform: Platform::Windows,
        //     short_name: "GameMakerStudio".to_string(),
        //     long_name: "GameMaker Studio 2".to_string(),
        // }
    }

    pub fn set_runtime_name(&mut self, runtime: String) {
        self.runtime = runtime;
    }

    pub fn set_beta(&mut self) {
        self.long_name = "GameMaker Studio 2-Beta".to_string();
        self.short_name = "GameMakerStudio-Beta".to_string();
        self.appdata_name = "GameMakerStudio2-Beta".to_string();
    }

    pub fn set_app_override(&mut self, app_override: Option<PathBuf>) {
        self.app_override = app_override;
    }

    #[cfg(target_os = "windows")]
    pub fn generate(self) -> Platform {
        Platform {
            runtime_location: Path::new(&format!(
                "C:/ProgramData/{}/Cache/runtimes/runtime-{}/",
                self.appdata_name, self.runtime
            ))
            .to_owned(),
            target_mask: 64,
            application_path: match self.app_override {
                Some(v) => v,
                None => Path::new(&format!(
                    "C:/Program Files/{}/{}.exe",
                    self.long_name, self.short_name
                ))
                .to_owned(),
            },
            gms2_data: directories::UserDirs::new()
                .unwrap()
                .home_dir()
                .join(format!("AppData/Roaming/{}", self.appdata_name)),
            user_data: directories::UserDirs::new().unwrap().home_dir().to_owned(),
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    pub const RUNTIME_LOCATION: &str =
        "/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401";
    pub const TARGET_MASK: usize = 2;
    pub const APPLICATION_PATH: &str = "/Applications/GameMaker Studio 2.app";
    pub const MONO_LOCATION: &str =
        "/Library/Frameworks/Mono.framework/Versions/Current/Commands/mono";
    pub const PLATFORM: super::Platform = super::Platform::Darwin;

    pub fn gms2_data() -> std::path::PathBuf {
        directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .join(".config/GameMakerStudio2")
    }

    pub fn user_data() -> std::path::PathBuf {
        directories::UserDirs::new().unwrap().home_dir().to_owned()
    }
}
#[cfg(target_os = "macos")]
pub use macos::*;

// #[cfg(target_os = "windows")]
// mod windows {
//     pub const RUNTIME_LOCATION: &str =
//         "C:\\ProgramData\\GameMakerStudio2\\Cache\\runtimes\\runtime-2.3.0.401";
//     pub const TARGET_MASK: usize = 64;
//     pub const APPLICATION_PATH: &str = "C:/Program Files/GameMaker Studio 2/GameMakerStudio.exe";

//     pub const PLATFORM: super::PlatformKind = super::PlatformKind::Windows;

//     pub fn gms2_data() -> std::path::PathBuf {
//         directories::UserDirs::new()
//             .unwrap()
//             .home_dir()
//             .join("AppData/Roaming/GameMakerStudio2")
//     }

//     pub fn user_data() -> std::path::PathBuf {
//         directories::UserDirs::new().unwrap().home_dir().to_owned()
//     }
// }
// #[cfg(target_os = "windows")]
// pub use windows::*;
