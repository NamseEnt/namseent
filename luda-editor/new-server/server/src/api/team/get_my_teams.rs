use crate::*;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::{team::get_my_teams::*, *};

pub async fn get_my_teams(
    Request {}: Request,
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
    .flatten();

    Ok(Response {
        teams: team_docs
            .map(|x| Team {
                id: x.id,
                name: x.name,
            })
            .collect(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_team_serialized() {
        let response = Response {
            teams: vec![
                Team {
                    id: 1,
                    name: "123".to_string(),
                },
                Team {
                    id: 2,
                    name: "1234".to_string(),
                },
            ],
        };

        let bytes = serializer::serialize(&response).unwrap();

        let response2: Response = serializer::deserialize(&bytes).unwrap();

        assert_eq!(response2.teams.len(), 2);
        assert_eq!(response2.teams[0].id, 1);
        assert_eq!(response2.teams[0].name, "123");
        assert_eq!(response2.teams[1].id, 2);
        assert_eq!(response2.teams[1].name, "1234");
    }
}
