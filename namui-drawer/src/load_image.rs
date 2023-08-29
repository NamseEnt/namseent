use namui_type::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::ImageBitmap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["globalThis"], catch)]
    async fn loadImageBitmap(
        url: &str,
    ) -> Result<
        JsValue, // ImageBitmap
        JsValue,
    >;
    #[wasm_bindgen(js_namespace = ["globalThis"])]
    fn onLoadImage();
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
                        crate::SKIA
                            .get()
                            .unwrap()
                            .read()
                            .unwrap()
                            .load_image(&src, &image_bitmap);
                        onLoadImage();
                    }
                    Err(_) => {
                        crate::log!("Failed to load image: {}", url);
                    }
                };
            }
        }
    });
}
