

use crate::*;
use database::schema::*;
use luda_rpc::asset::request_upload_team_asset::*;

pub async fn request_upload_team_asset(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
