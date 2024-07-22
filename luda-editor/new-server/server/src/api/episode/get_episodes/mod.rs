use crate::*;
use api::team::is_project_member;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{episode::get_episodes::*, Episode};

pub async fn get_episodes(
    ArchivedRequest { project_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_project_member(db, project_id, &user_id).await? {
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
