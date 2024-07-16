use crate::*;
use api::team::is_team_member;
use database::schema::*;
use luda_rpc::episode::create_new_episode::*;

pub async fn create_new_episode(
    ArchivedRequest { project_id, name }: &ArchivedRequest,
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

    let episode_id = randum::rand();

    db.transact((
        EpisodeDocPut {
            id: &episode_id,
            name,
            created_at: SystemTime::now(),
            scene_ids: &Vec::new(),
            ttl: None,
        },
        ProjectToEpisodeDocPut {
            project_id,
            episode_id: &episode_id,
            ttl: None,
        },
    ))
    .await?;

    Ok(Response {})
}
