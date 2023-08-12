use js_sys::Uint8Array;
use namui_type::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::ImageBitmap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["globalThis"], catch)]
    async fn loadImageBitmap(url: &str) -> Result<JsValue, JsValue>; // Result<ImageBitmap, String>
    #[wasm_bindgen(js_namespace = ["globalThis"], catch)]
    async fn loadImageBitmap2(url: &str) -> Result<JsValue, JsValue>; // Result<ImageBitmap, String>
}

pub(crate) fn start_load_image(src: &ImageSource) {
    static LOADING_IMAGES: StaticHashSet<ImageSource> = StaticHashSet::new();
    if LOADING_IMAGES.contains(src) {
        return;
    }
    LOADING_IMAGES.insert(src.clone());

    let src = src.clone();
    spawn_local(async move {
        match &src {
            ImageSource::Url { url } => {
                match loadImageBitmap(url.as_str()).await {
                    Ok(image_bitmap) => {
                        let image_bitmap: ImageBitmap = image_bitmap.into();
                        crate::SKIA.get().unwrap().load_image(&src, image_bitmap);
                    }
                    Err(_) => {
                        crate::log!("Failed to load image: {}", url);
                    }
                };
                // match loadImageBitmap2(url.as_str()).await {
                //     Ok(bytes) => {
                //         let bytes: Uint8Array = bytes.into();

                //         crate::SKIA
                //             .get()
                //             .unwrap()
                //             .load_image2(&src, &bytes.to_vec());
                //     }
                //     Err(_) => {
                //         crate::log!("Failed to load image: {}", url);
                //     }
                // };
            }
        }
    });
}
