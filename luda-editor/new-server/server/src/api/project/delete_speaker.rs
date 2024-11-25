use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, WantUpdate};
use luda_rpc::project::delete_speaker::*;

pub async fn delete_speaker(
    &ArchivedRequest {
        project_id,
        speaker_id,
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
        .ok_or(Error::ProjectNotExist)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    db.transact::<()>(ProjectDocUpdate {
        id: project_id,
        want_update: |doc| {
            if doc.speakers.contains_key(&speaker_id) {
                WantUpdate::Yes
            } else {
                WantUpdate::No
            }
        },
        update: |doc| {
            doc.speakers.remove(&speaker_id);
        },
    })
    .await?;

    Ok(Response {})
}
