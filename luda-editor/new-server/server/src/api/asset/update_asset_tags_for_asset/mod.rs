use crate::*;
use api::team::is_team_member;
use database::{schema::*, DeserializeInfallible, WantUpdate};
use luda_rpc::asset::update_asset_tags_for_asset::*;

pub async fn update_asset_tags_for_asset(
    ArchivedRequest { asset_id, tags }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    let asset_team_doc = db
        .get(AssetTeamDocGet { asset_id })
        .await?
        .ok_or(Error::AssetNotExist)?;

    if !is_team_member(db, &asset_team_doc.team_id, &user_id).await? {
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
