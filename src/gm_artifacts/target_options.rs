use crate::igor::OutputKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Default, Clone, Hash)]
pub struct GmTargetOptions {
    pub runtime: OutputKind,
}
