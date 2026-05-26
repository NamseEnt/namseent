use crate::common::auth;
use crate::docs::*;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    pub stack_hash: String,
}

#[derive(Serialize)]
pub struct DumpSummary {
    pub dump_id: String,
    pub build_id: String,
    pub uploaded_at: DateTime,
    pub client_ip: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok {
        stack_hash: String,
        first_seen: DateTime,
        last_seen: DateTime,
        count: u64,
        latest_context: CrashContext,
        dumps: Vec<DumpSummary>,
    },
    NotLoggedIn,
    NotFound,
    Error {
        message: String,
    },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(_user) = auth::session_or_bearer_user(req.jar, req.headers).await else {
        return Output::NotLoggedIn;
    };

    let db = doc_db::turso();
    let group = match (StackGroupDocGet {
        stack_hash: req.body.stack_hash.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(Some(g)) => g,
        Ok(None) => return Output::NotFound,
        Err(e) => {
            tracing::error!(?e, "get_stack_group StackGroupDocGet");
            return Output::Error {
                message: format!("StackGroupDocGet: {e}"),
            };
        }
    };

    let mut dumps: Vec<DumpSummary> = Vec::with_capacity(group.dump_ids.len());
    for dump_id in &group.dump_ids {
        match (DumpDocGet {
            dump_id: dump_id.clone(),
        })
        .send_with(&db)
        .await
        {
            Ok(Some(d)) => dumps.push(DumpSummary {
                dump_id: d.dump_id,
                build_id: d.build_id,
                uploaded_at: d.uploaded_at,
                client_ip: d.client_ip,
            }),
            Ok(None) => {
                tracing::warn!(dump_id, "get_stack_group: missing DumpDoc");
            }
            Err(e) => {
                tracing::error!(?e, dump_id, "get_stack_group DumpDocGet");
                return Output::Error {
                    message: format!("DumpDocGet: {e}"),
                };
            }
        }
    }

    Output::Ok {
        stack_hash: group.stack_hash,
        first_seen: group.first_seen,
        last_seen: group.last_seen,
        count: group.count,
        latest_context: group.latest_context,
        dumps,
    }
}
