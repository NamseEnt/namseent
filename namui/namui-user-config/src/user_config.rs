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
            target: if cfg!(target_os = "linux") {
                Target::X86_64UnknownLinuxGnu
            } else if cfg!(target_os = "macos") {
                Target::Aarch64AppleDarwin
            } else if cfg!(target_os = "windows") {
                Target::X86_64PcWindowsMsvc
            } else {
                panic!("{} is unsupported os", std::env::consts::OS)
            },
        }
    }
}

pub type NamuiCfgMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Target {
    Wasm32WasiWeb,
    X86_64PcWindowsMsvc,
    X86_64UnknownLinuxGnu,
    Aarch64AppleDarwin,
}
impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
