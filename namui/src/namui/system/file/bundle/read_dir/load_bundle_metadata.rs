use crate::file::electron;
use futures::TryFutureExt;
use namui_cfg::namui_cfg;
use std::path::PathBuf;

#[derive(Debug)]
pub enum LoadBundleMetadataError {
    #[allow(dead_code)]
    NetworkError(String),
    ParseError(String),
    FileNotFound(String),
    Other(String),
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub async fn load_bundle_metadata() -> Result<Vec<PathBuf>, LoadBundleMetadataError> {
    let file: Vec<u8> = electron::read_vec_u8("/bundle_metadata.json")
        .map_err(|error| error.into())
        .await?;
    serde_json::from_slice(&file)
        .map_err(|error| LoadBundleMetadataError::ParseError(error.to_string()))
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub async fn load_bundle_metadata() -> Result<Vec<PathBuf>, LoadBundleMetadataError> {
    let file: Vec<u8> = crate::network::fetch_get_vec_u8("/bundle_metadata.json")
        .map_err(|message| LoadBundleMetadataError::NetworkError(message.to_string()))
        .await?;
    serde_json::from_slice(&file)
        .map_err(|error| LoadBundleMetadataError::ParseError(error.to_string()))
}

impl Into<LoadBundleMetadataError> for electron::ReadVecU8Error {
    fn into(self) -> LoadBundleMetadataError {
        match self {
            electron::ReadVecU8Error::FileNotFound(message) => {
                LoadBundleMetadataError::FileNotFound(message)
            }
            electron::ReadVecU8Error::Other(message) => LoadBundleMetadataError::Other(message),
        }
    }
}
