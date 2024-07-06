use crate::*;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{team::get_my_teams::*, *};

pub async fn get_my_teams(
    ArchivedRequest {}: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    let Some(user_id) = session.user_id().await else {
        return Err(Error::NeedLogin);
    };

    let user_teams = db
        .query(UserToTeamDocQuery {
            user_id: user_id.as_str(),
        })
        .await?;

    let team_docs = try_join_all(user_teams.iter().map(|x| async {
        let team_doc = db
            .get(TeamDocGet {
                id: x.team_id.as_str(),
            })
            .await?
            .ok_or_else(|| Error::InternalServerError {
                err: format!("team not found: {}", x.team_id),
            })?
            .deserialize();
        Ok::<_, Error>(team_doc)
    }))
    .await?;

    Ok(Response {
        teams: team_docs
            .into_iter()
            .map(|x| Team {
                id: x.id,
                name: x.name,
            })
            .collect(),
    })
}
