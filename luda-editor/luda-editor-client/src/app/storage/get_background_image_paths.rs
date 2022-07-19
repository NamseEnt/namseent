use super::Storage;

pub trait GithubStorageBackgroundImagePathsGet {
    fn get_background_image_paths(&self) -> Vec<String>;
}

impl GithubStorageBackgroundImagePathsGet for Storage {
    fn get_background_image_paths(&self) -> Vec<String> {
        let paths = self
            .get_background_image_path_set()
            .iter()
            .map(|path_url_pair| path_url_pair.key().to_string())
            .collect();
        paths
    }
}
