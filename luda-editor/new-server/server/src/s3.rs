use crate::is_on_aws;
use anyhow::Result;
use aws_config::BehaviorVersion;
use std::sync::OnceLock;

static CLIENT: OnceLock<aws_sdk_s3::Client> = OnceLock::new();

pub fn s3() -> &'static aws_sdk_s3::Client {
    CLIENT.get().unwrap()
}

pub async fn init_s3() -> Result<()> {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_s3::Config::new(&sdk_config)
        .to_builder()
        .force_path_style(!is_on_aws())
        .build();
    let client = aws_sdk_s3::Client::from_conf(config);

    CLIENT
        .set(client)
        .map_err(|_| anyhow::anyhow!("s3 client already initialized"))?;
    Ok(())
}

pub fn database_bucket_name() -> &'static str {
    static DATABASE_BUCKET_NAME: OnceLock<String> = OnceLock::new();
    DATABASE_BUCKET_NAME.get_or_init(|| std::env::var("DATABASE_BUCKET_NAME").unwrap())
}

pub fn asset_bucket_name() -> &'static str {
    static ASSET_BUCKET_NAME: OnceLock<String> = OnceLock::new();
    ASSET_BUCKET_NAME.get_or_init(|| std::env::var("ASSET_BUCKET_NAME").unwrap())
}

pub fn asset_key(asset_id: &str) -> String {
    format!("asset/{}", asset_id)
}
