use crate::*;
use database::schema::*;
use luda_rpc::team::create_new_team::*;

const MAX_TEAM_COUNT: usize = 20;

pub async fn create_new_team(
    ArchivedRequest { name }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    let user_team_query = db.query(UserToTeamDocQuery { user_id: &user_id }).await?;
    if user_team_query.len() > MAX_TEAM_COUNT {
        bail!(Error::TooManyTeams)
    }

    let team_id = randum::rand();

    db.transact::<()>((
        TeamNameToTeamIdDocCreate {
            team_name: name,
            team_id: &team_id,
            ttl: None,
        },
        TeamDocPut {
            id: &team_id,
            name,
            ttl: None,
        },
        UserToTeamDocPut {
            user_id: &user_id,
            team_id: &team_id,
            ttl: None,
        },
        TeamAssetTotalBytesDocPut {
            team_id: &team_id,
            used_bytes: 0,
            limit_bytes: 100 * 1024 * 1024, // 100MB
            ttl: None,
        },
    ))
    .await
    .map_err(|err| match err {
        database::Error::AlreadyExistsOnCreate => anyhow!(Error::DuplicatedName),
        _ => anyhow!(err),
    })?;

    Ok(Response { team_id })
}
