use camino::Utf8PathBuf;

use crate::AnyResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RunOptions {
    pub task: TaskOptions,
    pub platform: PlatformOptions,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TaskOptions {
    /// Uses the YYC instead of the default VM. If this is the case, then we'll need to check
    /// your Visual Studio path on Windows.
    pub yyc: bool,

    /// Whether or not to use the x64 variant on windows.
    ///
    /// On non-Windows platforms, this option is meaningless. We do a best effort to detect x64 usage by reading
    /// your options.yy, but we don't currently parse configs deeply, which means that a special config set up
    /// to use x64 won't be discovered. For such a circumstance, use this flag to build correctly.
    ///
    /// In general, it's easiest if you don't use override x64 with certain configs in Gms2.
    pub x64_windows: bool,

    /// If this option is set, then we will not read your `~/.config/GameMakerStudio2` or `%APPDATA%/GameMakerStudio2` folders
    /// at all. If you pass this, then you MUST pass in a `user-license-folder` and (on Windows) a `visual-studio-path`. Otherwise,
    /// adam will exit out with an error.
    pub no_user_folder: bool,

    /// Specifies a configuration. If not passed, we use `Default` for our Config.
    pub config: String,

    /// Verbosity level. Can use multiple times, like '-vv'. >0 disables pretty compiles, >1 enables igor verbosity, >2 enables gmac verbosity
    pub verbosity: usize,

    /// The relative path to the output folder. Defaults to `target`.
    pub output_folder: Utf8PathBuf,

    /// Ignore cache. Can use multiples times, like `-ii`. >0 disables quick recompiles, >1 disables all caching.
    pub ignore_cache: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PlatformOptions {
    /// The path to your Gms2 installation. Defaults to C drive on Windows and Applications on macOS. If you use Steam, you will need to pass in that fullpath to the .exe, or the .app on macOS.
    pub gms2_application_location: Utf8PathBuf,

    /// This sets a complete path to the runtime location.
    pub runtime_location: Utf8PathBuf,

    /// Use this visual studio path, instead of the visual studio path within the `user_folder`
    /// at `~/.config`. This is only relevant on Windows.
    ///
    /// This should be a path to the `.bat` file, such as:
    ///
    /// ```zsh
    /// C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Auxiliary/Build/vcvars32.bat
    /// ```
    ///
    /// For more info on this path, see https://help.yoyogames.com/hc/en-us/articles/235186048-Setting-Up-For-Windows
    ///
    /// If this field and `user_license_folder` are both set, then we will not look in your
    /// `user_folder` at all. To ensure we don't do that, pass `-no-user-folder`.
    pub visual_studio_path: Utf8PathBuf,

    /// Use this folder for the user_license, instead of the path within the `user_folder`
    /// at `~/.config`.
    ///
    /// If this field and `visual_studio_path` are both set, then we will not look in your
    /// `user_folder` at all.
    pub user_license_folder: Utf8PathBuf,

    pub home_dir: Utf8PathBuf,
    pub compiler_cache: Utf8PathBuf,
}

impl PlatformOptions {
    pub fn canonicalize(&mut self) -> AnyResult {
        use color_eyre::Help;

        self.runtime_location = dunce::canonicalize(&self.runtime_location)
            .with_note(|| "runtime-overide is invalid")?
            .try_into()?;

        self.gms2_application_location = dunce::canonicalize(&self.gms2_application_location)
            .with_note(|| "install-location is invalid")?
            .try_into()?;

        if self.visual_studio_path.as_str().is_empty() == false {
            self.visual_studio_path = dunce::canonicalize(&self.visual_studio_path)
                .with_note(|| "visual_studio_path is invalid")?
                .try_into()?;
        }

        if self.user_license_folder.as_str().is_empty() == false {
            self.user_license_folder = dunce::canonicalize(&self.user_license_folder)
                .with_note(|| "user-license-folder is invalid")?
                .try_into()?;
        }

        Ok(())
    }
}

impl Default for TaskOptions {
    fn default() -> Self {
        TaskOptions {
            yyc: false,
            x64_windows: true,
            no_user_folder: false,
            config: "Default".to_string(),
            verbosity: 0,
            output_folder: "target".into(),
            ignore_cache: 0,
        }
    }
}
