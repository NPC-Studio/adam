mod run;
pub use run::run_command;

mod check_options;
pub use check_options::run_check;

#[cfg(not(target_os = "windows"))]
mod invoke_nix;
#[cfg(not(target_os = "windows"))]
pub(super) use invoke_nix::{invoke_release, invoke_run};

#[cfg(target_os = "windows")]
mod invoke_win;
#[cfg(target_os = "windows")]
pub(super) use invoke_win::{invoke_release, invoke_run};

mod compiler_handler;
mod printer;
mod run_options;

pub use run_options::*;

mod cache;
pub use cache::Cache;
