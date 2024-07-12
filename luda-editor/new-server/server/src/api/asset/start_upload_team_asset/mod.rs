

use crate::*;
use database::schema::*;
use luda_rpc::asset::start_upload_team_asset::*;

pub async fn start_upload_team_asset(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
