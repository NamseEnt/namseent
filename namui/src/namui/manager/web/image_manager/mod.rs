use crate::{
    fetch_get_vec_u8,
    namui::{
        self,
        skia::{Image, *},
    },
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
mod image_decoder;
use image_decoder::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn decodeImageFromUrl(url: &str) -> Result<JsValue, JsValue>;
}

pub struct ImageManager {
    image_map: DashMap<String, Arc<Image>>,
    image_requested_set: Mutex<HashSet<String>>,
    surface: Arc<Surface>,
}

impl ImageManager {
    pub(crate) fn new(surface: Arc<Surface>) -> Arc<Self> {
        Arc::new(Self {
            image_map: DashMap::new(),
            image_requested_set: Mutex::new(HashSet::new()),
            surface,
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
    async fn img1(data: &[u8], url: String, surface: Arc<Surface>, image_manager: Arc<Self>) {
        match decode_image(&data).await {
            Ok(image) => {
                crate::log!("image loaded: {}", url);
                let array = js_sys::Array::new();
                array.push(&image);
                web_sys::console::log(&array);
                let width = js_sys::Reflect::get(&image, &"displayWidth".into())
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let height = js_sys::Reflect::get(&image, &"displayHeight".into())
                    .unwrap()
                    .as_f64()
                    .unwrap();

                match surface.make_image_from_texture_source(&image, Some(width), Some(height)) {
                    Some(image) => {
                        crate::log!("image created: {}, {:?}", url, image);
                        image_manager.image_map.insert(url, Arc::new(image));
                    }
                    None => {
                        crate::log_error!("Failed to make image");
                    }
                }
            }
            Err(error) => {
                crate::log_error!("Failed to decode image: {}", error);
            }
        }
    }
    async fn img2(data: &[u8], url: String, surface: Arc<Surface>, image_manager: Arc<Self>) {
        let image = canvas_kit().MakeImageFromEncoded(data).unwrap();
        let image = Image::from(image);
        crate::log!("image created: {}", url);
        image_manager.image_map.insert(url, Arc::new(image));
    }
    async fn img3(data: &[u8], url: String, surface: Arc<Surface>, image_manager: Arc<Self>) {
        match decode_image(&data).await {
            Ok(image) => {
                crate::log!("image loaded: {}", url);
                let array = js_sys::Array::new();
                array.push(&image);
                web_sys::console::log(&array);
                let width = js_sys::Reflect::get(&image, &"displayWidth".into())
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let height = js_sys::Reflect::get(&image, &"displayHeight".into())
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let partial_image_info = js_sys::Object::new();
                js_sys::Reflect::set(
                    &partial_image_info,
                    &"alphaType".into(),
                    &AlphaType::Premul.into_canvas_kit().into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &partial_image_info,
                    &"colorType".into(),
                    &ColorType::Rgba8888.into_canvas_kit().into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &partial_image_info,
                    &"colorSpace".into(),
                    &ColorSpace::Srgb.into_canvas_kit().into(),
                )
                .unwrap();
                js_sys::Reflect::set(&partial_image_info, &"width".into(), &width.into()).unwrap();
                js_sys::Reflect::set(&partial_image_info, &"height".into(), &height.into())
                    .unwrap();
                let image =
                    canvas_kit().MakeLazyImageFromTextureSource(image, partial_image_info.into());

                let image = Image::from(image);
                crate::log!("image created: {}", url);
                image_manager.image_map.insert(url, Arc::new(image));
            }
            Err(error) => {
                crate::log_error!("Failed to decode image: {}", error);
            }
        }
    }
    async fn img4(data: &[u8], url: String, surface: Arc<Surface>, image_manager: Arc<Self>) {
        let image = decodeImageFromUrl(&url).await;
        if image.is_err() {
            crate::log_error!("Failed to decode image: {:?}", image.err().unwrap());
            return;
        }
        let image = image.unwrap();
        crate::log!("image loaded: {}", url);
        let array = js_sys::Array::new();
        array.push(&image);
        web_sys::console::log(&array);
        let width = js_sys::Reflect::get(&image, &"width".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let height = js_sys::Reflect::get(&image, &"height".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let partial_image_info = js_sys::Object::new();
        js_sys::Reflect::set(
            &partial_image_info,
            &"alphaType".into(),
            &AlphaType::Premul.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &partial_image_info,
            &"colorType".into(),
            &ColorType::Rgba8888.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &partial_image_info,
            &"colorSpace".into(),
            &ColorSpace::Srgb.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(&partial_image_info, &"width".into(), &width.into()).unwrap();
        js_sys::Reflect::set(&partial_image_info, &"height".into(), &height.into()).unwrap();
        let image = canvas_kit().MakeLazyImageFromTextureSource(image, partial_image_info.into());

        let image = Image::from(image);
        crate::log!("image created: {}", url);
        image_manager.image_map.insert(url, Arc::new(image));
    }
    async fn img5(data: &[u8], url: String, surface: Arc<Surface>, image_manager: Arc<Self>) {
        let image = decodeImageFromUrl(&url).await;
        if image.is_err() {
            crate::log_error!("Failed to decode image: {:?}", image.err().unwrap());
            return;
        }
        let image = image.unwrap();
        crate::log!("image loaded: {}", url);
        let array = js_sys::Array::new();
        array.push(&image);
        web_sys::console::log(&array);
        let width = js_sys::Reflect::get(&image, &"width".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let height = js_sys::Reflect::get(&image, &"height".into())
            .unwrap()
            .as_f64()
            .unwrap();
        let partial_image_info = js_sys::Object::new();
        js_sys::Reflect::set(
            &partial_image_info,
            &"alphaType".into(),
            &AlphaType::Premul.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &partial_image_info,
            &"colorType".into(),
            &ColorType::Rgba8888.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &partial_image_info,
            &"colorSpace".into(),
            &ColorSpace::Srgb.into_canvas_kit().into(),
        )
        .unwrap();
        js_sys::Reflect::set(&partial_image_info, &"width".into(), &width.into()).unwrap();
        js_sys::Reflect::set(&partial_image_info, &"height".into(), &height.into()).unwrap();
        let image = canvas_kit().MakeLazyImageFromTextureSource(image, partial_image_info.into());

        let array = js_sys::Array::new();
        array.push(&image);
        web_sys::console::log(&array);

        let image = Image::from(image);
        crate::log!("image created: {}", url);
        image_manager.image_map.insert(url, Arc::new(image));
    }
    pub fn start_load(self: Arc<Self>, url: &String) {
        let url = url.clone();
        let surface = self.surface.clone();
        let closure = Closure::wrap(Box::new(move || {
            let url = url.clone();
            let surface = surface.clone();
            let image_manager = self.clone();
            spawn_local(async move {
                match fetch_get_vec_u8(&url).await {
                    Ok(data) => Self::img5(&data, url, surface, image_manager).await,
                    Err(error) => {
                        namui::log_error(format!(
                            "ImageManager::start_load: failed to load image: {}, {}",
                            url, error
                        ));
                    }
                }
            })
        }) as Box<dyn FnMut()>);
        crate::window()
            .request_idle_callback(closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        // let url = url.clone();
        // let surface = self.surface.clone();
        // spawn_local(async move {
        //     match fetch_get_vec_u8(&url).await {
        //         Ok(data) => match decode_image(&data).await {
        //             Ok(image) => {
        //                 crate::log!("image loaded: {}", url);
        //                 let width = js_sys::Reflect::get(&image, &"displayWidth".into())
        //                     .unwrap()
        //                     .as_f64()
        //                     .unwrap();
        //                 let height = js_sys::Reflect::get(&image, &"displayHeight".into())
        //                     .unwrap()
        //                     .as_f64()
        //                     .unwrap();

        //                 match surface.make_image_from_texture_source(
        //                     image,
        //                     Some(width),
        //                     Some(height),
        //                 ) {
        //                     Some(image) => {
        //                         self.image_map.insert(url, Arc::new(image));
        //                     }
        //                     None => {
        //                         crate::log_error!("Failed to make image");
        //                     }
        //                 }
        //             }
        //             Err(error) => {
        //                 crate::log_error!("Failed to decode image: {}", error);
        //             }
        //         },
        //         Err(error) => {
        //             namui::log_error(format!(
        //                 "ImageManager::start_load: failed to load image: {}, {}",
        //                 url, error
        //             ));
        //         }
        //     }
        // });
    }
}
