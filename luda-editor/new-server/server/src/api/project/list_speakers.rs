use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, DeserializeInfallible};
use luda_rpc::project::list_speakers::*;

pub async fn list_speakers(
    &ArchivedRequest { project_id }: &ArchivedRequest,
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
        .ok_or(Error::TeamNotExist)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    Ok(Response {
        speakers: project_doc
            .speakers
            .values()
            .map(|speaker| speaker.deserialize())
            .collect(),
    })
}
