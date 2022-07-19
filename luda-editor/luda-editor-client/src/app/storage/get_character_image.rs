use super::{fetch_character_images::CHARACTER_IMAGE_PATH_PREFIX, Storage};
use crate::app::github_api::{DownloadError, ReadFileError};
use async_trait::async_trait;
use namui::{image::new_image_from_u8, Image};
use std::sync::Arc;

#[async_trait(?Send)]
pub trait GithubStorageCharacterImageGet {
    async fn get_character_image(
        &self,
        character_image_path: &str,
    ) -> Result<Arc<Image>, GetCharacterImageError>;
}

#[async_trait(?Send)]
impl GithubStorageCharacterImageGet for Storage {
    async fn get_character_image(
        &self,
        character_image_path: &str,
    ) -> Result<Arc<Image>, GetCharacterImageError> {
        let dirent = self
            .get_github_api_client()
            .read_file(format!("{CHARACTER_IMAGE_PATH_PREFIX}{character_image_path}").as_str())
            .await?;
        let file = dirent.download().await?;
        let image = new_image_from_u8(file.as_ref());
        match image {
            Some(image) => return Ok(image),
            None => Err(GetCharacterImageError::FailToMakeImage),
        }
    }
}

#[derive(Debug)]
pub enum GetCharacterImageError {
    FailToMakeImage,
    ReadFileError(ReadFileError),
    DownloadError(DownloadError),
}
impl From<ReadFileError> for GetCharacterImageError {
    fn from(error: ReadFileError) -> Self {
        Self::ReadFileError(error)
    }
}
impl From<DownloadError> for GetCharacterImageError {
    fn from(error: DownloadError) -> Self {
        Self::DownloadError(error)
    }
}
