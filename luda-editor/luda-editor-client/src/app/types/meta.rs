use super::*;
use crate::app::storage::GithubStorage;
use async_trait::async_trait;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Clone)]
pub struct Meta {
    pub subtitle_language_minimum_play_duration_map: HashMap<Language, Time>,
    pub subtitle_language_play_duration_per_character_map: HashMap<Language, Time>,
    pub subtitle_specific_text_token_play_duration_map: LinkedHashMap<String, Time>,
    pub subtitle_character_color_map: HashMap<String, Color>,
}
#[allow(dead_code)]
pub async fn save_meta(meta: &Meta, storage: &dyn GithubStorage) -> Result<(), String> {
    unimplemented!()
}

pub async fn get_meta(storage: &dyn GithubStorage) -> Result<Meta, String> {
    let meta = storage
        .get_meta()
        .await
        .map_err(|error| format!("Failed to get meta: {:#?}", error))?;
    Ok(meta)
}

enum MetaLoaderEvent {
    MetaLoaded(Meta),
}

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait MetaLoad {
    async fn load_meta(&self) -> Result<Meta, String>;
}

pub enum MetaContainerEvent {
    MetaReloadRequested,
}
pub struct MetaContainer {
    meta: Mutex<Option<Meta>>,
    loader: Arc<dyn MetaLoad + Sync + Send>,
}

impl MetaContainer {
    pub fn new(last_meta: Option<Meta>, loader: Arc<dyn MetaLoad + Sync + Send>) -> Self {
        Self {
            meta: Mutex::new(last_meta),
            loader,
        }
    }
    pub fn update(&self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<MetaLoaderEvent>() {
            match event {
                MetaLoaderEvent::MetaLoaded(meta) => {
                    self.meta.lock().unwrap().replace(meta.clone());
                }
            }
        } else if let Some(event) = event.downcast_ref::<MetaContainerEvent>() {
            match event {
                MetaContainerEvent::MetaReloadRequested => self.start_reloading(),
            }
        }
    }
    pub fn get_meta(&self) -> Option<Meta> {
        self.meta.lock().unwrap().clone()
    }
    pub fn start_reloading(&self) {
        spawn_local({
            let loader = self.loader.clone();
            async move {
                let meta = loader.load_meta().await.unwrap();
                namui::event::send(MetaLoaderEvent::MetaLoaded(meta));
            }
        });
    }
}
