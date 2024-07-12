

use crate::*;
use database::schema::*;
use luda_rpc::asset::upload_team_sprite_asset::*;

pub async fn upload_team_sprite_asset(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
