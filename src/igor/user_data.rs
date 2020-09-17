use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UserData {
    pub user_dir: PathBuf,
    pub user_string: String,
}

impl UserData {
    pub fn new() -> Self {
        let user_directory = crate::gm_artifacts::user_directory();

        let um_json: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&user_directory.join("um.json")).unwrap(),
        )
        .unwrap();

        let user_id: usize = um_json
            .get("userID")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();

        let user_name = um_json
            .get("username")
            .unwrap()
            .as_str()
            .unwrap()
            .split('@')
            .next()
            .unwrap()
            .to_owned();

        Self {
            user_dir: user_directory,
            user_string: format!("{}_{}", user_name, user_id),
        }
    }
}
