use super::Storage;
use crate::app::{
    github_api::{DownloadError, ReadFileError},
    types::Sequence,
};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageSequenceGet {
    async fn get_sequence(&self, sequence_title: &str) -> Result<Sequence, GetSequenceError>;
}

#[async_trait(?Send)]
impl GithubStorageSequenceGet for Storage {
    async fn get_sequence(&self, sequence_title: &str) -> Result<Sequence, GetSequenceError> {
        let path = format!("sequence/{}.json", sequence_title);
        let dirent = self.get_github_api_client().read_file(&path).await?;
        let sequence: Sequence = serde_json::from_slice(&dirent.download().await?)?;
        Ok(sequence)
    }
}

#[derive(Debug)]
pub enum GetSequenceError {
    ReadFileError(ReadFileError),
    DownloadError(DownloadError),
    JsonParseError(serde_json::Error),
}
impl From<ReadFileError> for GetSequenceError {
    fn from(error: ReadFileError) -> Self {
        Self::ReadFileError(error)
    }
}
impl From<DownloadError> for GetSequenceError {
    fn from(error: DownloadError) -> Self {
        Self::DownloadError(error)
    }
}
impl From<serde_json::Error> for GetSequenceError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
