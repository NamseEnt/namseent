use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{Project, project::get_projects::*};

pub async fn get_projects(
    Request { team_id }: Request,
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
    .flatten();

    Ok(Response {
        projects: project_docs
            .map(|doc| Project {
                id: doc.id,
                name: doc.name,
            })
            .collect(),
    })
}
