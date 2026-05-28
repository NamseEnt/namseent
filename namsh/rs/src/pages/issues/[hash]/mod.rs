use crate::actions::get_stack_group::DumpSummary;
use crate::common::auth;
use crate::docs::*;
use crate::route_generated::Redirect;
use forte_sdk::*;
use serde::Serialize;

pub struct PathParams {
    pub hash: String,
}

#[derive(Serialize)]
pub enum Props {
    Ok {
        github_login: String,
        stack_hash: String,
        first_seen: DateTime,
        last_seen: DateTime,
        count: u64,
        latest_context: CrashContext,
        dumps: Vec<DumpSummary>,
    },
    NotFound {
        github_login: String,
        stack_hash: String,
    },
}

pub async fn handler(
    req: ForteRequest<'_>,
    path_params: PathParams,
) -> anyhow::Result<Props> {
    let Some(user) = auth::current_user(req.jar).await else {
        return Err(Redirect::Login.into());
    };

    let db = doc_db::turso();
    let Some(group) = (StackGroupDocGet {
        stack_hash: path_params.hash.clone(),
    })
    .send_with(&db)
    .await?
    else {
        return Ok(Props::NotFound {
            github_login: user.github_login,
            stack_hash: path_params.hash,
        });
    };

    let mut dumps: Vec<DumpSummary> = Vec::with_capacity(group.dump_ids.len());
    for dump_id in &group.dump_ids {
        if let Some(d) = (DumpDocGet {
            dump_id: dump_id.clone(),
        })
        .send_with(&db)
        .await?
        {
            dumps.push(DumpSummary {
                dump_id: d.dump_id,
                build_id: d.build_id,
                uploaded_at: d.uploaded_at,
                client_ip: d.client_ip,
            });
        }
    }

    Ok(Props::Ok {
        github_login: user.github_login,
        stack_hash: group.stack_hash,
        first_seen: group.first_seen,
        last_seen: group.last_seen,
        count: group.count,
        latest_context: group.latest_context,
        dumps,
    })
}
