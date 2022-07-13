use super::{types::LockInfo, Storage};
use crate::app::github_api::{DownloadError, ReadFileError};

impl Storage {
    pub async fn get_sequence_lock_state(
        &self,
        sequence_name: &str,
    ) -> Result<SequenceLockState, GetSequenceLockStateError> {
        let path = format!("lock/{}.lock.json", sequence_name);
        match self.get_github_api_client().read_file(path.as_str()).await {
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
type ClientId = String;

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
