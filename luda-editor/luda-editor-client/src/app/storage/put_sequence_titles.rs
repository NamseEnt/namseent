use super::Storage;
use crate::app::github_api::WriteFileError;

impl Storage {
    pub async fn put_sequence_titles(
        &self,
        sequence_titles: &Vec<SequenceName>,
    ) -> Result<(), PutSequenceIndexError> {
        const PATH: &str = "sequence_index.json";
        let dirent = self
            .get_github_api_client()
            .write_file(PATH, serde_json::to_string(sequence_titles)?)
            .await?;
        Ok(())
    }
}

type SequenceName = String;

#[derive(Debug)]
pub enum PutSequenceIndexError {
    JsonParseError(serde_json::Error),
    WriteFileError(WriteFileError),
}
impl From<serde_json::Error> for PutSequenceIndexError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
impl From<WriteFileError> for PutSequenceIndexError {
    fn from(error: WriteFileError) -> Self {
        Self::WriteFileError(error)
    }
}
