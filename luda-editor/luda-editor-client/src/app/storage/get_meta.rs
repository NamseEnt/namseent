use super::Storage;
use crate::app::{
    github_api::{DownloadError, ReadFileError},
    types::Meta,
};

impl Storage {
    pub async fn get_meta(&self) -> Result<Meta, GetMetaError> {
        const PATH: &str = "meta.json";
        let dirent = self.get_github_api_client().read_file(PATH).await?;
        let meta: Meta = serde_json::from_slice(&dirent.download().await?)?;
        Ok(meta)
    }
}

pub enum GetMetaError {
    ReadFileError(ReadFileError),
    DownloadError(DownloadError),
    JsonParseError(serde_json::Error),
}
impl From<ReadFileError> for GetMetaError {
    fn from(error: ReadFileError) -> Self {
        Self::ReadFileError(error)
    }
}
impl From<DownloadError> for GetMetaError {
    fn from(error: DownloadError) -> Self {
        Self::DownloadError(error)
    }
}
impl From<serde_json::Error> for GetMetaError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
