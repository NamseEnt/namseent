use crate::*;
use database::schema::*;
use luda_rpc::{team_invite::list_team_invite_codes::*, TeamInviteCode};

pub async fn list_team_invite_codes(
    ArchivedRequest { team_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if db
        .get(UserToTeamDocGet {
            user_id: &user_id,
            team_id,
        })
        .await?
        .is_none()
    {
        bail!(Error::PermissionDenied)
    };

    let query = db.query(TeamInviteCodeDocQuery { team_id }).await?;

    Ok(Response {
        codes: query
            .into_iter()
            .map(|x| x.deserialize())
            .map(|x| TeamInviteCode {
                code: x.code,
                expiration_time: x.expiration_time,
            })
            .collect(),
    })
}
