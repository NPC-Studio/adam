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
}

#[derive(Clap, Debug)]
pub struct CleanOptions {
    /// The relative path to the output folder. Defaults to `target`.
    #[clap(short, long)]
    pub output_folder: Option<std::path::PathBuf>,
}
