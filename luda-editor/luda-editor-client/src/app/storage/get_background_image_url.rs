use super::Storage;
use namui::Url;

impl Storage {
    pub fn get_background_image_url(
        &self,
        background_image_path: &str,
    ) -> Result<Url, GetBackgroundImageUrlError> {
        match self
            .get_background_image_path_url_map()
            .get(background_image_path)
        {
            Some(path_url_pair_ref) => return Ok(path_url_pair_ref.value().clone()),
            None => Err(GetBackgroundImageUrlError::ImageNotFound),
        }
    }
}

#[derive(Debug)]
pub enum GetBackgroundImageUrlError {
    ImageNotFound,
}
