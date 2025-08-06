use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{episode::get_episodes::*, Episode};

pub async fn get_episodes(
    Request { project_id }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let project_doc = db
        .get(ProjectDocGet { id: project_id })
        .await?
        .ok_or(Error::ProjectNotExists)?;

    let team_doc = db
        .get(TeamDocGet {
            id: project_doc.team_id,
        })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    let episode_docs = try_join_all(
        project_doc
            .episode_ids
            .iter()
            .map(|&id| async move { db.get(EpisodeDocGet { id }).await }),
    )
    .await?
    .into_iter()
    .flatten();

    Ok(Response {
        episodes: episode_docs
            .map(|doc| Episode {
                id: doc.id,
                name: doc.name,
                created_at: doc.created_at,
            })
            .collect(),
    })
}
