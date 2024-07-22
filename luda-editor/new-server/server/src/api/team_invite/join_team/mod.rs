use crate::*;
use database::schema::*;
use luda_rpc::team_invite::join_team::*;

pub async fn join_team(
    ArchivedRequest { code }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    let Some(team_invite_code_to_team) = db.get(TeamInviteCodeToTeamDocGet { code }).await? else {
        bail!(Error::InvalidCode)
    };

    db.transact(UserToTeamDocPut {
        user_id: &user_id,
        team_id: &team_invite_code_to_team.team_id,
        ttl: None,
    })
    .await?;

    Ok(Response {})
}
