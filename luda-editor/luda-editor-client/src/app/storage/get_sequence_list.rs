use super::{SequenceName, Storage};
use crate::app::github_api::ReadDirError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GithubStorageSequenceListGet {
    async fn get_sequence_list(&self) -> Result<Vec<SequenceName>, GetSequenceListError>;
}

#[async_trait(?Send)]
impl GithubStorageSequenceListGet for Storage {
    async fn get_sequence_list(&self) -> Result<Vec<SequenceName>, GetSequenceListError> {
        const PATH: &str = "sequence";
        let dirent_list = self.get_github_api_client().read_dir(PATH).await?;
        Ok(dirent_list
            .into_iter()
            .map(|dirent| {
                let name = dirent.name();
                let name = name.trim_end_matches(".json");
                name.to_string()
            })
            .collect())
    }
}

#[derive(Debug)]
pub enum GetSequenceListError {
    ReadDirError(ReadDirError),
}
impl From<ReadDirError> for GetSequenceListError {
    fn from(error: ReadDirError) -> Self {
        Self::ReadDirError(error)
    }
}
