use crate::*;
use api::team::has_episode_edit_permission;
use database::schema::*;
use luda_rpc::episode_editor::save_speaker_slots::*;

pub async fn save_speaker_slots(
    ArchivedRequest {
        episode_id,
        speaker_ids,
    }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !has_episode_edit_permission(db, episode_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    db.transact::<()>(EpisodeSpeakerSlotDocPut {
        user_id: &user_id,
        episode_id,
        speaker_ids,
        ttl: None,
    })
    .await?;

    Ok(Response {})
}
