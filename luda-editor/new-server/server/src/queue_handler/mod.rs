use aws_config::BehaviorVersion;
use database::{
    schema::{AssetTeamDocGet, TeamAssetTotalBytesDocUpdate},
    Database,
};
use luda_rpc::asset_s3_put_key;
use sqs_message::*;

pub async fn run(db: Database) {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let queue_url = std::env::var("QUEUE_URL").unwrap();
    let client = Client::new(&sdk_config, queue_url);
    loop {
        if let Err(err) = client
            .recv(|type_id, bytes| async move {
                match SqsMessageType::try_from(type_id)? {
                    SqsMessageType::AudioTranscodingOk => {
                        let message = sqs_message::access_message::<AudioTranscodingOk>(&bytes)?;

                        crate::s3()
                            .delete_object()
                            .bucket(crate::s3::asset_bucket_name())
                            .key(asset_s3_put_key(
                                &message.asset_id,
                                luda_rpc::AssetKind::Audio,
                            ))
                            .send()
                            .await?;

                        let asset_team_doc = db
                            .get(AssetTeamDocGet {
                                asset_id: &message.asset_id,
                            })
                            .await?
                            .ok_or_else(|| anyhow::anyhow!("No asset team doc"))?;

                        rkyv를 최신버전으로 업데이트하자.

                        db.transact(TeamAssetTotalBytesDocUpdate {
                            team_id: &asset_team_doc.team_id,
                            want_update: |doc| database::WantUpdate::Yes,
                            update: |doc| {
                                doc.used_bytes -= message.before_size;
                                doc.used_bytes += message.after_size;
                            },
                        })
                        .await?
                        .unwrap();
                    }
                    SqsMessageType::AudioTranscodingError => {
                        let message = sqs_message::access_message::<AudioTranscodingError>(&bytes)?;

                        // send message to user
                    }
                }

                Ok(())
            })
            .await
        {
            eprintln!("Error on queue_handler: {:?}", err);
        }
    }
}
