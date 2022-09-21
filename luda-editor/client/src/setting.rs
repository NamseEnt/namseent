#[derive(Clone, serde::Deserialize)]
pub struct Setting {
    pub rpc_endpoint: String,
    pub resource_base_url: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            rpc_endpoint: "http://localhost:8888".to_string(),
            resource_base_url: "http://localhost:9000/one-for-all/".to_string(),
        }
    }
}
