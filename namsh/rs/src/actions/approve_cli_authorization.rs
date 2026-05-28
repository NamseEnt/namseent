use crate::common::auth;
use crate::docs::*;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

const CODE_TTL_SECS: i64 = 300;

#[derive(Deserialize)]
pub struct Input {
    pub redirect_uri: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
    pub state: String,
    pub label: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { redirect_to: String },
    NotLoggedIn,
    InvalidRequest { message: String },
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(user) = auth::current_user(req.jar).await else {
        return Output::NotLoggedIn;
    };

    if req.body.code_challenge_method != "S256" {
        return Output::InvalidRequest {
            message: "code_challenge_method must be S256".to_string(),
        };
    }
    if req.body.code_challenge.is_empty() {
        return Output::InvalidRequest {
            message: "code_challenge is required".to_string(),
        };
    }
    if !auth::is_loopback_redirect(&req.body.redirect_uri) {
        return Output::InvalidRequest {
            message: "redirect_uri must be http loopback (127.0.0.1, localhost, or [::1])"
                .to_string(),
        };
    }

    let label = req.body.label.trim().to_string();
    if label.is_empty() {
        return Output::InvalidRequest {
            message: "label cannot be empty".to_string(),
        };
    }

    let code_bytes = rand::get_random_bytes(32).await;
    let code = URL_SAFE_NO_PAD.encode(&code_bytes);

    let now = forte_sdk::now();
    let expires_at = now + forte_sdk::chrono::Duration::seconds(CODE_TTL_SECS);

    let db = doc_db::turso();
    let put_result = CliAuthorizationCodeDocPut(CliAuthorizationCodeDoc {
        code: code.clone(),
        github_id: user.github_id,
        code_challenge: req.body.code_challenge.clone(),
        redirect_uri: req.body.redirect_uri.clone(),
        label,
        expires_at,
    })
    .send_with(&db)
    .await;
    if let Err(e) = put_result {
        return Output::Error {
            message: e.to_string(),
        };
    }

    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("code", &code)
        .append_pair("state", &req.body.state)
        .finish();
    let separator = if req.body.redirect_uri.contains('?') {
        '&'
    } else {
        '?'
    };
    let redirect_to = format!("{}{}{}", req.body.redirect_uri, separator, query);

    Output::Ok { redirect_to }
}
