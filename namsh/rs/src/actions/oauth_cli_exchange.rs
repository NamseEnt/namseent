use crate::common::auth;
use crate::docs::*;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use forte_sdk::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
pub struct Input {
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { token: String },
    InvalidGrant { message: String },
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let db = doc_db::turso();

    let auth_code = match (CliAuthorizationCodeDocGet {
        code: req.body.code.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Output::InvalidGrant {
                message: "code not found".to_string(),
            };
        }
        Err(e) => {
            return Output::Error {
                message: e.to_string(),
            };
        }
    };

    if let Err(e) = (CliAuthorizationCodeDocDelete {
        code: req.body.code.clone(),
    })
    .send_with(&db)
    .await
    {
        return Output::Error {
            message: e.to_string(),
        };
    }

    if auth_code.expires_at < forte_sdk::now() {
        return Output::InvalidGrant {
            message: "code expired".to_string(),
        };
    }
    if auth_code.redirect_uri != req.body.redirect_uri {
        return Output::InvalidGrant {
            message: "redirect_uri mismatch".to_string(),
        };
    }

    let mut hasher = Sha256::new();
    hasher.update(req.body.code_verifier.as_bytes());
    let computed_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());
    if computed_challenge != auth_code.code_challenge {
        return Output::InvalidGrant {
            message: "code_verifier does not match code_challenge".to_string(),
        };
    }

    let mut user = match (UserDocGet {
        github_id: auth_code.github_id,
    })
    .send_with(&db)
    .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Output::InvalidGrant {
                message: "user no longer exists".to_string(),
            };
        }
        Err(e) => {
            return Output::Error {
                message: e.to_string(),
            };
        }
    };

    let bytes = rand::get_random_bytes(16).await;
    let Ok(uuid_bytes): Result<[u8; 16], _> = bytes.as_slice().try_into() else {
        return Output::Error {
            message: "rng returned wrong length".to_string(),
        };
    };
    let token_uuid = Uuid::from_bytes(uuid_bytes);

    let token = match auth::mint_cli_token(user.github_id, &token_uuid) {
        Ok(t) => t,
        Err(e) => {
            return Output::Error {
                message: e.to_string(),
            };
        }
    };

    user.cli_tokens.push(CliTokenEntry {
        id: token_uuid.to_string(),
        label: auth_code.label,
        created_at: forte_sdk::now(),
    });

    if let Err(e) = UserDocPut(user).send_with(&db).await {
        return Output::Error {
            message: e.to_string(),
        };
    }

    Output::Ok { token }
}
