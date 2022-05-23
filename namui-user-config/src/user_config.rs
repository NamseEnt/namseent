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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Target {
    WasmUnknownWeb,
    WasmWindowsElectron,
    WasmLinuxElectron,
}
impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
