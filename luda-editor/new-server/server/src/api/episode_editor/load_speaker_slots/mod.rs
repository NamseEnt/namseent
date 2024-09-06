use crate::*;
use api::team::has_episode_edit_permission;
use database::{schema::*, DeserializeInfallible};
use luda_rpc::episode_editor::load_speaker_slots::*;

pub async fn load_speaker_slots(
    ArchivedRequest { episode_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !has_episode_edit_permission(db, episode_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let doc = db
        .get(EpisodeSpeakerSlotDocGet {
            user_id: &user_id,
            episode_id,
        })
        .await?
        .ok_or(Error::EpisodeNotExist)?;

    Ok(Response {
        speaker_ids: doc.speaker_ids.deserialize(),
    })
}
