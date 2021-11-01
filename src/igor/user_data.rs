use anyhow::Context;
use std::path::{Path, PathBuf};

use crate::gm_artifacts::Platform;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UserData {
    // pub user_dir: PathBuf,
    pub user_string: String,
    pub visual_studio_path: PathBuf,
}

impl UserData {
    pub fn new(platform: &Platform) -> anyhow::Result<Self> {
        let gms2_data = platform.gms2_data.clone();
        let um_json: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&gms2_data.join("um.json"))
                .with_context(|| format!("could not find {}", gms2_data.join("um.json").display()))?,
        )?;

        let user_id: usize = um_json.get("userID").unwrap().as_str().unwrap().parse()?;

        let user_name = um_json
            .get("username")
            .unwrap()
            .as_str()
            .unwrap()
            .split('@')
            .next()
            .unwrap()
            .to_owned();

        let local_settings: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(
            gms2_data.join(&format!("{}_{}/local_settings.json", user_name, user_id)),
        )?)?;

        let supposed_path = local_settings
            .get("machine.Platform Settings.Windows.visual_studio_path")
            .map(|v| {
                let v = v.as_str().unwrap();
                Path::new(v).to_owned()
            })
            .unwrap_or_else(|| {
                Path::new("C:/Program Files (x86)/Microsoft Visual Studio 14.0/VC/bin/vcvars32.bat")
                    .to_owned()
            });

        Ok(Self {
            user_string: format!("{}_{}", user_name, user_id),
            visual_studio_path: supposed_path,
        })
    }
}
