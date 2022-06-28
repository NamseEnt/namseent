use super::*;
use crate::{namui::skia::canvas_kit, Image};
use dashmap::DashMap;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use url::Url;
use wasm_bindgen_futures::spawn_local;

struct ImageSystem {
    image_map: DashMap<Url, Arc<Image>>,
    image_requested_set: Mutex<HashSet<Url>>,
}

lazy_static::lazy_static! {
    static ref IMAGE_SYSTEM: Arc<ImageSystem> = Arc::new(ImageSystem::new());
}

pub(super) async fn init() -> InitResult {
    lazy_static::initialize(&IMAGE_SYSTEM);
    Ok(())
}

impl ImageSystem {
    fn new() -> Self {
        Self {
            image_map: DashMap::new(),
            image_requested_set: Mutex::new(HashSet::new()),
        }
    }
}
pub fn try_load(url: &Url) -> Option<Arc<Image>> {
    if let Some(image) = IMAGE_SYSTEM.image_map.get(url) {
        return Some(image.clone());
    };

    {
        let mut image_requested_set = IMAGE_SYSTEM.image_requested_set.lock().unwrap();

        if image_requested_set.contains(url) {
            return None;
        }
        image_requested_set.insert(url.clone());
    }

    start_load(url);
    None
}
fn start_load(url: &Url) {
    let url = url.clone();
    spawn_local(async move {
        let read_url_result = match url.scheme() {
            "http" | "https" => crate::system::network::fetch_get_vec_u8(url.as_str())
                .await
                .map_err(|e| format!("{}", e)),
            "bundle" => crate::system::file::bundle::read(url.clone())
                .await
                .map_err(|e| format!("{:?}", e)),
            _ => Err(format!("unknown scheme: {}", url.scheme())),
        };

        match read_url_result {
            Ok(data) => match new_image_from_u8(&data) {
                Some(image) => {
                    IMAGE_SYSTEM.image_map.insert(url, image);
                }
                None => {
                    crate::log!("failed to MakeImageFromEncoded: {}, {:?}", url, data);
                }
            },
            Err(error) => {
                crate::log!(
                    "ImageSystem::start_load: failed to load image: {}, {}",
                    url,
                    error
                );
            }
        }
    });
}
pub fn new_image_from_u8(data: &[u8]) -> Option<Arc<Image>> {
    match canvas_kit().MakeImageFromEncoded(data) {
        Some(canvas_kit_image) => Some(Arc::new(Image::new(canvas_kit_image))),
        None => None,
    }
}
