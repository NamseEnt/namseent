use crate::*;
use database::schema::*;
use luda_rpc::team::get_my_teams::*;

pub async fn get_my_teams(
    ArchivedRequest {}: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
