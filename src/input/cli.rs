use std::path::PathBuf;

use clap::Clap;

/// A CLI intended for use by humans and machines to build GameMakerStudio 2 projects.
#[derive(Clap, Debug)]
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

#[derive(Clap, Debug)]
pub enum ClapOperation {
    /// Builds a project *without* running it.
    Build(RunOptions),

    /// Compiles, if necessary, and then runs a project.
    Run(RunOptions),

    // /// Creates a release executable, running `clean` first.
    // Release(RunOptions),
    /// Cleans a project target directory.
    Clean(CleanOptions),
}

#[derive(Clap, Debug, PartialEq, Eq, Clone, Default)]
pub struct RunOptions {
    /// Uses the YYC instead of the default VM.
    #[clap(long, short)]
    pub yyc: bool,

    /// Option to switch to using the Gms2 Beta. By default, this will use the
    /// `C:/Program Files/GameMaker Studio 2 Beta/GameMakerStudio-Beta.exe` filepath,
    /// but can be overriden with `gms2_install_location` for beta Steam builds.
    #[clap(long, short)]
    pub beta: bool,

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
    #[clap(short, long)]
    pub gms2_install_location: Option<PathBuf>,

    /// If the non-current runtime is desired, it can be set here. We default right now to `2.3.1.536` on stable and beta.
    #[clap(short, long)]
    pub runtime: Option<String>,
}

#[derive(Clap, Debug)]
pub struct CleanOptions {
    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<std::path::PathBuf>,
}
