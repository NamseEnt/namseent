use super::Storage;
use crate::app::github_api::{DownloadError, ReadFileError};

impl Storage {
    pub async fn get_sequence_titles(&self) -> Result<Vec<SequenceName>, GetSequenceIndexError> {
        const PATH: &str = "sequence_index.json";
        let dirent = self.get_github_api_client().read_file(PATH).await?;
        let sequence_titles = serde_json::from_slice(&dirent.download().await?)?;
        Ok(sequence_titles)
    }
}

type SequenceName = String;

#[derive(Debug)]
pub enum GetSequenceIndexError {
    ReadFileError(ReadFileError),
    JsonParseError(serde_json::Error),
    DownloadError(DownloadError),
}
impl From<ReadFileError> for GetSequenceIndexError {
    fn from(error: ReadFileError) -> Self {
        Self::ReadFileError(error)
    }
}
impl From<DownloadError> for GetSequenceIndexError {
    fn from(error: DownloadError) -> Self {
        Self::DownloadError(error)
    }
}
impl From<serde_json::Error> for GetSequenceIndexError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
