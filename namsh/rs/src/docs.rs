use forte_sdk::*;
use serde::{Deserialize, Serialize};

pub use doc_db::DbRequest;

#[derive(Serialize, Deserialize, Clone)]
pub struct CliTokenEntry {
    pub id: String,
    pub label: String,
    pub created_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebSessionEntry {
    pub token: String,
    pub created_at: DateTime,
}

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

#[forte_doc]
pub struct UserDoc {
    #[sk]
    pub github_id: i64,
    pub github_login: String,
    pub created_at: DateTime,
    pub cli_tokens: Vec<CliTokenEntry>,
    pub web_sessions: Vec<WebSessionEntry>,
}

#[forte_doc]
pub struct BuildDoc {
    #[sk]
    pub build_id: String,
    pub created_at: DateTime,
    pub uploaded_by: i64,
    pub hmac_key_hex: String,
    pub pdb_uploaded: bool,
    pub pdb_r2_key: Option<String>,
    pub pdb_size: Option<u64>,
}

#[forte_doc]
pub struct StackGroupDoc {
    #[sk]
    pub stack_hash: String,
    pub first_seen: DateTime,
    pub last_seen: DateTime,
    pub count: u64,
    pub dump_ids: Vec<String>,
    pub latest_context: CrashContext,
}

#[forte_doc]
pub struct DumpDoc {
    #[sk]
    pub dump_id: String,
    pub stack_hash: String,
    pub build_id: String,
    pub uploaded_at: DateTime,
    pub r2_key: String,
    pub context: CrashContext,
    pub client_ip: String,
}

#[forte_doc]
pub struct IpRateLimitDoc {
    #[sk]
    pub ip: String,
    pub recent_requests: Vec<DateTime>,
}

#[forte_doc]
pub struct CliAuthorizationCodeDoc {
    #[sk]
    pub code: String,
    pub github_id: i64,
    pub code_challenge: String,
    pub redirect_uri: String,
    pub label: String,
    pub expires_at: DateTime,
}
