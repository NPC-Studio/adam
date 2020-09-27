use super::cli::{ClapOperation, RunOptions};
use clap::Clap;

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    /// Compiles and then runs a project.
    Run(RunData),

    // Builds the project without running it
    Build(RunData),

    // /// Creates a release executable, running `clean` first.
    // Release(RunData),
    /// Cleans a project target directory.
    Clean(std::path::PathBuf, RunOptions),
}

impl Input {
    pub fn yyp_name(&self) -> &Option<String> {
        match self {
            Input::Run(b) | Input::Build(b) => &b.yyp_name,
            Input::Clean(_, o) => &o.yyp,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct RunData {
    pub yyc: bool,
    pub config: String,
    pub yyp_name: Option<String>,
    pub verbosity: usize,
    pub output_folder: std::path::PathBuf,
    pub ignore_cache: usize,
}

impl RunData {
    pub fn new(b: RunOptions) -> Self {
        let RunOptions {
            config,
            verbosity,
            yyc,
            yyp,
            output_folder,
            ignore_cache,
        } = b;

        Self {
            yyc,
            config: config.unwrap_or_else(|| "Default".to_string()),
            yyp_name: yyp,
            verbosity,
            output_folder: output_folder
                .unwrap_or_else(|| std::path::Path::new("target").to_owned()),
            ignore_cache,
        }
    }
}

pub fn get_input() -> Input {
    let mut config_file: RunOptions = super::config_file::ConfigFile::find_config()
        .unwrap_or_default()
        .into();
    let mut value: super::cli::InputOpts = super::cli::InputOpts::parse();

    let b = match &mut value.subcmd {
        ClapOperation::Run(b) | ClapOperation::Build(b) => b.clone(),
        ClapOperation::Clean(_) => Default::default(),
    };

    let RunOptions {
        config,
        verbosity,
        yyc,
        yyp,
        output_folder,
        ignore_cache,
    } = b;

    if let Some(cfg) = config {
        config_file.config = Some(cfg);
    }
    if let Some(cfg) = yyp {
        config_file.yyp = Some(cfg);
    }
    if let Some(of) = output_folder {
        config_file.output_folder = Some(of);
    }
    config_file.verbosity = verbosity;
    config_file.yyc = yyc;
    config_file.ignore_cache = ignore_cache;

    match value.subcmd {
        ClapOperation::Run(_) => Input::Run(RunData::new(config_file)),
        ClapOperation::Build(_) => Input::Build(RunData::new(config_file)),
        ClapOperation::Clean(v) => {
            if let Some(passed_in) = v.output_folder {
                config_file.output_folder = Some(passed_in);
            }

            Input::Clean(
                config_file
                    .output_folder
                    .clone()
                    .unwrap_or_else(|| "target".into()),
                config_file,
            )
        }
    }
}