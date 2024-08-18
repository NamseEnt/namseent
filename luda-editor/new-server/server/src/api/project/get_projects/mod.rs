use crate::*;
use api::team::is_team_member;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{project::get_projects::*, Project};

pub async fn get_projects(
    ArchivedRequest { team_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let team_to_project_query = db.query(TeamToProjectDocQuery { team_id }).await?;
    let project_docs = try_join_all(team_to_project_query.into_iter().map(|doc| async move {
        db.get(ProjectDocGet {
            id: doc.project_id.as_str(),
        })
        .await
    }))
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize());

    Ok(Response {
        projects: project_docs
            .map(|doc| Project {
                id: doc.id,
                name: doc.name,
            })
            .collect(),
    })
}
