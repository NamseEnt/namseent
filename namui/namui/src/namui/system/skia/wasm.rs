use crate::system::InitResult;
use crate::*;
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn requestDraw(draw_input: Uint8Array);

    #[wasm_bindgen()]
    async fn loadTypeface(typeface_name: &str, buffer: Uint8Array);

    #[wasm_bindgen()]
    async fn loadImage(http_url: &str) -> JsValue; // Uint8Array --> ImageInfo

    // #[wasm_bindgen(catch)]
    // async fn encodeLoadedImageToPng(
    //     image: Vec<u8>, // Image
    // ) -> Result<
    //     JsValue, // Uint8Array
    //     JsValue,
    // >;
}

static mut LAST_RENDERING_TREE: Option<RenderingTree> = None;

#[wasm_bindgen]
pub fn on_load_image() {
    if let Some(last_rendering_tree) = unsafe { &mut LAST_RENDERING_TREE } {
        let draw_input = DrawInput {
            rendering_tree: last_rendering_tree.clone(),
        };
        let buffer = Uint8Array::from(draw_input.to_postcard_vec().as_ref());

        requestDraw(buffer);
    }
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    if let Some(last_rendering_tree) = unsafe { &mut LAST_RENDERING_TREE } {
        if last_rendering_tree == &rendering_tree {
            return;
        }
    }

    unsafe {
        LAST_RENDERING_TREE = Some(rendering_tree.clone());
    }

    let draw_input = DrawInput { rendering_tree };
    let buffer = Uint8Array::from(draw_input.to_postcard_vec().as_ref());

    requestDraw(buffer);
}

pub(crate) async fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    let buffer = Uint8Array::from(bytes);
    loadTypeface(typeface_name, buffer).await;
}

pub(crate) async fn load_image(image_source: &ImageSource) -> ImageInfo {
    match image_source {
        ImageSource::Url { url } => {
            let bytes: Uint8Array = loadImage(url.as_ref()).await.into();
            ImageInfo::from_postcard_bytes(&bytes.to_vec())
        }
        // assumed that image already loaded so they have image handle.
        ImageSource::ImageHandle { image_handle } => ImageInfo {
            alpha_type: image_handle.alpha_type,
            color_type: image_handle.color_type,
            width: image_handle.width,
            height: image_handle.height,
        },
    }
}

pub(crate) fn load_image_from_encoded(image_source: &ImageSource, bytes: &[u8]) -> ImageInfo {
    todo!()
}

pub(crate) fn load_image_from_raw(image_info: ImageInfo, bytes: &mut [u8]) -> ImageHandle {
    todo!()
}

pub(crate) fn load_image_from_url(image_info: ImageInfo, url: impl AsRef<str>) -> ImageHandle {
    todo!()
}

// pub(crate) async fn encode_loaded_image_to_png(image: &Image) -> Vec<u8> {
//     let bytes: Uint8Array = encodeLoadedImageToPng(image.to_postcard_vec())
//         .await
//         .unwrap()
//         .into();

//     bytes.to_vec()
// }
