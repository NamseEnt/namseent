use crate::system::InitResult;
use crate::*;
use js_sys::Uint8Array;
use std::{cell::RefCell, sync::Arc};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

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
    async fn loadImage(http_url: &str) -> JsValue; // Uint8Array --> ImageLoaded

    #[wasm_bindgen()]
    fn unloadImage(image_id: u32);
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    thread_local! {
        static LAST_RENDERING_TREE: RefCell<Option<RenderingTree>> = const { RefCell::new(None) };
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

pub(crate) async fn load_image_from_raw(
    bytes: &[u8],
    image_info: Option<ImageInfo>,
    encoded: bool,
) -> Result<Image> {
    todo!()
}

pub(crate) async fn load_image_from_url(url: impl AsRef<str>) -> Result<Image> {
    let buffer: Uint8Array = loadImage(url.as_ref()).await.dyn_into().map_err(|_| {
        anyhow!(
            "Failed to dyn_into from JsValue to Uint8Array: {}",
            url.as_ref()
        )
    })?;

    let ImageLoaded { id, image_info } = postcard::from_bytes(buffer.to_vec().as_ref())?;
    Ok(Image {
        drop_box: Arc::new(DropBox::new(id, move || {
            unloadImage(id);
        })),
        info: image_info,
    })
}
