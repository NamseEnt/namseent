use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use luda_rpc::episode_editor::get_speaker_names::*;

pub async fn get_speaker_names(
    &ArchivedRequest {
        project_id,
        ref speaker_ids,
        ref language_code,
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

    Ok(Response {
        speaker_names: speaker_ids
            .iter()
            .map(|speaker_id| {
                project_doc
                    .speakers
                    .get(speaker_id)
                    .and_then(|x| x.name_l10n.get(language_code).map(|x| x.to_string()))
            })
            .collect(),
    })
}
