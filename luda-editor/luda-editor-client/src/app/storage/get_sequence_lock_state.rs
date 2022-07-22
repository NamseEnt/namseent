use super::{sequence_title_into_lock_file_path, types::LockInfo, Storage};
use crate::app::github_api::{DownloadError, ReadFileError};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageSequenceLockStateGet {
    async fn get_sequence_lock_state(
        &self,
        sequence_title: &str,
    ) -> Result<SequenceLockState, GetSequenceLockStateError>;
}

#[async_trait(?Send)]
impl GithubStorageSequenceLockStateGet for Storage {
    async fn get_sequence_lock_state(
        &self,
        sequence_title: &str,
    ) -> Result<SequenceLockState, GetSequenceLockStateError> {
        let lock_file_path = sequence_title_into_lock_file_path(sequence_title);
        match self
            .get_github_api_client()
            .read_file(lock_file_path.as_str())
            .await
        {
            Ok(dirent) => {
                let lock_info: LockInfo = serde_json::from_slice(&dirent.download().await?)?;
                if lock_info.is_expired() {
                    return Ok(SequenceLockState::Unlocked);
                }
                if lock_info.get_client_id() == self.get_client_id() {
                    return Ok(SequenceLockState::LockedByMe);
                }
                return Ok(SequenceLockState::LockedByOther);
            }
            Err(error) => match error {
                ReadFileError::FileNotFound => {
                    return Ok(SequenceLockState::Unlocked);
                }
                _ => return Err(GetSequenceLockStateError::ReadFileError(error)),
            },
        }
    }
}

pub enum SequenceLockState {
    LockedByOther,
    LockedByMe,
    Unlocked,
}

#[derive(Debug)]
pub enum GetSequenceLockStateError {
    ReadFileError(ReadFileError),
    DownloadError(DownloadError),
    JsonParseError(serde_json::Error),
}
impl From<DownloadError> for GetSequenceLockStateError {
    fn from(error: DownloadError) -> Self {
        Self::DownloadError(error)
    }
}
impl From<serde_json::Error> for GetSequenceLockStateError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
