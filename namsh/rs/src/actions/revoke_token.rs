use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub id: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok,
    NotLoggedIn,
    NotFound,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(mut user) = auth::current_user(req.jar).await else {
        return Output::NotLoggedIn;
    };

    let before = user.cli_tokens.len();
    user.cli_tokens.retain(|t| t.id != req.body.id);
    if user.cli_tokens.len() == before {
        return Output::NotFound;
    }

    let db = doc_db::turso();
    if let Err(e) = UserDocPut(user).send_with(&db).await {
        return Output::Error {
            message: e.to_string(),
        };
    }
    Output::Ok
}
