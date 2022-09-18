use super::*;
use crate::{file::picker::File, namui::skia::canvas_kit, Image};
use dashmap::DashMap;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use url::Url;
use wasm_bindgen_futures::spawn_local;

struct ImageSystem {
    image_url_map: DashMap<Url, Arc<Image>>,
    image_file_map: DashMap<File, Arc<Image>>,
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
            image_url_map: DashMap::new(),
            image_file_map: DashMap::new(),
            image_requested_set: Mutex::new(HashSet::new()),
        }
    }
}
pub fn try_load_url(url: &Url) -> Option<Arc<Image>> {
    if let Some(image) = IMAGE_SYSTEM.image_url_map.get(url) {
        return Some(image.clone());
    };

    {
        let mut image_requested_set = IMAGE_SYSTEM.image_requested_set.lock().unwrap();

        if image_requested_set.contains(url) {
            return None;
        }
        image_requested_set.insert(url.clone());
    }

    start_load_url(url);
    None
}
fn start_load_url(url: &Url) {
    let url = url.clone();
    spawn_local(async move {
        let read_url_result: Result<_, Box<dyn std::error::Error>> = match url.scheme() {
            "http" | "https" => crate::network::http::get_bytes(url.as_str())
                .await
                .map_err(|error| error.into())
                .map(|bytes| bytes.as_ref().to_vec()),
            "bundle" => crate::file::bundle::read(&url)
                .await
                .map_err(|error| error.into())
                .map(|bytes| bytes.as_ref().to_vec()),
            _ => Err(format!("unknown scheme: {}", url.scheme()).into()),
        };

        match read_url_result {
            Ok(data) => match new_image_from_u8(&data) {
                Some(image) => {
                    IMAGE_SYSTEM.image_url_map.insert(url, image);
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

pub(crate) fn try_load_file(file: &file::picker::File) -> Option<Arc<Image>> {
    if let Some(image) = IMAGE_SYSTEM.image_file_map.get(file) {
        return Some(image.clone());
    };

    start_load_file(file.clone());
    None
}

fn start_load_file(file: File) {
    spawn_local(async move {
        let content = file.content().await;
        match new_image_from_u8(&content) {
            Some(image) => {
                IMAGE_SYSTEM.image_file_map.insert(file, image);
            }
            None => {
                crate::log!(
                    "failed to new_image_from_u8 for file: {:?}, {:?}",
                    file,
                    content
                );
            }
        };
    });
}
