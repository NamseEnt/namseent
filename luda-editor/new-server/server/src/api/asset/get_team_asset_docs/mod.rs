use crate::*;
use api::team::IsTeamMember;
use database::schema::*;
use futures::future::try_join_all;
use luda_rpc::asset::get_team_asset_docs::*;

pub async fn get_team_asset_docs(
    ArchivedRequest { team_id }: ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    let team_doc = db
        .get(TeamDocGet { id: team_id })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    let asset_docs = try_join_all(
        team_doc
            .asset_ids
            .iter()
            .map(|&asset_id| async move { db.get(AssetDocGet { id: asset_id }).await }),
    )
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize())
    .collect();

    Ok(Response { asset_docs })
}
