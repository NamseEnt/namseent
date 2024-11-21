use crate::*;
use api::team::is_team_member;
use database::schema::*;
use luda_rpc::{team_invite::create_team_invite_code::*, TeamInviteCode};
use std::time::Duration;

const MAX_TEAM_INVITE_CODE_COUNT: usize = 20;
const SEVEN_DAYS: Duration = Duration::from_secs(3600 * 24 * 7);

pub async fn create_team_invite_code(
    ArchivedRequest { team_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let team_invite_codes = db.query(TeamInviteCodeDocQuery { team_id }).await?;

    if MAX_TEAM_INVITE_CODE_COUNT < team_invite_codes.len() {
        bail!(Error::TooManyCodes)
    }

    let code = randum::uuid();
    let expiration_time = SystemTime::now() + SEVEN_DAYS;

    db.transact::<()>((
        TeamInviteCodeDocPut {
            team_id,
            code: &code,
            expiration_time,
            ttl: Some(SEVEN_DAYS),
        },
        TeamInviteCodeToTeamDocPut {
            team_id,
            code: &code,
            ttl: Some(SEVEN_DAYS),
        },
    ))
    .await?;

    Ok(Response {
        code: TeamInviteCode {
            code,
            expiration_time,
        },
    })
}
