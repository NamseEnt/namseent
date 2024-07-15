use crate::*;
use database::schema::*;
use luda_rpc::asset::reserve_team_asset_upload::*;

/// Reserving the space is not very strictly controlled with hard transaction.
/// So user may upload more than the limit, I assumed that is not that much of a problem.
pub async fn reserve_team_asset_upload(
    ArchivedRequest {
        team_id,
        asset_name,
        byte_size,
        asset_kind,
    }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    let doc =
        db.get(TeamAssetTotalBytesDocGet { team_id })
            .await?
            .ok_or(Error::InternalServerError {
                err: format!("TeamAssetTotalBytesDoc should exist"),
            })?;

    if doc.limit_bytes < doc.used_bytes + byte_size {
        return Err(Error::NotEnoughSpace);
    }

    todo!();
}
