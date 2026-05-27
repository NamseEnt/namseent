use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct StackGroupSummary {
    pub stack_hash: String,
    pub first_seen: DateTime,
    pub last_seen: DateTime,
    pub count: u64,
    pub stored_dumps: usize,
    pub latest_build_id: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { groups: Vec<StackGroupSummary> },
    NotLoggedIn,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(_user) = auth::session_or_bearer_user(req.jar, req.headers).await else {
        return Output::NotLoggedIn;
    };

    let db = doc_db::turso();
    let docs: Vec<StackGroupDoc> = match (StackGroupDocQuery {
        stack_hash: None,
        limit: None,
    })
    .send_with(&db)
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!(?e, "list_stack_groups StackGroupDocQuery");
            return Output::Error {
                message: format!("StackGroupDocQuery: {e}"),
            };
        }
    };

    let mut groups: Vec<StackGroupSummary> = docs
        .into_iter()
        .map(|g| StackGroupSummary {
            stack_hash: g.stack_hash,
            first_seen: g.first_seen,
            last_seen: g.last_seen,
            count: g.count,
            stored_dumps: g.dump_ids.len(),
            latest_build_id: g.latest_context.build_id,
        })
        .collect();
    groups.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));

    Output::Ok { groups }
}
