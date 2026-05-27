//! [`CrashContext`] mirrors namsh's `docs.rs::CrashContext` definition byte-for-byte
//! so that `serde_json::to_vec` on either side yields identical bytes — which is
//! what the HMAC signature is computed over.
//!
//! Minimal by design: anything the server can derive by parsing the uploaded
//! minidump (OS, arch, CPU, exception code, etc.) lives there, not here.

use crate::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CrashContext {
    pub build_id: String,
    pub install_id: String,
    pub session_uptime_sec: u64,
    pub log_tail: Option<String>,
}

pub struct CollectArgs<'a> {
    pub config: &'a Config,
    pub install_id: &'a str,
    pub session_uptime_sec: u64,
    pub log_tail: Option<String>,
}

pub fn collect(args: CollectArgs<'_>) -> CrashContext {
    CrashContext {
        build_id: args.config.build_id.clone(),
        install_id: args.install_id.to_string(),
        session_uptime_sec: args.session_uptime_sec,
        log_tail: args.log_tail,
    }
}
