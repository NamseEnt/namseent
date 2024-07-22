use crate::*;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{team::get_my_teams::*, *};

pub async fn get_my_teams(
    ArchivedRequest {}: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    let user_teams = db
        .query(UserToTeamDocQuery {
            user_id: user_id.as_str(),
        })
        .await?;

    let team_docs = try_join_all(user_teams.iter().map(|x| async {
        db.get(TeamDocGet {
            id: x.team_id.as_str(),
        })
        .await
    }))
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize());

    Ok(Response {
        teams: team_docs
            .map(|x| Team {
                id: x.id,
                name: x.name,
            })
            .collect(),
    })
}
