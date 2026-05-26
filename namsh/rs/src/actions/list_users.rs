use crate::common::admin;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct UserSummary {
    pub github_id: i64,
    pub github_login: String,
    pub created_at: DateTime,
}

#[derive(Serialize)]
pub enum Output {
    Ok { users: Vec<UserSummary> },
    Unauthorized,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    if !admin::verify(req.headers) {
        return Output::Unauthorized;
    }

    let db = doc_db::turso();
    let docs: Vec<UserDoc> = match (UserDocQuery {
        github_id: None,
        limit: None,
    })
    .send_with(&db)
    .await
    {
        Ok(v) => v,
        Err(e) => {
            return Output::Error {
                message: e.to_string(),
            };
        }
    };

    let users = docs
        .into_iter()
        .map(|u| UserSummary {
            github_id: u.github_id,
            github_login: u.github_login,
            created_at: u.created_at,
        })
        .collect();
    Output::Ok { users }
}
