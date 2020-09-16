use clap::Clap;

/// A CLI intended for use by humans and machines to build GameMakerStudio 2 projects.
#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!())]
pub struct InputOpts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// builds a project without running it
    Build(Build),

    /// builds and runs a project
    Run(Build),
}

/// A subcommand for controlling testing
#[derive(Clap, Debug)]
pub struct Build;

