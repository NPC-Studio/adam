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

    /// A list of environment variable names that will be set to "1" if running `adam test`.
    pub test_env_variables: Vec<String>,

    /// This is the code in a test case that we search for.
    pub test_success_needle: String,

    /// If true, will try to find the PID of a runner game and force it to close.
    pub close_on_sig_kill: bool,
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
    pub fn canonicalize(&mut self) -> Result<(), CanonicalizationErr> {
        self.runtime_location = dunce::canonicalize(&self.runtime_location)
            .map_err(|_| CanonicalizationErr::Runtime)?
            .try_into()
            .expect("failed to convert runtime-location path to utf8");

        self.gms2_application_location = dunce::canonicalize(&self.gms2_application_location)
            .map_err(|_| CanonicalizationErr::Installation)?
            .try_into()
            .expect("failed to convert installation path to utf8");

        if self.user_license_folder.as_str().is_empty() == false {
            self.user_license_folder = dunce::canonicalize(&self.user_license_folder)
                .map_err(|_| CanonicalizationErr::UserLicense)?
                .try_into()
                .expect("failed to convert user-license path to utf8");
        }

        Ok(())
    }

    pub fn canonicalize_yyc(&mut self) -> AnyResult {
        use color_eyre::Help;

        if self.visual_studio_path.as_str().is_empty() == false {
            self.visual_studio_path = dunce::canonicalize(&self.visual_studio_path)
                .with_note(|| {
                    format!(
                        "visual_studio_path ({:?}) is invalid",
                        self.visual_studio_path
                    )
                })?
                .try_into()?;
        }

        Ok(())
    }
}

impl Default for TaskOptions {
    fn default() -> Self {
        TaskOptions {
            yyc: false,
            no_user_folder: false,
            config: "Default".to_string(),
            verbosity: 0,
            output_folder: "target".into(),
            ignore_cache: 0,
            test_env_variables: vec![],
            test_success_needle: "RUN_SUCCESS".to_string(),
            close_on_sig_kill: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CanonicalizationErr {
    Runtime,
    Installation,
    UserLicense,
}

impl std::fmt::Display for CanonicalizationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            CanonicalizationErr::Runtime => "runtime",
            CanonicalizationErr::Installation => "installation",
            CanonicalizationErr::UserLicense => "user license",
        };

        f.pad(w)
    }
}
