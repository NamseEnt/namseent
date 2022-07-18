use super::Storage;

pub trait GithubStorageCharacterImagePathsGet {
    fn get_character_image_paths(&self) -> Vec<String>;
}

impl GithubStorageCharacterImagePathsGet for Storage {
    fn get_character_image_paths(&self) -> Vec<String> {
        let paths = self
            .get_character_image_path_url_map()
            .iter()
            .map(|path_url_pair| path_url_pair.key().to_string())
            .collect();
        paths
    }
}
