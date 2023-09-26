#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cache {
    pub time: std::time::Duration,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            // 5 mins
            time: std::time::Duration::new(60 * 5, 0),
        }
    }
}
