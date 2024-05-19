use crate::system::InitResult;
use crate::*;
use js_sys::Uint8Array;
use std::cell::RefCell;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn requestDraw(rendering_tree: Uint8Array);

    #[wasm_bindgen(catch)]
    async fn loadTypeface(typeface_name: &str, buffer: Uint8Array) -> Result<(), JsValue>;

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

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    thread_local! {
        static LAST_RENDERING_TREE: RefCell<Option<RenderingTree>> = RefCell::new(None);
    }

    LAST_RENDERING_TREE.with_borrow_mut(|last_rendering_tree| {
        if let Some(last_rendering_tree) = last_rendering_tree {
            if last_rendering_tree == &rendering_tree {
                return;
            }
        }

        let buffer = Uint8Array::from(rendering_tree.to_postcard_vec().as_ref());

        *last_rendering_tree = Some(rendering_tree);

        requestDraw(buffer);
    })
}

pub(crate) async fn load_typeface(typeface_name: &str, bytes: &[u8]) -> Result<()> {
    let buffer = Uint8Array::from(bytes);
    loadTypeface(typeface_name, buffer)
        .await
        .map_err(|_| anyhow!("Failed to load typeface."))?;
    Ok(())
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

pub(crate) async fn load_image_from_encoded(
    image_source: &ImageSource,
    bytes: &[u8],
) -> Result<ImageHandle> {
    todo!()
}

pub(crate) async fn load_image_from_raw(
    image_info: ImageInfo,
    bytes: &mut [u8],
) -> Result<ImageHandle> {
    todo!()
}

pub(crate) async fn load_image_from_url(url: impl AsRef<str>) -> Result<ImageHandle> {
    todo!()
}

// pub(crate) async fn encode_loaded_image_to_png(image: &Image) -> Vec<u8> {
//     let bytes: Uint8Array = encodeLoadedImageToPng(image.to_postcard_vec())
//         .await
//         .unwrap()
//         .into();

//     bytes.to_vec()
// }
