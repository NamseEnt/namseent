use crate::*;
use api::team::is_team_member;
use aws_sdk_s3::presigning::PresigningConfig;
use database::{schema::*, DeserializeInfallible, WantUpdate};
use luda_rpc::asset::reserve_team_asset_upload::*;
use randum::rand;

pub async fn reserve_team_asset_upload(
    ArchivedRequest {
        team_id,
        asset_name,
        byte_size,
        asset_kind,
    }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;
    if !is_team_member(db, team_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    let asset_id = rand();

    db.transact((
        TeamAssetTotalBytesDocUpdate {
            team_id,
            want_update: |doc| {
                if doc.limit_bytes < doc.used_bytes + byte_size {
                    return WantUpdate::Abort;
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.used_bytes += byte_size;
            },
        },
        TeamAssetDocPut {
            team_id,
            asset_id: &asset_id,
            ttl: None,
        },
        AssetDocPut {
            id: &asset_id,
            name: asset_name,
            shared: false,
            asset_kind: &asset_kind.deserialize(),
            byte_size: *byte_size,
            ttl: None,
        },
    ))
    .await?;

    let presigned = s3::s3()
        .put_object()
        .bucket(s3::bucket_name())
        .key(s3::asset_key(&asset_id))
        .content_length(*byte_size as i64)
        .presigned(PresigningConfig::expires_in(
            std::time::Duration::from_secs(180),
        )?)
        .await?;

    Ok(Response {
        asset_id,
        presigned_put_uri: presigned.uri().to_string(),
        headers: presigned
            .headers()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect(),
    })
}
