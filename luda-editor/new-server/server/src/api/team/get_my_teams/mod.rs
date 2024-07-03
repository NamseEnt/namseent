use crate::*;
use database::schema::*;
use luda_rpc::team::get_my_teams::*;

pub async fn get_my_teams(
    ArchivedRequest {}: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    let Some(user_id) = session.user_id() else {
        return Err(Error::NeedLogin);
    };

    let user_team = db
        .get(UserTeamDocGet {
            user_id: user_id.as_str(),
        })
        .await?;

    todo!()
}
