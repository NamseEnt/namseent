use super::{
    lock_sequence::{GithubStorageSequenceLock, LockSequenceError},
    Storage,
};
use crate::app::github_api::DeleteFileError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub(super) trait GithubStorageSequenceDelete: GithubStorageSequenceLock {
    async fn delete_sequence(&self, sequence_name: &str) -> Result<(), DeleteSequenceError>;
}

#[async_trait(?Send)]
impl GithubStorageSequenceDelete for Storage {
    async fn delete_sequence(&self, sequence_name: &str) -> Result<(), DeleteSequenceError> {
        self.lock_sequence(sequence_name).await?;
        let path = format!("sequence/{}.json", sequence_name);
        self.get_github_api_client()
            .delete_file(path.as_str())
            .await?;
        Ok(())
    }
}

pub enum DeleteSequenceError {
    LockSequenceError(LockSequenceError),
    DeleteFileError(DeleteFileError),
}
impl From<LockSequenceError> for DeleteSequenceError {
    fn from(error: LockSequenceError) -> Self {
        Self::LockSequenceError(error)
    }
}
impl From<DeleteFileError> for DeleteSequenceError {
    fn from(error: DeleteFileError) -> Self {
        Self::DeleteFileError(error)
    }
}
