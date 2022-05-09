use crate::fetch_get_vec_u8;
use futures::TryFutureExt;
use namui_cfg::namui_cfg;
use std::path::PathBuf;

#[derive(Debug)]
pub enum LoadBundleMetadataError {
    NetworkError(String),
    ParseError(String),
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub async fn load_bundle_metadata() -> Result<Vec<PathBuf>, LoadBundleMetadataError> {
    todo!()
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub async fn load_bundle_metadata() -> Result<Vec<PathBuf>, LoadBundleMetadataError> {
    let file = fetch_get_vec_u8("/bundle_metadata.json")
        .map_err(|message| LoadBundleMetadataError::NetworkError(message.to_string()))
        .await?;
    serde_json::from_slice(&file)
        .map_err(|error| LoadBundleMetadataError::ParseError(error.to_string()))
}
