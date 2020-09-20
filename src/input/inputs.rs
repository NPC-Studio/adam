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
    pub visual_studio_path: Option<std::path::PathBuf>,
}

impl RunData {
    pub fn new(b: RunOptions) -> Self {
        let RunOptions {
            config,
            verbosity,
            visual_studio_path,
            yyc,
            yyp,
        } = b;

        Self {
            yyc,
            config: config.unwrap_or_else(|| "Default".to_string()),
            yyp_name: yyp,
            verbosity,
            visual_studio_path,
        }
    }
}

pub fn get_input() -> Input {
    let mut config_file: RunOptions = super::config_file::ConfigFile::find_config()
        .unwrap_or_default()
        .into();
    let mut value: super::cli::InputOpts = super::cli::InputOpts::parse();

    match &mut value.subcmd {
        ClapOperation::Run(b) | ClapOperation::Release(b) => {
            let RunOptions {
                config,
                verbosity,
                yyc,
                yyp,
                visual_studio_path,
            } = b;

            if let Some(cfg) = config.take() {
                config_file.config = Some(cfg);
            }
            if let Some(cfg) = yyp.take() {
                config_file.yyp = Some(cfg);
            }
            if let Some(vsp) = visual_studio_path.take() {
                config_file.visual_studio_path = Some(vsp);
            }
            config_file.verbosity = *verbosity;
            config_file.yyc = *yyc;

            *b = config_file;
        }
        ClapOperation::Clean => {}
    }

    match value.subcmd {
        ClapOperation::Run(b) => Input::Run(RunData::new(b)),
        ClapOperation::Release(b) => Input::Release(RunData::new(b)),
        ClapOperation::Clean => Input::Clean,
    }
}
