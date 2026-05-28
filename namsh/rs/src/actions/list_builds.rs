use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct BuildSummary {
    pub build_id: String,
    pub created_at: DateTime,
    pub uploaded_by: i64,
    pub pdb_uploaded: bool,
    pub pdb_size: Option<u64>,
}

#[derive(Serialize)]
pub enum Output {
    Ok { builds: Vec<BuildSummary> },
    NotLoggedIn,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(_user) = auth::session_or_bearer_user(req.jar, req.headers).await else {
        return Output::NotLoggedIn;
    };

    let db = doc_db::turso();
    let docs: Vec<BuildDoc> = match (BuildDocQuery {
        build_id: None,
        limit: None,
    })
    .send_with(&db)
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(?e, "list_builds BuildDocQuery");
            return Output::Error {
                message: format!("BuildDocQuery: {e}"),
            };
        }
    };

    let mut builds: Vec<BuildSummary> = docs
        .into_iter()
        .map(|b| BuildSummary {
            build_id: b.build_id,
            created_at: b.created_at,
            uploaded_by: b.uploaded_by,
            pdb_uploaded: b.pdb_uploaded,
            pdb_size: b.pdb_size,
        })
        .collect();
    builds.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Output::Ok { builds }
}
