use std::path::PathBuf;

use clap::Parser;

use crate::AnyResult;

/// A CLI intended for use by humans and machines to build GameMakerStudio 2 projects.
#[derive(Parser, Debug)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!())]
pub struct InputOpts {
    #[clap(subcommand)]
    pub subcmd: ClapOperation,

    /// The path to a non-standard named configuration file. Possible names are .adam, .adam.json, and adam.toml
    #[clap(short, long)]
    pub config: Option<String>,

    /// Prints version information
    #[clap(short, long)]
    pub version: bool,
}

#[derive(Parser, Debug)]
pub enum ClapOperation {
    /// Builds a project *without* running it.
    Build(RunOptions),

    /// Compiles, if necessary, and then runs a project.
    Run(RunOptions),

    /// Creates a release executable, running `clean` first.
    Release(RunOptions),

    /// Cleans a project target directory.
    Clean(CleanOptions),
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default)]
pub struct RunOptions {
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
    pub output_folder: Option<std::path::PathBuf>,

    /// Ignore cache. Can use multiples times, like `-ii`. >0 disables quick recompiles, >1 disables all caching.
    #[clap(short, long, parse(from_occurrences))]
    pub ignore_cache: usize,

    /// The path to your Gms2 installation. Defaults to C drive on Windows and Applications on macOS. If you use Steam, you will need to pass in that fullpath to the .exe, or the .app on macOS.
    #[clap(long)]
    pub gms2_install_location: Option<PathBuf>,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.409` on stable and beta.
    #[clap(short, long)]
    pub runtime: Option<String>,

    /// This sets a complete path to the runtime location.
    #[clap(long)]
    pub runtime_location_override: Option<PathBuf>,

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
    pub visual_studio_path: Option<PathBuf>,

    /// Use this folder for the user_license, instead of the path within the `user_folder`
    /// at `~/.config`.
    ///
    /// If this field and `visual_studio_path` are both set, then we will not look in your
    /// `user_folder` at all.
    #[clap(long)]
    pub user_license_folder: Option<PathBuf>,
}

impl RunOptions {
    pub fn output_folder(&self) -> &std::path::Path {
        self.output_folder
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("target"))
    }

    pub fn canonicalize(&mut self) -> AnyResult {
        use color_eyre::Help;

        if let Some(runtime_override) = &mut self.runtime_location_override {
            *runtime_override = dunce::canonicalize(&runtime_override)
                .with_note(|| "runtime-overide is invalid")?;

            // println!("{}", runtime_override.to_str().unwrap());
        }

        if let Some(install_location) = &mut self.gms2_install_location {
            *install_location = dunce::canonicalize(&install_location)
                .with_note(|| "install-location is invalid")?;
        }

        if let Some(visual_studio_path) = &mut self.visual_studio_path {
            // we set the visual studio path to an empty path as a dummy
            // outside the --yyc option and on all macOS compiles.
            if visual_studio_path.as_os_str().is_empty() == false {
                *visual_studio_path = dunce::canonicalize(&visual_studio_path)
                    .with_note(|| "visual-studio-path is invalid")?;
            }
        }

        if let Some(user_license_folder) = &mut self.user_license_folder {
            *user_license_folder = dunce::canonicalize(&user_license_folder)
                .with_note(|| "user-license-folder is invalid")?;
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct CleanOptions {
    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<std::path::PathBuf>,
}
