use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const PRESIGNED_GET_EXPIRES_SECS: u64 = 600;

#[derive(Deserialize)]
pub struct Input {
    pub build_id: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { presigned_get_url: String },
    NotLoggedIn,
    NotFound,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(_user) = auth::session_or_bearer_user(req.jar, req.headers).await else {
        return Output::NotLoggedIn;
    };

    let db = doc_db::turso();
    let build = match (BuildDocGet {
        build_id: req.body.build_id.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(Some(b)) => b,
        Ok(None) => return Output::NotFound,
        Err(e) => {
            tracing::error!(?e, "request_pdb_download BuildDocGet");
            return Output::Error {
                message: format!("BuildDocGet: {e}"),
            };
        }
    };

    let Some(r2_key) = build.pdb_r2_key else {
        return Output::NotFound;
    };

    let bucket = object_storage::bucket();
    let presigned_get_url = match bucket
        .presigned_get_url(&r2_key, Duration::from_secs(PRESIGNED_GET_EXPIRES_SECS))
        .await
    {
        Ok(u) => u,
        Err(e) => {
            tracing::error!(?e, "request_pdb_download presigned_get_url");
            return Output::Error {
                message: format!("presigned_get_url: {e}"),
            };
        }
    };

    Output::Ok { presigned_get_url }
}
