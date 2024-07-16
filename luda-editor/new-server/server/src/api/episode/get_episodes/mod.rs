use crate::*;
use api::team::is_team_member;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{episode::get_episodes::*, Episode};

pub async fn get_episodes(
    ArchivedRequest { project_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let Some(user_id) = session.user_id().await else {
        bail!(Error::NeedLogin)
    };

    let project_doc = db
        .get(ProjectDocGet { id: project_id })
        .await?
        .ok_or(Error::ProjectNotExist)?;
    let team_id = &project_doc.team_id;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let project_to_episode_query = db.query(ProjectToEpisodeDocQuery { project_id }).await?;
    let episode_docs = try_join_all(project_to_episode_query.into_iter().map(|doc| async move {
        db.get(EpisodeDocGet {
            id: doc.episode_id.as_str(),
        })
        .await?
        .ok_or(anyhow!("episode not found: {}", doc.episode_id))
    }))
    .await?;

    Ok(Response {
        episodes: episode_docs
            .into_iter()
            .map(|x| x.deserialize())
            .map(|doc| Episode {
                id: doc.id,
                name: doc.name,
                created_at: doc.created_at,
            })
            .collect(),
    })
}
