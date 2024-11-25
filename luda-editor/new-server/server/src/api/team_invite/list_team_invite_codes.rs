use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{team_invite::list_team_invite_codes::*, TeamInviteCode};

pub async fn list_team_invite_codes(
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

    let team_invite_code_docs = try_join_all(
        team_doc
            .invite_codes
            .iter()
            .map(|&code| async move { db.get(TeamInviteCodeDocGet { code }).await }),
    )
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize());

    Ok(Response {
        codes: team_invite_code_docs
            .map(|x| TeamInviteCode {
                code: x.code,
                expiration_time: x.expiration_time,
            })
            .collect(),
    })
}
