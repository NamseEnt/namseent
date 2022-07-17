use super::Storage;
use crate::app::github_api::ReadDirError;

impl Storage {
    pub async fn get_sequence_list(&self) -> Result<Vec<SequenceName>, GetSequenceListError> {
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

type SequenceName = String;

#[derive(Debug)]
pub enum GetSequenceListError {
    ReadDirError(ReadDirError),
}
impl From<ReadDirError> for GetSequenceListError {
    fn from(error: ReadDirError) -> Self {
        Self::ReadDirError(error)
    }
}
