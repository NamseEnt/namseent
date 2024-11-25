use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, WantUpdate};
use luda_rpc::{team_invite::create_team_invite_code::*, TeamInviteCode};
use std::time::Duration;

const MAX_TEAM_INVITE_CODE_COUNT: usize = 20;
const SEVEN_DAYS: Duration = Duration::from_secs(3600 * 24 * 7);

pub async fn create_team_invite_code(
    &ArchivedRequest { team_id }: &ArchivedRequest,
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
    let code = new_id();
    let expiration_time = SystemTime::now() + SEVEN_DAYS;

    enum AbortReason {
        TooManyCodes,
    }

    db.transact::<AbortReason>((
        TeamInviteCodeDocPut {
            team_id,
            code,
            expiration_time,
        },
        TeamDocUpdate {
            id: team_id,
            want_update: |doc| {
                if doc.invite_codes.len() > MAX_TEAM_INVITE_CODE_COUNT {
                    return WantUpdate::Abort {
                        reason: AbortReason::TooManyCodes,
                    };
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.invite_codes.insert(code);
            },
        },
    ))
    .await?
    .err_if_aborted(|reason| match reason {
        AbortReason::TooManyCodes => Error::TooManyCodes,
    })?;

    Ok(Response {
        code: TeamInviteCode {
            code,
            expiration_time,
        },
    })
}
