use camino::Utf8PathBuf;
use clap::Parser;

use crate::{runner::CheckOptions, RunOptions, DEFAULT_PLATFORM_DATA};

/// A CLI intended for use by humans and machines to build GameMakerStudio 2 projects.
#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct InputOpts {
    #[clap(subcommand)]
    pub subcmd: Option<ClapOperation>,

    /// The path to a non-standard named configuration file. Possible names are .adam, .adam.json, and adam.toml
    #[clap(short, long, parse(from_os_str))]
    pub config: Option<std::path::PathBuf>,

    /// Prints version information
    #[clap(short, long)]
    pub version: bool,

    /// Prints the GM runtime this directory is set up to use.
    #[clap(short, long)]
    pub runtime: bool,
}

#[derive(Parser, Debug)]
pub enum ClapOperation {
    /// Runs some presumably shorter "check" script
    Check(CliCheckOptions),

    /// Builds a project *without* running it.
    Build(CliOptions),

    /// Compiles, if necessary, and then runs a project.
    Run(CliOptions),

    /// Creates a release executable, running `clean` first.
    Release(CliOptions),

    /// Runs the project, enabling any `test_env_variables` and searches for the `test_success_code`, set in the config.
    Test(CliOptions),

    /// Cleans a project target directory.
    Clean(CleanOptions),

    /// Edits the user's personal configuration file
    #[clap(subcommand)]
    UserConfig(UserConfigOptions),
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UserConfigOptions {
    /// Prints out the User Configuration file. If one does not exist, it is created.
    View,

    /// Saves a given path as a user config.
    SavePath(SavePathOptions),
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct SavePathOptions {
    pub path: Utf8PathBuf,
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default, Ord, PartialOrd)]
pub struct CliCheckOptions {
    /// This is the shell script which we will run.
    ///
    /// This path is relative to the current working directory.
    pub path_to_run: Option<Utf8PathBuf>,

    /// This is the path the shell script will be executed in. If not given, defaults to use
    /// the current working directory.
    ///
    /// This path is relative to the current working directory.
    #[clap(long, short)]
    pub directory_to_use: Option<Utf8PathBuf>,
}

impl CliCheckOptions {
    pub fn write_to_options(self, check_options: &mut CheckOptions) {
        if let Some(v) = self.path_to_run {
            check_options.path_to_run = v;
        }
        if let Some(v) = self.directory_to_use {
            check_options.directory_to_use = Some(v);
        }
    }
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default)]
pub struct CliOptions {
    /// Uses the YYC instead of the default VM. If this is the case, then we'll need to check
    /// your Visual Studio path on Windows.
    #[clap(long, short)]
    pub yyc: bool,

    /// Option to switch to using the Gms2 Beta. By default, this will use the
    /// `C:/Program Files/GameMaker Studio 2 Beta/GameMakerStudio-Beta.exe` filepath,
    /// but can be overriden with `gms2_install_location` for beta Steam builds.
    #[clap(long)]
    pub beta: bool,

    /// Whether or not to use the x64 variant on windows.
    ///
    /// On non-Windows platforms, this option is meaningless. We do a best effort to detect x64 usage by reading
    /// your options.yy, but we don't currently parse configs deeply, which means that a special config set up
    /// to use x64 won't be discovered. For such a circumstance, use this flag to build correctly.
    ///
    /// In general, it's easiest if you don't use override x64 with certain configs in Gms2.
    #[clap(long, short)]
    pub x64_windows: bool,

    /// If this option is set, then we will not read your `~/.config/GameMakerStudio2` or `%APPDATA%/GameMakerStudio2` folders
    /// at all. If you pass this, then you MUST pass in a `user-license-folder` and (on Windows) a `visual-studio-path`. Otherwise,
    /// adam will exit out with an error.
    #[clap(long)]
    pub no_user_folder: bool,

    /// Specifies a configuration. If not passed, we use `Default` for our Config.
    #[clap(short, long)]
    pub config: Option<String>,

    /// Specifies the target Yyp to build, if there are multiple.
    #[clap(long)]
    pub yyp: Option<String>,

    /// Verbosity level. Can use multiple times, like '-vv'. >0 disables pretty compiles, >1 enables igor verbosity, >2 enables gmac verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbosity: usize,

    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<Utf8PathBuf>,

    /// Ignore cache. Can use multiples times, like `-ii`. >0 disables quick recompiles, >1 disables all caching.
    #[clap(short, long, parse(from_occurrences))]
    pub ignore_cache: usize,

    /// The path to your Gms2 installation. Defaults to C drive on Windows and Applications on macOS. If you use Steam, you will need to pass in that fullpath to the .exe, or the .app on macOS.
    #[clap(long)]
    pub gms2_install_location: Option<Utf8PathBuf>,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.409` on stable and beta.
    #[clap(short, long)]
    pub runtime: Option<String>,

    /// This sets a complete path to the runtime location.
    #[clap(long)]
    pub runtime_location_override: Option<Utf8PathBuf>,

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
    #[clap(long)]
    pub visual_studio_path: Option<Utf8PathBuf>,

    /// Use this folder for the user_license, instead of the path within the `user_folder`
    /// at `~/.config`.
    ///
    /// If this field and `visual_studio_path` are both set, then we will not look in your
    /// `user_folder` at all.
    #[clap(long)]
    pub user_license_folder: Option<Utf8PathBuf>,

    /// If true, will try to find the PID of a runner game and force it to close.
    #[clap(long)]
    pub close_on_sig_kill: bool,
}

impl CliOptions {
    pub fn write_to_options(self, run_options: &mut RunOptions) {
        if let Some(cfg) = self.config {
            run_options.task.config = cfg;
        }
        if let Some(of) = self.output_folder {
            run_options.task.output_folder = of;
        }
        if let Some(gms2) = self.gms2_install_location {
            run_options.platform.gms2_application_location = gms2;
        }
        if let Some(runtime) = self.runtime {
            let path = run_options
                .platform
                .runtime_location
                .parent()
                .unwrap()
                .join(format!("runtime-{}", runtime));
            run_options.platform.runtime_location = path;
        }
        if let Some(runtime_location_override) = self.runtime_location_override {
            run_options.platform.runtime_location = runtime_location_override;
        }
        if let Some(visual_studio_path) = self.visual_studio_path {
            run_options.platform.visual_studio_path = visual_studio_path;
        }
        if let Some(user_license_folder) = self.user_license_folder {
            run_options.platform.user_license_folder = user_license_folder;
        }

        // Macos never has a visual studio path!
        // good stuff, eh?
        #[cfg(target_os = "macos")]
        {
            run_options.platform.visual_studio_path = Default::default();
        }

        if self.x64_windows {
            run_options.task.x64_windows = true;
        }
        if self.no_user_folder {
            run_options.task.no_user_folder = true;
        }

        if self.beta {
            run_options.platform.gms2_application_location =
                DEFAULT_PLATFORM_DATA.beta_application_path.into();

            run_options.platform.runtime_location =
                DEFAULT_PLATFORM_DATA.beta_runtime_location.into();

            run_options.platform.compiler_cache = DEFAULT_PLATFORM_DATA.beta_cached_data.clone();
        }
        if self.verbosity != 0 {
            run_options.task.verbosity = self.verbosity;
        }

        // if we say to use the yyc, we use the yyc
        if self.yyc {
            run_options.task.yyc = true;
        } else {
            // we just set the visual studio path here..
            run_options.platform.visual_studio_path = Default::default();
        }

        if self.ignore_cache != 0 {
            run_options.task.ignore_cache = self.ignore_cache;
        }

        if self.close_on_sig_kill {
            run_options.task.close_on_sig_kill = self.close_on_sig_kill;
        }
    }
}

#[derive(Parser, Debug)]
pub struct CleanOptions {
    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<Utf8PathBuf>,
}
