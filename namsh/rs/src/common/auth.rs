use crate::docs::*;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use forte_sdk::*;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub const SESSION_COOKIE: &str = "namsh_session";
pub const OAUTH_STATE_COOKIE: &str = "namsh_oauth_state";
pub const PENDING_CLI_CONSENT_COOKIE: &str = "namsh_pending_cli_consent";
const SESSION_MAX_AGE_DAYS: i64 = 30;
const OAUTH_STATE_MAX_AGE_SECS: i64 = 600;
const PENDING_CLI_CONSENT_MAX_AGE_SECS: i64 = 600;
const PENDING_CLI_CONSENT_PATH_PREFIX: &str = "/oauth/cli/authorize";
const CLI_TOKEN_PREFIX: &str = "namsh_";
const CLI_TOKEN_PAYLOAD_LEN: usize = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionCookie {
    pub github_id: i64,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
struct OauthState {
    nonce: String,
    timestamp: i64,
}

pub async fn current_user(jar: &CookieJar) -> Option<UserDoc> {
    let cookie: SessionCookie = cookie_sign::unsign_cookie(jar, SESSION_COOKIE)?;
    let db = doc_db::turso();
    let user = (UserDocGet {
        github_id: cookie.github_id,
    })
    .send_with(&db)
    .await
    .ok()??;
    if user.web_sessions.iter().any(|e| e.token == cookie.token) {
        Some(user)
    } else {
        None
    }
}

pub async fn create_session(jar: &mut CookieJar, mut user: UserDoc) -> anyhow::Result<()> {
    let bytes = rand::get_random_bytes(32).await;
    let token = hex::encode(&bytes);
    let github_id = user.github_id;
    user.web_sessions.push(WebSessionEntry {
        token: token.clone(),
        created_at: forte_sdk::now(),
    });
    let db = doc_db::turso();
    UserDocPut(user).send_with(&db).await?;
    cookie_sign::sign_cookie(
        jar,
        SESSION_COOKIE,
        &SessionCookie { github_id, token },
        Some(time::Duration::days(SESSION_MAX_AGE_DAYS)),
    );
    Ok(())
}

pub async fn clear_session(jar: &mut CookieJar) -> anyhow::Result<()> {
    let cookie: Option<SessionCookie> = cookie_sign::unsign_cookie(jar, SESSION_COOKIE);
    clear_session_cookie(jar);
    let Some(cookie) = cookie else { return Ok(()) };
    let db = doc_db::turso();
    if let Some(mut user) = (UserDocGet {
        github_id: cookie.github_id,
    })
    .send_with(&db)
    .await?
    {
        let before = user.web_sessions.len();
        user.web_sessions.retain(|e| e.token != cookie.token);
        if user.web_sessions.len() != before {
            UserDocPut(user).send_with(&db).await?;
        }
    }
    Ok(())
}

pub fn clear_session_cookie(jar: &mut CookieJar) {
    jar.remove(cookie::CookieBuilder::new(SESSION_COOKIE, "").path("/"));
}

pub async fn prepare_oauth(jar: &mut CookieJar) -> String {
    let bytes = rand::get_random_bytes(32).await;
    let nonce = hex::encode(&bytes);
    let state = serde_json::to_string(&OauthState {
        nonce: nonce.clone(),
        timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
    })
    .expect("serialize oauth state");
    jar.add(
        cookie::CookieBuilder::new(OAUTH_STATE_COOKIE, state)
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Lax)
            .path("/")
            .max_age(time::Duration::seconds(OAUTH_STATE_MAX_AGE_SECS)),
    );
    nonce
}

pub fn verify_oauth_state(jar: &mut CookieJar, state_from_url: &str) -> bool {
    let Some(c) = jar.get(OAUTH_STATE_COOKIE) else {
        return false;
    };
    let Ok(state) = serde_json::from_str::<OauthState>(c.value()) else {
        return false;
    };
    if state.nonce != state_from_url {
        return false;
    }
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    if now - state.timestamp > OAUTH_STATE_MAX_AGE_SECS {
        return false;
    }
    jar.remove(cookie::CookieBuilder::new(OAUTH_STATE_COOKIE, "").path("/"));
    true
}

pub fn mint_cli_token(github_id: i64, token_id: &Uuid) -> anyhow::Result<String> {
    let key = std::env::var("NAMSH_TOKEN_HMAC_KEY")
        .map_err(|_| anyhow::anyhow!("NAMSH_TOKEN_HMAC_KEY not set"))?;

    let mut payload = Vec::with_capacity(CLI_TOKEN_PAYLOAD_LEN);
    payload.extend_from_slice(&github_id.to_be_bytes());
    payload.extend_from_slice(token_id.as_bytes());

    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|e| anyhow::anyhow!("hmac key: {e}"))?;
    mac.update(&payload);
    let signature = mac.finalize().into_bytes();

    let payload_b64 = URL_SAFE_NO_PAD.encode(&payload);
    let sig_b64 = URL_SAFE_NO_PAD.encode(signature);
    Ok(format!("{CLI_TOKEN_PREFIX}{payload_b64}.{sig_b64}"))
}

pub async fn verify_cli_token(token: &str) -> Option<UserDoc> {
    let stripped = token.strip_prefix(CLI_TOKEN_PREFIX)?;
    let (payload_b64, sig_b64) = stripped.split_once('.')?;
    let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
    let sig = URL_SAFE_NO_PAD.decode(sig_b64).ok()?;
    if payload.len() != CLI_TOKEN_PAYLOAD_LEN {
        return None;
    }

    let key = std::env::var("NAMSH_TOKEN_HMAC_KEY").ok()?;
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).ok()?;
    mac.update(&payload);
    if mac.verify_slice(&sig).is_err() {
        return None;
    }

    let github_id = i64::from_be_bytes(payload[..8].try_into().ok()?);
    let token_uuid_bytes: [u8; 16] = payload[8..].try_into().ok()?;
    let token_id = Uuid::from_bytes(token_uuid_bytes).to_string();

    let db = doc_db::turso();
    let user = (UserDocGet { github_id }).send_with(&db).await.ok()??;
    if user.cli_tokens.iter().any(|t| t.id == token_id) {
        Some(user)
    } else {
        None
    }
}

pub async fn bearer_user(headers: &::http::HeaderMap) -> Option<UserDoc> {
    let raw = headers.get(http_header::AUTHORIZATION)?.to_str().ok()?;
    let token = raw.strip_prefix("Bearer ")?.trim();
    verify_cli_token(token).await
}

pub async fn session_or_bearer_user(
    jar: &CookieJar,
    headers: &::http::HeaderMap,
) -> Option<UserDoc> {
    if let Some(u) = current_user(jar).await {
        return Some(u);
    }
    bearer_user(headers).await
}

pub fn stash_pending_cli_consent(jar: &mut CookieJar, url: &str) {
    if !url.starts_with(PENDING_CLI_CONSENT_PATH_PREFIX) {
        return;
    }
    jar.add(
        cookie::CookieBuilder::new(PENDING_CLI_CONSENT_COOKIE, url.to_string())
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Lax)
            .path("/")
            .max_age(time::Duration::seconds(PENDING_CLI_CONSENT_MAX_AGE_SECS)),
    );
}

pub fn take_pending_cli_consent(jar: &mut CookieJar) -> Option<String> {
    let cookie = jar.get(PENDING_CLI_CONSENT_COOKIE)?;
    let url = cookie.value().to_string();
    jar.remove(cookie::CookieBuilder::new(PENDING_CLI_CONSENT_COOKIE, "").path("/"));
    if !url.starts_with(PENDING_CLI_CONSENT_PATH_PREFIX) {
        return None;
    }
    Some(url)
}

pub fn is_loopback_redirect(redirect_uri: &str) -> bool {
    let Ok(uri) = redirect_uri.parse::<::http::Uri>() else {
        return false;
    };
    if uri.scheme_str() != Some("http") {
        return false;
    }
    let Some(host) = uri.host() else {
        return false;
    };
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}
