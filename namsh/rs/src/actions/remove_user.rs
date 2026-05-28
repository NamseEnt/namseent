use crate::common::admin;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub github_id: i64,
}

#[derive(Serialize)]
pub enum Output {
    Ok,
    Unauthorized,
    NotFound,
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
        Ok(Some(_)) => {}
        Ok(None) => return Output::NotFound,
        Err(e) => {
            return Output::Error {
                message: e.to_string(),
            };
        }
    }

    if let Err(e) = (UserDocDelete {
        github_id: req.body.github_id,
    })
    .send_with(&db)
    .await
    {
        return Output::Error {
            message: e.to_string(),
        };
    }
    Output::Ok
}
