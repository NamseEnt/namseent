use super::{
    lock_sequence::{GithubStorageSequenceLock, LockSequenceError},
    Storage,
};
use crate::app::{github_api::WriteFileError, types::Sequence};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageSequencePut: GithubStorageSequenceLock {
    async fn put_sequence(
        &self,
        sequence_name: &str,
        sequence: &Sequence,
    ) -> Result<(), PutSequenceError>;
}

#[async_trait(?Send)]
impl GithubStorageSequencePut for Storage {
    async fn put_sequence(
        &self,
        sequence_name: &str,
        sequence: &Sequence,
    ) -> Result<(), PutSequenceError> {
        self.lock_sequence(sequence_name).await?;
        let path = format!("sequence/{}.json", sequence_name);
        self.get_github_api_client()
            .write_file(path.as_str(), serde_json::to_string(sequence)?)
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum PutSequenceError {
    LockSequenceError(LockSequenceError),
    JsonParseError(serde_json::Error),
    WriteFileError(WriteFileError),
}
impl From<LockSequenceError> for PutSequenceError {
    fn from(error: LockSequenceError) -> Self {
        Self::LockSequenceError(error)
    }
}
impl From<serde_json::Error> for PutSequenceError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
impl From<WriteFileError> for PutSequenceError {
    fn from(error: WriteFileError) -> Self {
        Self::WriteFileError(error)
    }
}
