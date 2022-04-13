use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NamuiUserConfig {
    pub cfg_map: NamuiCfgMap,
}
impl Default for NamuiUserConfig {
    fn default() -> Self {
        Self {
            cfg_map: HashMap::new(),
        }
    }
}

pub type NamuiCfgMap = HashMap<String, String>;
