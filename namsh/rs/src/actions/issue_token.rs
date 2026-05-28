use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub label: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { id: String, token: String },
    NotLoggedIn,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(mut user) = auth::current_user(req.jar).await else {
        return Output::NotLoggedIn;
    };

    let label = req.body.label.trim().to_string();
    if label.is_empty() {
        return Output::Error {
            message: "label cannot be empty".to_string(),
        };
    }

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

    let token_id = token_uuid.to_string();
    user.cli_tokens.push(CliTokenEntry {
        id: token_id.clone(),
        label,
        created_at: forte_sdk::now(),
    });

    let db = doc_db::turso();
    if let Err(e) = UserDocPut(user).send_with(&db).await {
        return Output::Error {
            message: e.to_string(),
        };
    }

    Output::Ok {
        id: token_id,
        token,
    }
}
