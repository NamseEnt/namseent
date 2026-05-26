//! [`CrashContext`] mirrors namsh's `docs.rs::CrashContext` definition byte-for-byte
//! so that `serde_json::to_vec` on either side yields identical bytes — which is
//! what the HMAC signature is computed over.

use crate::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CrashContext {
    pub app_version: String,
    pub build_id: String,
    pub os: String,
    pub os_version: String,
    pub arch: String,
    pub cpu: Option<String>,
    pub gpu_adapter: Option<String>,
    pub gpu_driver: Option<String>,
    pub locale: Option<String>,
    pub install_id: String,
    pub session_uptime_sec: u64,
    pub error_message: Option<String>,
    pub log_tail: Option<String>,
}

pub fn collect(config: &Config, install_id: &str, session_uptime_sec: u64) -> CrashContext {
    let info = os_info::get();
    CrashContext {
        app_version: config.build_id.clone(),
        build_id: config.build_id.clone(),
        os: info.os_type().to_string(),
        os_version: info.version().to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu: None,
        gpu_adapter: None,
        gpu_driver: None,
        locale: None,
        install_id: install_id.to_string(),
        session_uptime_sec,
        error_message: None,
        log_tail: None,
    }
}
