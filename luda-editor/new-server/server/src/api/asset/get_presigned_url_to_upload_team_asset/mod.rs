

use crate::*;
use database::schema::*;
use luda_rpc::asset::get_presigned_url_to_upload_team_asset::*;

pub async fn get_presigned_url_to_upload_team_asset(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
