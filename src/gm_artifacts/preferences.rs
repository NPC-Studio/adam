use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Default, Clone, Hash)]
pub struct GmPreferences {
    default_team_id: String,
    suppress_build: bool,
}
