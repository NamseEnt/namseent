use super::{
    get_sequence_lock_state::{
        GetSequenceLockStateError, SequenceLockState, StorageSequenceLockStateGet,
    },
    Storage,
};
use crate::app::github_api::DeleteFileError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageSequenceUnlock: StorageSequenceLockStateGet {
    async fn unlock_sequence(&self, sequence_title: &str) -> Result<(), UnlockSequenceError>;
}

#[async_trait(?Send)]
impl GithubStorageSequenceUnlock for Storage {
    async fn unlock_sequence(&self, sequence_title: &str) -> Result<(), UnlockSequenceError> {
        let lock_state = self.get_sequence_lock_state(sequence_title).await?;
        match lock_state {
            SequenceLockState::LockedByOther => return Err(UnlockSequenceError::LockedByOther),
            SequenceLockState::LockedByMe => {
                let path = format!("lock/{}.lock.json", sequence_title);
                self.get_github_api_client()
                    .delete_file(path.as_str())
                    .await?;
            }
            _ => (),
        };
        Ok(())
    }
}

pub enum UnlockSequenceError {
    LockedByOther,
    GetSequenceLockStateError(GetSequenceLockStateError),
    DeleteFileError(DeleteFileError),
}
impl From<GetSequenceLockStateError> for UnlockSequenceError {
    fn from(error: GetSequenceLockStateError) -> Self {
        Self::GetSequenceLockStateError(error)
    }
}
impl From<DeleteFileError> for UnlockSequenceError {
    fn from(error: DeleteFileError) -> Self {
        Self::DeleteFileError(error)
    }
}
