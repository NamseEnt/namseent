use crate::{
    fetch_get_vec_u8, managers,
    namui::{self, skia::Image},
    CANVAS_KIT,
};
use dashmap::DashMap;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use url::Url;
use wasm_bindgen_futures::spawn_local;

pub struct ImageManager {
    image_map: DashMap<Url, Arc<Image>>,
    image_requested_set: Mutex<HashSet<Url>>,
}

impl ImageManager {
    pub fn new() -> Self {
        Self {
            image_map: DashMap::new(),
            image_requested_set: Mutex::new(HashSet::new()),
        }
    }
    pub fn try_load(&self, url: &Url) -> Option<Arc<Image>> {
        if let Some(image) = self.image_map.get(url) {
            return Some(image.clone());
        };

        {
            let mut image_requested_set = self.image_requested_set.lock().unwrap();

            if image_requested_set.contains(url) {
                return None;
            }
            image_requested_set.insert(url.clone());
        }

        ImageManager::start_load(url);
        None
    }
    fn start_load(url: &Url) {
        let url = url.clone();
        spawn_local(async move {
            let read_url_result = match url.scheme() {
                "http" | "https" => fetch_get_vec_u8(url.as_str())
                    .await
                    .map_err(|e| format!("{}", e)),
                "bundle" => crate::fs::bundle::read(url.clone())
                    .await
                    .map_err(|e| format!("{:?}", e)),
                _ => Err(format!("unknown scheme: {}", url.scheme())),
            };

            match read_url_result {
                Ok(data) => match CANVAS_KIT.get().unwrap().MakeImageFromEncoded(&data) {
                    Some(canvas_kit_image) => {
                        let image = Image::new(canvas_kit_image);
                        managers()
                            .image_manager
                            .image_map
                            .insert(url, Arc::new(image));
                    }
                    None => {
                        crate::log!("failed to MakeImageFromEncoded: {}, {:?}", url, data);
                    }
                },
                Err(error) => {
                    crate::log!(
                        "ImageManager::start_load: failed to load image: {}, {}",
                        url,
                        error
                    );
                }
            }
        });
    }
}
