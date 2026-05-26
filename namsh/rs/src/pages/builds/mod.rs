use crate::actions::list_builds::BuildSummary;
use crate::common::auth;
use crate::docs::*;
use crate::route_generated::Redirect;
use forte_sdk::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Props {
    pub github_login: String,
    pub builds: Vec<BuildSummary>,
}

pub async fn handler(req: ForteRequest<'_>) -> anyhow::Result<Props> {
    let Some(user) = auth::current_user(req.jar).await else {
        return Err(Redirect::Login.into());
    };

    let db = doc_db::turso();
    let docs: Vec<BuildDoc> = (BuildDocQuery {
        build_id: None,
        limit: None,
    })
    .send_with(&db)
    .await?;

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

    Ok(Props {
        github_login: user.github_login,
        builds,
    })
}
