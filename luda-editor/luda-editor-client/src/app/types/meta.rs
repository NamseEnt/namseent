use std::sync::Mutex;

use super::*;
use async_trait::async_trait;
use linked_hash_map::LinkedHashMap;
use luda_editor_rpc::Socket;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Clone)]
pub struct Meta {
    pub subtitle_language_minimum_play_duration_map: HashMap<Language, Time>,
    pub subtitle_language_play_duration_per_character_map: HashMap<Language, Time>,
    pub subtitle_specific_text_token_play_duration_map: LinkedHashMap<String, Time>,
    pub subtitle_character_color_map: HashMap<String, Color>,
}

pub async fn save_meta(meta: &Meta, socket: &Socket) -> Result<(), String> {
    let result = socket
        .write_file(luda_editor_rpc::write_file::Request {
            dest_path: "meta.json".to_string(),
            file: serde_json::to_vec(meta).unwrap(),
        })
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn get_meta(socket: &Socket) -> Result<Meta, String> {
    let result = socket
        .read_file(luda_editor_rpc::read_file::Request {
            dest_path: "meta.json".to_string(),
        })
        .await;
    match result {
        Ok(file) => {
            let meta: Meta = serde_json::from_slice(&file.file).unwrap();
            Ok(meta)
        }
        Err(err) => Err(err.to_string()),
    }
}

enum MetaLoaderEvent {
    MetaLoaded(Meta),
}

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait MetaLoad {
    async fn load_meta(&self) -> Result<Meta, String>;
}

pub enum MetaContainerEvent {
    MetaReloadRequested,
}
pub struct MetaContainer {
    meta: Mutex<Option<Meta>>,
    loader: Arc<dyn MetaLoad>,
}

impl MetaContainer {
    pub fn new(last_meta: Option<Meta>, loader: Arc<dyn MetaLoad>) -> Self {
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
