use crate::*;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{team::get_my_teams::*, *};

pub async fn get_my_teams(
    &ArchivedRequest {}: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let user_doc = db
        .get(UserDocGet { id: user_id })
        .await?
        .ok_or(Error::UserNotExists)?;

    let team_docs = try_join_all(
        user_doc
            .team_ids
            .iter()
            .map(|&team_id| async move { db.get(TeamDocGet { id: team_id }).await }),
    )
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
