use crate::common::auth;
use crate::docs::*;
use crate::route_generated::Redirect;
use forte_sdk::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct StackGroupRow {
    pub stack_hash: String,
    pub first_seen: DateTime,
    pub last_seen: DateTime,
    pub count: u64,
    pub stored_dumps: usize,
    pub latest_build_id: String,
}

#[derive(Serialize)]
pub struct Props {
    pub github_login: String,
    pub groups: Vec<StackGroupRow>,
}

pub async fn handler(req: ForteRequest<'_>) -> anyhow::Result<Props> {
    let Some(user) = auth::current_user(req.jar).await else {
        return Err(Redirect::Login.into());
    };

    let db = doc_db::turso();
    let docs: Vec<StackGroupDoc> = (StackGroupDocQuery {
        stack_hash: None,
        limit: None,
    })
    .send_with(&db)
    .await?;

    let mut groups: Vec<StackGroupRow> = docs
        .into_iter()
        .map(|g| StackGroupRow {
            stack_hash: g.stack_hash,
            first_seen: g.first_seen,
            last_seen: g.last_seen,
            count: g.count,
            stored_dumps: g.dump_ids.len(),
            latest_build_id: g.latest_context.build_id,
        })
        .collect();
    groups.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));

    Ok(Props {
        github_login: user.github_login,
        groups,
    })
}
