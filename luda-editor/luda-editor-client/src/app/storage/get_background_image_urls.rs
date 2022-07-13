use super::Storage;
use crate::app::github_api::{Dirent, ReadDirError};
use async_recursion::async_recursion;
use namui::Url;

impl Storage {
    pub async fn get_character_image_urls(&self) -> Result<Vec<Url>, GetCharacterImageUrlsError> {
        const PATH: &str = "characterImages";
        let mut urls: Vec<Url> = Vec::new();
        self.collect_background_image_url_recursively(PATH, &mut urls)
            .await?;
        Ok(urls)
    }

    #[async_recursion(?Send)]
    async fn collect_background_image_url_recursively(
        &self,
        path: &str,
        urls: &mut Vec<Url>,
    ) -> Result<(), ReadDirError> {
        let dirent_list = self.get_github_api_client().read_dir(path).await?;
        for dirent in dirent_list {
            match dirent {
                Dirent::File { download_url, .. } => {
                    urls.push(Url::parse(download_url.as_str()).unwrap())
                }
                Dirent::Dir { path, .. } => {
                    self.collect_background_image_url_recursively(path.as_str(), urls)
                        .await?
                }
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

pub enum GetCharacterImageUrlsError {
    ReadDirError(ReadDirError),
}
impl From<ReadDirError> for GetCharacterImageUrlsError {
    fn from(error: ReadDirError) -> Self {
        Self::ReadDirError(error)
    }
}
