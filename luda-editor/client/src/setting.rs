#[derive(Clone, serde::Deserialize)]
pub struct Setting {
    pub endpoint: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8888".to_string(),
        }
    }
}
