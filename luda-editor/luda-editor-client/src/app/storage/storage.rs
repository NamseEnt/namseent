use super::{
    fetch_background_images::FetchBackgroundImagesError,
    fetch_character_images::FetchCharacterImagesError,
};
use crate::app::github_api::GithubAPiClient;
use dashmap::DashMap;
use namui::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Storage {
    github_api_client: Arc<GithubAPiClient>,
    client_id: String,
    background_image_path_url_map: DashMap<String, Url>,
    character_image_path_url_map: DashMap<String, Url>,
}
impl Storage {
    pub fn new(github_api_client: Arc<GithubAPiClient>) -> Self {
        let client_id = nanoid();
        Self {
            github_api_client,
            client_id,
            background_image_path_url_map: DashMap::new(),
            character_image_path_url_map: DashMap::new(),
        }
    }

    pub(super) fn get_github_api_client(&self) -> &Arc<GithubAPiClient> {
        &&self.github_api_client
    }

    pub(super) fn get_client_id(&self) -> &String {
        &self.client_id
    }

    pub(super) fn get_background_image_path_url_map(&self) -> &DashMap<String, Url> {
        &self.background_image_path_url_map
    }

    pub(super) fn get_character_image_path_url_map(&self) -> &DashMap<String, Url> {
        &self.character_image_path_url_map
    }

    pub async fn init(&self) -> Result<(), StorageInitError> {
        self.fetch_background_images().await?;
        self.fetch_character_images().await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum StorageInitError {
    FetchBackgroundImagesError(FetchBackgroundImagesError),
    FetchCharacterImagesError(FetchCharacterImagesError),
}
impl From<FetchBackgroundImagesError> for StorageInitError {
    fn from(error: FetchBackgroundImagesError) -> Self {
        Self::FetchBackgroundImagesError(error)
    }
}
impl From<FetchCharacterImagesError> for StorageInitError {
    fn from(error: FetchCharacterImagesError) -> Self {
        Self::FetchCharacterImagesError(error)
    }
}
