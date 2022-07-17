use super::Storage;
use crate::app::github_api::{Dirent, ReadDirError};
use async_recursion::async_recursion;
use dashmap::DashMap;
use namui::Url;

const PATH: &str = "backgroundImages";

impl Storage {
    pub async fn fetch_background_images(&self) -> Result<(), FetchBackgroundImagesError> {
        let path_url_map = self.get_background_image_path_url_map();
        match self
            .collect_background_image_url_recursively(PATH, path_url_map)
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
        path_url_map: &DashMap<String, Url>,
    ) -> Result<(), ReadDirError> {
        let dirent_list = self.get_github_api_client().read_dir(path).await?;
        for dirent in dirent_list {
            match dirent {
                Dirent::File {
                    download_url, path, ..
                } => {
                    let path = path.trim_start_matches(PATH).to_string();
                    let url = Url::parse(download_url.as_str()).unwrap();
                    path_url_map.insert(path, url);
                }
                Dirent::Dir { path, .. } => {
                    self.collect_background_image_url_recursively(path.as_str(), path_url_map)
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
