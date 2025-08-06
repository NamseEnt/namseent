use crate::*;
use database::{WantUpdate, schema::*};
use luda_rpc::team_invite::join_team::*;

pub async fn join_team(
    Request { code }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let Some(team_invite_code_doc) = db.get(TeamInviteCodeDocGet { code }).await? else {
        bail!(Error::InvalidCode)
    };

    if team_invite_code_doc.expiration_time < SystemTime::now() {
        bail!(Error::InvalidCode)
    }

    enum AbortReason {
        AlreadyJoined,
    }

    db.transact::<AbortReason>((
        UserDocUpdate {
            id: user_id,
            want_update: |doc| {
                if doc.team_ids.contains(&team_invite_code_doc.team_id) {
                    return WantUpdate::Abort {
                        reason: AbortReason::AlreadyJoined,
                    };
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.team_ids.insert(team_invite_code_doc.team_id);
            },
        },
        TeamDocUpdate {
            id: team_invite_code_doc.team_id,
            want_update: |doc| {
                if doc.member_ids.contains(&user_id) {
                    return WantUpdate::Abort {
                        reason: AbortReason::AlreadyJoined,
                    };
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.member_ids.insert(user_id);
            },
        },
    ))
    .await?
    .err_if_aborted(|reason| match reason {
        AbortReason::AlreadyJoined => Error::AlreadyJoined,
    })?;

    Ok(Response {})
}
