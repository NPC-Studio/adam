use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Default, Clone, Hash)]
pub struct GmSteamOptions {
    steamsdk_path: std::path::PathBuf,
}
