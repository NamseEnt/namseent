use crate::cli::Target;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NamuiUserConfig {
    pub cfg_map: NamuiCfgMap,
    pub target: Target,
}
impl Default for NamuiUserConfig {
    fn default() -> Self {
        Self {
            cfg_map: HashMap::new(),
            target: Target::WasmUnknownWeb,
        }
    }
}

pub type NamuiCfgMap = HashMap<String, String>;
