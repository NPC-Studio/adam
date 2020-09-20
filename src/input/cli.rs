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
    /// Compiles and then runs a project.
    Run(RunOptions),

    /// Compiles a project without running it.
    Build(RunOptions),

    /// Creates a release executable, running `clean` first.
    Release(RunOptions),

    /// Cleans a project target directory.
    Clean,
}

#[derive(Clap, Debug)]
pub struct RunOptions {
    /// Uses the YYC instead of the default VM.
    #[clap(long, short)]
    pub yyc: bool,

    /// Specifies a configuration. If not passed, we use `Default` for our Config.
    #[clap(short, long)]
    pub config: Option<String>,

    /// Specifies a the target Yyp to build
    #[clap(short, long)]
    pub target: Option<String>,

    /// Verbosity level. Can use multiple times. >0 disables pretty compiles, >1 enables igor verbosity, >2 enables gmac verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub verbosity: usize,
}
