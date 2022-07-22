use super::{fetch_background_images::BACKGROUND_IMAGE_PATH_PREFIX, Storage};
use crate::app::github_api::{DownloadError, ReadFileError};
use async_trait::async_trait;
use namui::{image::new_image_from_u8, Image};
use std::sync::Arc;

#[async_trait(?Send)]
pub trait GithubStorageBackgroundImageGet {
    async fn get_background_image(
        &self,
        background_image_path: &str,
    ) -> Result<Arc<Image>, GetBackgroundImageError>;
}

#[async_trait(?Send)]
impl GithubStorageBackgroundImageGet for Storage {
    async fn get_background_image(
        &self,
        background_image_path: &str,
    ) -> Result<Arc<Image>, GetBackgroundImageError> {
        let dirent = self
            .get_github_api_client()
            .read_file(format!("{BACKGROUND_IMAGE_PATH_PREFIX}{background_image_path}").as_str())
            .await?;
        let file = dirent.download().await?;
        let image = new_image_from_u8(file.as_ref());
        match image {
            Some(image) => return Ok(image),
            None => Err(GetBackgroundImageError::FailToMakeImage),
        }
    }
}

#[derive(Debug)]
pub enum GetBackgroundImageError {
    FailToMakeImage,
    ReadFileError(ReadFileError),
    DownloadError(DownloadError),
}
impl From<ReadFileError> for GetBackgroundImageError {
    fn from(error: ReadFileError) -> Self {
        Self::ReadFileError(error)
    }
}
impl From<DownloadError> for GetBackgroundImageError {
    fn from(error: DownloadError) -> Self {
        Self::DownloadError(error)
    }
}
