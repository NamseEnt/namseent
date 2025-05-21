use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, WantUpdate};
use luda_rpc::project::put_speaker::*;

pub async fn put_speaker(
    Request {
        project_id,
        speaker,
    }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let project_doc = db
        .get(ProjectDocGet { id: project_id })
        .await?
        .ok_or(Error::ProjectNotExists)?;

    let team_doc = db
        .get(TeamDocGet {
            id: project_doc.team_id,
        })
        .await?
        .ok_or(Error::TeamNotExists)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    db.transact::<()>(ProjectDocUpdate {
        id: project_id,
        want_update: |doc| {
            if let Some(value) = doc.speakers.get(&speaker.id) {
                if value == &speaker {
                    return WantUpdate::No;
                }
            }
            WantUpdate::Yes
        },
        update: |doc| {
            doc.speakers.insert(speaker.id, speaker.clone());
        },
    })
    .await?;

    Ok(Response {})
}
