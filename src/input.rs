mod cli;
mod get_input;
mod manifest;

pub use cli::*;
pub use get_input::{parse_inputs, Operation, RunKind};
pub use manifest::Manifest;
