use crate::*;
use api::team::is_team_member;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::scene::get_scenes::*;

pub async fn get_scenes(
    ArchivedRequest { episode_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let Some(user_id) = session.user_id().await else {
        bail!(Error::NeedLogin)
    };

    let project_id = &db
        .get(EpisodeToProjectDocGet { episode_id })
        .await?
        .ok_or(Error::EpisodeNotExist)?
        .project_id;

    let team_id = &db
        .get(ProjectToTeamDocGet { project_id })
        .await?
        .ok_or(Error::ProjectNotExist)?
        .team_id;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let episode_doc = db
        .get(EpisodeDocGet { id: episode_id })
        .await?
        .ok_or(Error::EpisodeNotExist)?;

    let scenes = try_join_all(episode_doc.scene_ids.iter().map(|scene_id| async move {
        db.get(SceneDocGet {
            id: scene_id.as_str(),
        })
        .await
    }))
    .await?
    .into_iter()
    .flatten()
    .map(|scene_doc| scene_doc.deserialize())
    .collect();

    Ok(Response { scenes })
}
