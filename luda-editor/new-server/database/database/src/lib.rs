mod kv_store;
use anyhow::Result;
use std::path::Path;

pub async fn init(s3_client: aws_sdk_s3::Client, bucket_name: String) -> Result<Database> {
    let sqlite = kv_store::SqliteKvStore::new(s3_client, bucket_name).await?;

    todo!()
}

pub struct Database {}
