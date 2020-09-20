use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UserData {
    pub user_dir: PathBuf,
    pub user_string: String,
    pub visual_studio_path: Option<PathBuf>,
}

impl UserData {
    pub fn new() -> anyhow::Result<Self> {
        let user_directory = crate::gm_artifacts::user_directory();

        let um_json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&user_directory.join("um.json"))?)?;

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
            user_directory.join(&format!("{}_{}/local_settings.json", user_name, user_id)),
        )?)?;

        let supposed_path = local_settings
            .get("machine.Platform Settings.Windows.visual_studio_path")
            .map(|v| {
                let v = v.as_str().unwrap();
                Path::new(v).to_owned()
            })
            .filter(|p| p.exists());

        Ok(Self {
            user_dir: user_directory,
            user_string: format!("{}_{}", user_name, user_id),
            visual_studio_path: supposed_path,
        })
    }
}
