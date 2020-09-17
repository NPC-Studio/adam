#[cfg(target_os = "macos")]
mod macos {
    pub const RUNTIME_LOCATION: &str =
        "/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401";

    pub const TARGET_MASK: usize = 2;

    pub const APPLICATION_PATH: &str =
        "/Applications/GameMaker Studio 2.app/Contents/MonoBundle/GameMaker Studio 2.exe";

    pub const RUNNER: &str = "/Library/Frameworks/Mono.framework/Versions/Current/Commands/mono";

    pub const IGOR: &str =
        "/Users/Shared/GameMakerStudio2/Cache/runtimes/runtime-2.3.0.401/bin/Igor.exe";
}
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
mod windows {
    pub const RUNTIME_LOCATION: &str = "C:/ProgramData/GameMakerStudio2/runtimes/runtime-2.3.0.401";

    pub const TARGET_MASK: usize = 64;

    pub const APPLICATION_PATH: &str = "C:/Program Files/GameMaker Studio 2/GameMakerStudio.exe";

    pub const RUNNER: &str = "cmd";

    pub const IGOR: &str =
        "C:/ProgramData/GameMakerStudio2/runtimes/runtime-2.3.0.401/bin/Igor.exe";
}
#[cfg(target_os = "windows")]
pub use windows::*;
