use crate::*;
use database::schema::*;
use luda_rpc::team_invite::invalidate_team_invite_code::*;

pub async fn invalidate_team_invite_code(
    ArchivedRequest { team_id, code }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    let Some(user_id) = session.user_id().await else {
        return Err(Error::NeedLogin);
    };

    let user_to_team_query = db.query(UserToTeamDocQuery { user_id: &user_id }).await?;
    let is_team_member = user_to_team_query.iter().any(|doc| doc.team_id == *team_id);

    if !is_team_member {
        return Err(Error::PermissionDenied);
    }

    db.transact((
        TeamInviteCodeDocDelete { team_id, code },
        TeamInviteCodeToTeamDocDelete { code },
    ))
    .await?;

    Ok(Response {})
}
