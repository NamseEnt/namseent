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

    let team_id = &db
        .get(ProjectToTeamDocGet { project_id })
        .await?
        .ok_or(Error::ProjectNotExist)?
        .team_id;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let project_to_episode_query = db.query(ProjectToEpisodeDocQuery { project_id }).await?;
    let episode_docs = try_join_all(project_to_episode_query.into_iter().map(|doc| async move {
        db.get(EpisodeDocGet {
            id: doc.episode_id.as_str(),
        })
        .await
    }))
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize());

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
