use crate::common::admin;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub github_id: i64,
    pub github_login: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok,
    Unauthorized,
    AlreadyExists,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    if !admin::verify(req.headers) {
        return Output::Unauthorized;
    }

    let db = doc_db::turso();
    match (UserDocGet {
        github_id: req.body.github_id,
    })
    .send_with(&db)
    .await
    {
        Ok(Some(_)) => return Output::AlreadyExists,
        Ok(None) => {}
        Err(e) => {
            return Output::Error {
                message: e.to_string(),
            };
        }
    }

    let fresh = UserDoc {
        github_id: req.body.github_id,
        github_login: req.body.github_login.clone(),
        created_at: forte_sdk::now(),
        cli_tokens: Vec::new(),
        web_sessions: Vec::new(),
    };
    if let Err(e) = UserDocPut(fresh).send_with(&db).await {
        return Output::Error {
            message: e.to_string(),
        };
    }
    Output::Ok
}
