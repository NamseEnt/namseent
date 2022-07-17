use super::{
    get_sequence_lock_state::{GetSequenceLockStateError, SequenceLockState},
    types::LockInfo,
    Storage,
};
use crate::app::github_api::WriteFileError;
use chrono::{DateTime, FixedOffset};

impl Storage {
    pub async fn lock_sequence(&self, sequence_name: &str) -> Result<ExpiredAt, LockSequenceError> {
        let lock_state = self.get_sequence_lock_state(sequence_name).await?;
        if let SequenceLockState::LockedByOther = lock_state {
            return Err(LockSequenceError::LockedByOther);
        }

        let path = format!("sequence/{}.json", sequence_name);
        let new_lock_info = LockInfo::lock_now(self.get_client_id().clone());
        self.get_github_api_client()
            .write_file(path.as_str(), serde_json::to_string(&new_lock_info)?)
            .await?;
        Ok(new_lock_info.get_expired_at().clone())
    }
}

type ExpiredAt = DateTime<FixedOffset>;

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
