use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, WantUpdate};
use luda_rpc::team_invite::invalidate_team_invite_code::*;

pub async fn invalidate_team_invite_code(
    Request { team_id, code }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let team_doc = db
        .get(TeamDocGet { id: team_id })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    db.transact::<()>((
        TeamInviteCodeDocDelete { code },
        TeamDocUpdate {
            id: team_id,
            want_update: |doc| {
                if !doc.invite_codes.contains(&code) {
                    return WantUpdate::No;
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.invite_codes.remove(&code);
            },
        },
    ))
    .await?;

    Ok(Response {})
}
