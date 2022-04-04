use camino::Utf8PathBuf;
use once_cell::sync::Lazy;

#[cfg(target_os = "windows")]
pub const PLATFORM_KIND: crate::igor::PlatformKind = crate::igor::PlatformKind::Windows;

#[cfg(not(target_os = "windows"))]
pub const PLATFORM_KIND: crate::igor::PlatformKind = crate::igor::PlatformKind::Darwin;

#[derive(Debug)]
pub struct DefaultPlatformData {
    pub stable_runtime_location: &'static str,
    pub beta_runtime_location: &'static str,
    pub stable_application_path: &'static str,
    pub beta_application_path: &'static str,

    pub target_mask: usize,
    pub home_dir: Lazy<Utf8PathBuf>,
    pub stable_cached_data: Lazy<Utf8PathBuf>,
    pub beta_cached_data: Lazy<Utf8PathBuf>,
}
pub const DEFAULT_RUNTIME_NAME: &str = "2022.3.0.496";

#[cfg(not(target_os = "windows"))]
pub static DEFAULT_PLATFORM_DATA: DefaultPlatformData = {
    DefaultPlatformData {
        stable_runtime_location: const_format::concatcp!(
            "/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-",
            DEFAULT_RUNTIME_NAME
        ),
        beta_runtime_location: const_format::concatcp!(
            "/Users/Shared/GameMakerStudio2-Beta/Cache/runtimes/runtime-",
            DEFAULT_RUNTIME_NAME
        ),
        stable_application_path: "/Applications/GameMaker.app/Contents/MonoBundle",
        beta_application_path: "/Applications/GameMaker-Beta.app/Contents/MonoBundle",

        target_mask: 2,
        stable_cached_data: Lazy::new(|| home_dir().join(".config/GameMakerStudio2")),
        beta_cached_data: Lazy::new(|| home_dir().join(".config/GameMakerStudio2-Beta")),
        home_dir: Lazy::new(home_dir),
    }
};

#[cfg(target_os = "windows")]
pub static DEFAULT_PLATFORM_DATA: DefaultPlatformData = {
    DefaultPlatformData {
        stable_runtime_location: const_format::concatcp!(
            "C:/ProgramData/GameMakerStudio2/Cache/runtimes/runtime-",
            DEFAULT_RUNTIME_NAME
        ),
        beta_runtime_location: const_format::concatcp!(
            "C:/ProgramData/GameMakerStudio2-Beta/Cache/runtimes/runtime-",
            DEFAULT_RUNTIME_NAME
        ),
        stable_application_path: "C:/Program Files/GameMaker/GameMaker.exe",
        beta_application_path: "C:/Program Files/GameMaker-Beta/GameMaker-Beta.exe",

        target_mask: 64,
        stable_cached_data: Lazy::new(|| home_dir().join("AppData/Roaming/GameMakerStudio2")),
        beta_cached_data: Lazy::new(|| home_dir().join("AppData/Roaming/GameMakerStudio2-Beta")),
        home_dir: Lazy::new(home_dir),
    }
};

fn home_dir() -> Utf8PathBuf {
    directories::UserDirs::new()
        .unwrap()
        .home_dir()
        .to_owned()
        .try_into()
        .unwrap()
}

#[cfg(target_os = "macos")]
pub const MONO_LOCATION: &str = "/Library/Frameworks/Mono.framework/Versions/Current/Commands/mono";
