use super::Storage;
use namui::Url;

pub trait GithubStorageCharacterImageUrlGet {
    fn get_character_image_url(
        &self,
        character_image_path: &str,
    ) -> Result<Url, GetCharacterImageUrlError>;
}

impl GithubStorageCharacterImageUrlGet for Storage {
    fn get_character_image_url(
        &self,
        character_image_path: &str,
    ) -> Result<Url, GetCharacterImageUrlError> {
        match self
            .get_character_image_path_url_map()
            .get(character_image_path)
        {
            Some(path_url_pair_ref) => return Ok(path_url_pair_ref.value().clone()),
            None => Err(GetCharacterImageUrlError::ImageNotFound),
        }
    }
}

#[derive(Debug)]
pub enum GetCharacterImageUrlError {
    ImageNotFound,
}
