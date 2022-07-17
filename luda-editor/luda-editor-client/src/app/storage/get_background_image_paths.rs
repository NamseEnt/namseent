use super::Storage;

impl Storage {
    pub fn get_background_image_paths(&self) -> Vec<String> {
        let paths = self
            .get_background_image_path_url_map()
            .iter()
            .map(|path_url_pair| path_url_pair.key().to_string())
            .collect();
        paths
    }
}
