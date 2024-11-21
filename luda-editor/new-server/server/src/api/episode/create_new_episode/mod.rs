use crate::*;
use api::team::is_project_member;
use database::schema::*;
use luda_rpc::episode::create_new_episode::*;

pub async fn create_new_episode(
    ArchivedRequest { project_id, name }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_project_member(db, project_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let episode_id = randum::uuid();

    db.transact::<()>((
        EpisodeDocPut {
            id: &episode_id,
            name,
            created_at: SystemTime::now(),
            scene_ids: &Vec::<String>::new(),
            ttl: None,
        },
        ProjectToEpisodeDocPut {
            project_id,
            episode_id: &episode_id,
            ttl: None,
        },
        EpisodeToProjectDocPut {
            episode_id: &episode_id,
            project_id,
            ttl: None,
        },
        EpisodeEditingUserDocPut {
            episode_id: &episode_id,
            editing_user: &None,
            ttl: None,
        },
    ))
    .await?;

    Ok(Response {})
}
