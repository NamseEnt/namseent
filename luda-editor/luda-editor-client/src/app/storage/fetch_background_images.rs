use super::Storage;
use crate::app::github_api::{Dirent, ReadDirError};
use async_recursion::async_recursion;
use dashmap::DashSet;
use futures::future::join_all;

pub const BACKGROUND_IMAGE_PATH_PREFIX: &str = "backgroundImages";

impl Storage {
    pub async fn fetch_background_images(&self) -> Result<(), FetchBackgroundImagesError> {
        let path_set = self.get_background_image_path_set();
        match self
            .collect_background_image_url_recursively(
                BACKGROUND_IMAGE_PATH_PREFIX.to_string(),
                path_set,
            )
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
        path: String,
        path_set: &DashSet<String>,
    ) -> Result<(), ReadDirError> {
        let dirent_list = self.get_github_api_client().read_dir(path.as_str()).await?;
        let mut next_collect_future_list = Vec::new();
        for dirent in dirent_list {
            match dirent {
                Dirent::File { path, .. } => {
                    let path = path
                        .trim_start_matches(BACKGROUND_IMAGE_PATH_PREFIX)
                        .to_string();
                    path_set.insert(path);
                }
                Dirent::Dir { path, .. } => next_collect_future_list
                    .push(self.collect_background_image_url_recursively(path, path_set)),
                _ => unimplemented!(),
            }
        }
        let next_collect_result_list = join_all(next_collect_future_list).await;
        throw_error_if_some_result_is_error(next_collect_result_list)?;
        Ok(())
    }
}

fn throw_error_if_some_result_is_error(
    next_collect_result_list: Vec<Result<(), ReadDirError>>,
) -> Result<(), ReadDirError> {
    for next_collect_result in next_collect_result_list {
        match next_collect_result {
            Ok(_) => (),
            Err(error) => return Err(error),
        }
    }
    Ok(())
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
