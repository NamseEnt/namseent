use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub build_id: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { size: u64 },
    NotLoggedIn,
    BuildNotFound,
    NotUploaded,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(_user) = auth::session_or_bearer_user(req.jar, req.headers).await else {
        return Output::NotLoggedIn;
    };

    let db = doc_db::turso();
    let mut build = match (BuildDocGet {
        build_id: req.body.build_id.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(Some(b)) => b,
        Ok(None) => return Output::BuildNotFound,
        Err(e) => {
            tracing::error!(?e, "confirm_pdb_uploaded BuildDocGet");
            return Output::Error {
                message: format!("BuildDocGet: {e}"),
            };
        }
    };

    let Some(r2_key) = build.pdb_r2_key.clone() else {
        return Output::NotUploaded;
    };

    let bucket = object_storage::bucket();
    let metadata = match bucket.head(&r2_key).await {
        Ok(Some(m)) => m,
        Ok(None) => return Output::NotUploaded,
        Err(e) => {
            tracing::error!(?e, "confirm_pdb_uploaded head");
            return Output::Error {
                message: format!("head: {e}"),
            };
        }
    };

    build.pdb_uploaded = true;
    build.pdb_size = Some(metadata.size);
    if let Err(e) = BuildDocPut(build).send_with(&db).await {
        tracing::error!(?e, "confirm_pdb_uploaded BuildDocPut");
        return Output::Error {
            message: format!("BuildDocPut: {e}"),
        };
    }
    Output::Ok { size: metadata.size }
}
