use super::*;
use crate::{namui::skia::canvas_kit, time::delay, File, Image};
use dashmap::DashMap;
use namui_type::Time;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use url::Url;
use wasm_bindgen::prelude::*;
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
            Ok(data) => match new_image_from_u8(&data).await {
                Ok(image) => {
                    IMAGE_SYSTEM.image_url_map.insert(url, image);
                }
                Err(error) => {
                    crate::log!(
                        "failed to MakeImageFromEncoded: {error}, {}, {:?}",
                        url,
                        data
                    );
                }
            },
            Err(error) => {
                crate::log!(
                    "ImageSystem::start_load: failed to load image: {}, {}",
                    url,
                    error,
                );
            }
        }
    });
}
pub async fn load_url(url: &Url) -> Arc<Image> {
    const OBSERVATION_INTERVAL: Time = Time::Ms(8.);

    loop {
        if let Some(image) = try_load_url(url) {
            return image.clone();
        };

        delay(OBSERVATION_INTERVAL).await;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = createImageBitmap)]
    async fn create_image_bitmap(image: JsValue) -> JsValue;
}

pub async fn new_image_from_u8(data: &[u8]) -> Result<Arc<Image>, Box<dyn std::error::Error>> {
    let u8_array = js_sys::Uint8Array::from(data);

    let u8_array_sequence = {
        let array = js_sys::Array::new();
        array.push(&u8_array);
        array
    };
    let blob = web_sys::Blob::new_with_u8_array_sequence(&u8_array_sequence.into()).unwrap();

    let image = blob_to_image(blob).await;

    Ok(image)
}

pub(crate) async fn blob_to_image(blob: web_sys::Blob) -> Arc<Image> {
    let image_bitmap = create_image_bitmap(blob.into()).await;

    let image = canvas_kit().make_lazy_image_from_texture_source(image_bitmap, None, None);

    Arc::new(image)
}

pub(crate) fn try_load_file(file: &File) -> Option<Arc<Image>> {
    if let Some(image) = IMAGE_SYSTEM.image_file_map.get(file) {
        return Some(image.clone());
    };

    start_load_file(file.clone());
    None
}

fn start_load_file(file: File) {
    spawn_local(async move {
        let content = file.content().await;
        match new_image_from_u8(&content).await {
            Ok(image) => {
                IMAGE_SYSTEM.image_file_map.insert(file, image);
            }
            Err(error) => {
                crate::log!(
                    "failed to new_image_from_u8 for file: {error}, {:?}, {:?}",
                    file,
                    content
                );
            }
        };
    });
}
