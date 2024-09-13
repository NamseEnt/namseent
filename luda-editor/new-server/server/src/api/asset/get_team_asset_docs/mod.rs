use crate::*;
use api::team::is_team_member;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::asset::get_team_asset_docs::*;

pub async fn get_team_asset_docs(
    ArchivedRequest { team_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let team_asset_docs = db.query(TeamAssetDocQuery { team_id }).await?;

    let asset_docs = try_join_all(team_asset_docs.into_iter().map(|doc| async move {
        db.get(AssetDocGet {
            id: doc.asset_id.as_str(),
        })
        .await
    }))
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize())
    .collect();

    Ok(Response { asset_docs })
}
