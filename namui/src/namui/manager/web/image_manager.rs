use crate::{
    fetch_get_vec_u8,
    namui::{self, skia::Image},
    CANVAS_KIT,
};
use dashmap::DashMap;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use wasm_bindgen_futures::spawn_local;

pub struct ImageManager {
    image_map: DashMap<String, Arc<Image>>,
    image_requested_set: Mutex<HashSet<String>>,
}

impl ImageManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            image_map: DashMap::new(),
            image_requested_set: Mutex::new(HashSet::new()),
        })
    }
    pub fn try_load(self: Arc<Self>, url: &String) -> Option<Arc<Image>> {
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

        self.start_load(url);
        None
    }
    fn start_load(self: Arc<Self>, url: &String) {
        let url = url.clone();
        spawn_local(async move {
            match fetch_get_vec_u8(&url).await {
                Ok(data) => match CANVAS_KIT.get().unwrap().MakeImageFromEncoded(&data) {
                    Some(canvas_kit_image) => {
                        let image = Image::from(canvas_kit_image);
                        self.image_map.insert(url, Arc::new(image));
                    }
                    None => {
                        namui::log(format!(
                            "failed to MakeImageFromEncoded: {}, {:?}",
                            url, data
                        ));
                    }
                },
                Err(error) => {
                    namui::log(format!(
                        "ImageManager::start_load: failed to load image: {}, {}",
                        url, error
                    ));
                }
            }
        });
    }
}
