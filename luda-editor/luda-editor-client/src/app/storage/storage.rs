use super::{
    fetch_background_images::FetchBackgroundImagesError,
    fetch_character_images::FetchCharacterImagesError,
    get_background_image::GithubStorageBackgroundImageGet,
    get_background_image_paths::GithubStorageBackgroundImagePathsGet,
    get_character_image::GithubStorageCharacterImageGet,
    get_character_image_paths::GithubStorageCharacterImagePathsGet, get_meta::GithubStorageMetaGet,
    get_sequence::GithubStorageSequenceGet, get_sequence_list::GithubStorageSequenceListGet,
    get_sequence_lock_state::StorageSequenceLockStateGet,
    get_sequence_titles::GithubStorageSequenceTitlesGet, lock_sequence::GithubStorageSequenceLock,
    put_sequence::GithubStorageSequencePut, put_sequence_titles::GithubStorageSequenceTitlesPut,
    unlock_sequence::GithubStorageSequenceUnlock,
};
use crate::app::github_api::GithubAPiClient;
use async_trait::async_trait;
use dashmap::DashSet;
use namui::prelude::*;
use std::{fmt::Debug, sync::Arc};

#[async_trait(?Send)]
pub trait GithubStorage:
    Debug
    + Send
    + Sync
    + GithubStorageMetaGet
    + StorageSequenceLockStateGet
    + GithubStorageSequenceLock
    + GithubStorageSequenceUnlock
    + GithubStorageSequenceGet
    + GithubStorageSequencePut
    + GithubStorageSequenceListGet
    + GithubStorageSequenceTitlesGet
    + GithubStorageSequenceTitlesPut
    + GithubStorageBackgroundImagePathsGet
    + GithubStorageBackgroundImageGet
    + GithubStorageCharacterImageGet
    + GithubStorageCharacterImagePathsGet
{
    async fn init(&self) -> Result<(), StorageInitError>;
}

#[derive(Debug)]
pub struct Storage {
    github_api_client: Arc<GithubAPiClient>,
    client_id: String,
    background_image_path_set: DashSet<String>,
    character_image_path_set: DashSet<String>,
}
impl Storage {
    pub fn new(github_api_client: Arc<GithubAPiClient>) -> Self {
        let client_id = nanoid();
        Self {
            github_api_client,
            client_id,
            background_image_path_set: DashSet::new(),
            character_image_path_set: DashSet::new(),
        }
    }

    pub(super) fn get_github_api_client(&self) -> &Arc<GithubAPiClient> {
        &&self.github_api_client
    }

    pub(super) fn get_client_id(&self) -> &String {
        &self.client_id
    }

    pub(super) fn get_background_image_path_set(&self) -> &DashSet<String> {
        &self.background_image_path_set
    }

    pub(super) fn get_character_image_path_set(&self) -> &DashSet<String> {
        &self.character_image_path_set
    }
}

#[async_trait(?Send)]
impl GithubStorage for Storage {
    async fn init(&self) -> Result<(), StorageInitError> {
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
