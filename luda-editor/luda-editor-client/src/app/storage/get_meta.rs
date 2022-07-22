use super::Storage;
use crate::app::{
    github_api::{DownloadError, ReadFileError},
    types::{Meta, MetaLoad},
};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageMetaGet {
    async fn get_meta(&self) -> Result<Meta, GetMetaError>;
}

#[async_trait(?Send)]
impl GithubStorageMetaGet for Storage {
    async fn get_meta(&self) -> Result<Meta, GetMetaError> {
        const PATH: &str = "meta.json";
        let dirent = self.get_github_api_client().read_file(PATH).await?;
        let meta: Meta = serde_json::from_slice(&dirent.download().await?)?;
        Ok(meta)
    }
}

#[derive(Debug)]
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

#[async_trait(?Send)]
impl MetaLoad for Storage {
    async fn load_meta(&self) -> Result<Meta, String> {
        self.get_meta()
            .await
            .map_err(|error| format!("fail to load meta {:#?}", error))
    }
}
