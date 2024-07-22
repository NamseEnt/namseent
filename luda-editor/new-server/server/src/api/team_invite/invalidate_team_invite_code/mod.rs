use crate::*;
use api::team::is_team_member;
use database::schema::*;
use luda_rpc::team_invite::invalidate_team_invite_code::*;

pub async fn invalidate_team_invite_code(
    ArchivedRequest { team_id, code }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    db.transact((
        TeamInviteCodeDocDelete { team_id, code },
        TeamInviteCodeToTeamDocDelete { code },
    ))
    .await?;

    Ok(Response {})
}
