use super::Storage;
use crate::app::{
    github_api::{DownloadError, ReadFileError},
    types::Sequence,
};

impl Storage {
    pub async fn get_sequence(&self, sequence_name: &str) -> Result<Sequence, GetSequenceError> {
        let path = format!("sequence/{}.json", sequence_name);
        let dirent = self.get_github_api_client().read_file(&path).await?;
        let sequence: Sequence = serde_json::from_slice(&dirent.download().await?)?;
        Ok(sequence)
    }
}

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
