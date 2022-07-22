use super::{
    get_sequence_lock_state::{
        GetSequenceLockStateError, SequenceLockState, StorageSequenceLockStateGet,
    },
    sequence_name_into_lock_file_path,
    types::LockInfo,
    Storage,
};
use crate::app::github_api::WriteFileError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageSequenceLock: StorageSequenceLockStateGet {
    async fn lock_sequence(&self, sequence_name: &str) -> Result<LockInfo, LockSequenceError>;
}

#[async_trait(?Send)]
impl GithubStorageSequenceLock for Storage {
    async fn lock_sequence(&self, sequence_name: &str) -> Result<LockInfo, LockSequenceError> {
        let lock_state = self.get_sequence_lock_state(sequence_name).await?;
        if let SequenceLockState::LockedByOther = lock_state {
            return Err(LockSequenceError::LockedByOther);
        }

        let lock_path = sequence_name_into_lock_file_path(sequence_name);
        let new_lock_info = LockInfo::lock_now(self.get_client_id().clone());
        self.get_github_api_client()
            .write_file(lock_path.as_str(), serde_json::to_string(&new_lock_info)?)
            .await?;
        Ok(new_lock_info)
    }
}

#[derive(Debug)]
pub enum LockSequenceError {
    LockedByOther,
    GetSequenceLockStateError(GetSequenceLockStateError),
    WriteFileError(WriteFileError),
    SerializeError(serde_json::Error),
}
impl From<GetSequenceLockStateError> for LockSequenceError {
    fn from(error: GetSequenceLockStateError) -> Self {
        Self::GetSequenceLockStateError(error)
    }
}
impl From<WriteFileError> for LockSequenceError {
    fn from(error: WriteFileError) -> Self {
        Self::WriteFileError(error)
    }
}
impl From<serde_json::Error> for LockSequenceError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerializeError(error)
    }
}
