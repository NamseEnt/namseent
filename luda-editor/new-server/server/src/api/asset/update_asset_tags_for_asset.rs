use crate::*;
use api::team::IsTeamMember;
use database::{schema::*, DeserializeInfallible, WantUpdate};
use luda_rpc::asset::update_asset_tags_for_asset::*;

pub async fn update_asset_tags_for_asset(
    &ArchivedRequest { asset_id, ref tags }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let asset_doc = db
        .get(AssetDocGet { id: asset_id })
        .await?
        .ok_or(Error::AssetNotExist)?;

    let team_doc = db
        .get(TeamDocGet {
            id: asset_doc.team_id,
        })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    db.transact::<()>(AssetDocUpdate {
        id: asset_id,
        want_update: |_| WantUpdate::Yes,
        update: |doc| {
            doc.tags = tags.iter().map(|x| x.deserialize()).collect();
        },
    })
    .await?
    .unwrap();

    Ok(Response {})
}
