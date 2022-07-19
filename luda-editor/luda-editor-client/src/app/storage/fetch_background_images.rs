use super::Storage;
use crate::app::github_api::{Dirent, ReadDirError};
use async_recursion::async_recursion;
use dashmap::DashSet;

pub const BACKGROUND_IMAGE_PATH_PREFIX: &str = "backgroundImages";

impl Storage {
    pub async fn fetch_background_images(&self) -> Result<(), FetchBackgroundImagesError> {
        let path_set = self.get_background_image_path_set();
        match self
            .collect_background_image_url_recursively(BACKGROUND_IMAGE_PATH_PREFIX, path_set)
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => match error {
                ReadDirError::DirNotFound => Ok(()),
                _ => Err(FetchBackgroundImagesError::ReadDirError(error)),
            },
        }
    }

    #[async_recursion(?Send)]
    async fn collect_background_image_url_recursively(
        &self,
        path: &str,
        path_set: &DashSet<String>,
    ) -> Result<(), ReadDirError> {
        let dirent_list = self.get_github_api_client().read_dir(path).await?;
        for dirent in dirent_list {
            match dirent {
                Dirent::File { path, .. } => {
                    let path = path
                        .trim_start_matches(BACKGROUND_IMAGE_PATH_PREFIX)
                        .to_string();
                    path_set.insert(path);
                }
                Dirent::Dir { path, .. } => {
                    self.collect_background_image_url_recursively(path.as_str(), path_set)
                        .await?
                }
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum FetchBackgroundImagesError {
    ReadDirError(ReadDirError),
}
impl From<ReadDirError> for FetchBackgroundImagesError {
    fn from(error: ReadDirError) -> Self {
        Self::ReadDirError(error)
    }
}
