use crate::*;
use database::schema::*;
use luda_rpc::team::create_new_team::*;

const MAX_TEAM_COUNT: usize = 20;

pub async fn create_new_team(
    ArchivedRequest { name }: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    let Some(user_id) = session.user_id().await else {
        return Err(Error::NeedLogin);
    };

    let user_team_query = db.query(UserToTeamDocQuery { user_id: &user_id }).await?;
    if user_team_query.len() > MAX_TEAM_COUNT {
        return Err(Error::TooManyTeams);
    }

    let team_id = randum::rand();

    db.transact((
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
    ))
    .await?;

    Ok(Response {})
}
