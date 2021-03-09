use clap::Clap;

use super::cli::{ClapOperation, RunOptions};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum Operation {
    Run(RunKind),
    Clean,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum RunKind {
    Run,
    Build,
}

pub fn parse_inputs() -> (RunOptions, Operation) {
    let mut config_file: RunOptions = super::config_file::ConfigFile::find_config()
        .unwrap_or_default()
        .into();

    let value: super::cli::InputOpts = super::cli::InputOpts::parse();
    let (b, operation) = match value.subcmd {
        ClapOperation::Run(b) => (b, Operation::Run(RunKind::Run)),
        ClapOperation::Build(b) => (b, Operation::Run(RunKind::Build)),
        ClapOperation::Clean(co) => (
            RunOptions {
                output_folder: co.output_folder,
                ..Default::default()
            },
            Operation::Clean,
        ),
    };

    let RunOptions {
        config,
        verbosity,
        yyc,
        yyp,
        output_folder,
        ignore_cache,
        gms2_install_location,
        beta,
        runtime,
        x64_windows,
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
    if let Some(gms2) = gms2_install_location {
        config_file.gms2_install_location = Some(gms2);
    }
    if let Some(runtime) = runtime {
        config_file.runtime = Some(runtime);
    }

    config_file.beta = beta;
    config_file.x64_windows = x64_windows;
    config_file.verbosity = verbosity;
    config_file.yyc = yyc;
    config_file.ignore_cache = ignore_cache;

    (config_file, operation)
}
