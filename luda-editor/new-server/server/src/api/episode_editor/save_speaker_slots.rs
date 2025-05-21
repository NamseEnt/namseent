use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, WantUpdate};
use luda_rpc::episode_editor::save_speaker_slots::*;

pub async fn save_speaker_slots(
    Request {
        episode_id,
        speaker_ids,
    }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let episode_doc = db
        .get(EpisodeDocGet { id: episode_id })
        .await?
        .ok_or(Error::EpisodeNotExists)?;

    let team_doc = db
        .get(TeamDocGet {
            id: episode_doc.team_id,
        })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    db.transact::<()>(EpisodeDocUpdate {
        id: episode_id,
        want_update: |_| WantUpdate::Yes,
        update: |doc| {
            doc.speaker_slots
                .insert(user_id, speaker_ids.iter().cloned().collect());
        },
    })
    .await?;

    Ok(Response {})
}
