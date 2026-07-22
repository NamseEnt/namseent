use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const PRESIGNED_PUT_EXPIRES_SECS: u64 = 600;

#[derive(Deserialize)]
pub struct Input {
    pub build_id: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok {
        build_id: String,
        hmac_key_hex: String,
        pdb_presigned_put_url: String,
    },
    NotLoggedIn,
    InvalidBuildId,
    Error {
        message: String,
    },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(user) = auth::session_or_bearer_user(req.jar, req.headers).await else {
        return Output::NotLoggedIn;
    };

    let build_id = req.body.build_id.trim().to_string();
    if build_id.is_empty() || !is_safe_id(&build_id) {
        return Output::InvalidBuildId;
    }

    let db = doc_db::turso();
    let existing = match (BuildDocGet {
        build_id: build_id.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(b) => b,
        Err(e) => {
            tracing::error!(?e, "request_pdb_upload BuildDocGet");
            return Output::Error {
                message: format!("BuildDocGet: {e}"),
            };
        }
    };

    let r2_key = format!("pdb/{build_id}.pdb");

    let build = match existing {
        Some(b) => b,
        None => {
            let key_bytes = rand::get_random_bytes(32).await;
            let hmac_key_hex = hex::encode(&key_bytes);
            let fresh = BuildDoc {
                build_id: build_id.clone(),
                created_at: forte_sdk::now(),
                uploaded_by: user.github_id,
                hmac_key_hex,
                pdb_uploaded: false,
                pdb_r2_key: Some(r2_key.clone()),
                pdb_size: None,
            };
            if let Err(e) = BuildDocPut(fresh.clone()).send_with(&db).await {
                tracing::error!(?e, "request_pdb_upload BuildDocPut");
                return Output::Error {
                    message: format!("BuildDocPut: {e}"),
                };
            }
            fresh
        }
    };

    let bucket = object_storage::bucket();
    let pdb_presigned_put_url = match bucket
        .presigned_put_url(&r2_key, None, Duration::from_secs(PRESIGNED_PUT_EXPIRES_SECS))
        .await
    {
        Ok(u) => u,
        Err(e) => {
            tracing::error!(?e, "request_pdb_upload presigned_put_url");
            return Output::Error {
                message: format!("presigned_put_url: {e}"),
            };
        }
    };

    Output::Ok {
        build_id: build.build_id,
        hmac_key_hex: build.hmac_key_hex,
        pdb_presigned_put_url,
    }
}

fn is_safe_id(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && s.len() <= 128
}
