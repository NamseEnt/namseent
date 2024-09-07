use crate::*;
use api::team::is_project_member;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::episode_editor::get_speaker_names::*;

pub async fn get_speaker_names(
    ArchivedRequest {
        project_id,
        speaker_ids,
        language_code,
    }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_project_member(db, project_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let speaker_names = try_join_all(speaker_ids.iter().map(|speaker_id| async move {
        db.get(SpeakerNameL10nDocGet {
            speaker_id,
            language_code,
        })
        .await
    }))
    .await?
    .into_iter()
    .map(|x| x.map(|x| x.text.to_string()))
    .collect();

    Ok(Response { speaker_names })
}
