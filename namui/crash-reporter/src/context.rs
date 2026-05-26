//! [`CrashContext`] mirrors namsh's `docs.rs::CrashContext` definition byte-for-byte
//! so that `serde_json::to_vec` on either side yields identical bytes — which is
//! what the HMAC signature is computed over.

use crate::Config;
use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

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

pub struct CollectArgs<'a> {
    pub config: &'a Config,
    pub install_id: &'a str,
    pub session_uptime_sec: u64,
    pub error_message: Option<String>,
    pub gpu_adapter: Option<String>,
    pub gpu_driver: Option<String>,
    pub log_tail: Option<String>,
}

pub fn collect(args: CollectArgs<'_>) -> CrashContext {
    let info = os_info::get();
    let system =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    let cpu = system.cpus().first().map(|c| {
        let brand = c.brand().trim();
        if brand.is_empty() {
            c.name().to_string()
        } else {
            brand.to_string()
        }
    });
    let locale = sys_locale::get_locale();
    CrashContext {
        app_version: args.config.build_id.clone(),
        build_id: args.config.build_id.clone(),
        os: info.os_type().to_string(),
        os_version: info.version().to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu,
        gpu_adapter: args.gpu_adapter,
        gpu_driver: args.gpu_driver,
        locale,
        install_id: args.install_id.to_string(),
        session_uptime_sec: args.session_uptime_sec,
        error_message: args.error_message,
        log_tail: args.log_tail,
    }
}
