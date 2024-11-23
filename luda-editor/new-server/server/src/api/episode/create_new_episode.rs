use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, WantUpdate};
use luda_rpc::episode::create_new_episode::*;

pub async fn create_new_episode(
    &ArchivedRequest {
        project_id,
        ref name,
    }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let project_doc = db
        .get(ProjectDocGet { id: project_id })
        .await?
        .ok_or(Error::ProjectNotExist)?;

    let team_doc = db
        .get(TeamDocGet {
            id: project_doc.team_id,
        })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    let episode_id = new_id();

    db.transact::<()>((
        EpisodeDocPut {
            id: episode_id,
            name,
            created_at: SystemTime::now(),
            team_id: project_doc.team_id,
            project_id,
            scenes: &Default::default(),
            editing_user: &Default::default(),
            speaker_slots: &Default::default(),
        },
        ProjectDocUpdate {
            id: project_id,
            want_update: |_| WantUpdate::Yes,
            update: |doc| {
                doc.episode_ids.insert(episode_id);
            },
        },
    ))
    .await?;

    Ok(Response {})
}
