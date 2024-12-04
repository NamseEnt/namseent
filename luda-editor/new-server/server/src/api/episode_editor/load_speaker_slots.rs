use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use luda_rpc::episode_editor::load_speaker_slots::*;

pub async fn load_speaker_slots(
    Request { episode_id }: Request,
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

    Ok(Response {
        speaker_ids: episode_doc
            .speaker_slots
            .get(&user_id)
            .map(|x| x.iter().cloned().collect())
            .unwrap_or_default(),
    })
}
