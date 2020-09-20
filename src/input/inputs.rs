use super::cli::{ClapOperation, RunOptions};
use clap::Clap;

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    /// Compiles and then runs a project.
    Run(RunData),

    /// Creates a release executable, running `clean` first.
    Release(RunData),

    /// Cleans a project target directory.
    Clean,
}

impl Input {
    pub fn yyp_name(&self) -> &Option<String> {
        match self {
            Input::Run(b) | Input::Release(b) => &b.yyp_name,
            Input::Clean => &None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RunData {
    pub yyc: bool,
    pub config: String,
    pub yyp_name: Option<String>,
    pub verbosity: usize,
}

impl RunData {
    pub fn new(b: RunOptions) -> Self {
        Self {
            yyc: b.yyc,
            config: b.config.unwrap_or_else(|| "Default".to_string()),
            yyp_name: b.target,
            verbosity: b.verbosity,
        }
    }
}

pub fn get_input() -> Input {
    let mut config_file = super::config_file::ConfigFile::find_config().unwrap_or_default();
    let value: super::cli::InputOpts = super::cli::InputOpts::parse();
    if let Some(cfg) = value.config {
        config_file.configuration = Some(cfg);
    }

    match value.subcmd {
        ClapOperation::Run(b) => Input::Run(RunData::new(b)),
        ClapOperation::Release(b) => Input::Release(RunData::new(b)),
        ClapOperation::Clean => Input::Clean,
    }
}
