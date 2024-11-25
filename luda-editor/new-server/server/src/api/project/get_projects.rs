use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{project::get_projects::*, Project};

pub async fn get_projects(
    &ArchivedRequest { team_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let team_doc = db
        .get(TeamDocGet { id: team_id })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    let project_docs = try_join_all(
        team_doc
            .project_ids
            .iter()
            .map(|&project_id| async move { db.get(ProjectDocGet { id: project_id }).await }),
    )
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
