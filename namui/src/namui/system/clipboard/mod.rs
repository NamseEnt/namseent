use crate::Image;
use js_sys::{Array, ArrayBuffer, Promise, Reflect, Uint8Array};
use std::sync::Arc;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::Blob;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "navigator", "clipboard"])]
    fn writeText(text: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["window", "navigator", "clipboard"])]
    fn read() -> Promise;
}

pub async fn write_text(text: impl AsRef<str>) -> Result<(), ()> {
    let text = text.as_ref();
    let promise = writeText(text);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub async fn read_images() -> Result<Vec<Arc<Image>>, ()> {
    let mut outputs = Vec::new();

    for blob in read_image_blobs().await?.into_iter() {
        let image = crate::system::image::blob_to_image(blob).await;
        outputs.push(image);
    }

    Ok(outputs)
}

pub async fn read_image_buffers() -> Result<Vec<Vec<u8>>, ()> {
    let mut outputs = Vec::new();

    for blob in read_image_blobs().await?.into_iter() {
        let array_buffer: ArrayBuffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
            .await
            .map_err(|_| ())?
            .into();

        let uint8array = Uint8Array::new(&array_buffer);
        outputs.push(uint8array.to_vec());
    }

    Ok(outputs)
}

async fn read_image_blobs() -> Result<Vec<Blob>, ()> {
    let mut outputs = Vec::new();

    let promise = read();
    let items: Array = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| ())?
        .into();

    for item in items.iter() {
        let types: Array = Reflect::get(&item, &"types".into()).unwrap().into();
        let is_png = types.includes(&"image/png".into(), 0);
        if is_png {
            let blob_promise: Promise = Reflect::get(&item, &"getType".into())
                .unwrap()
                .dyn_into::<js_sys::Function>()
                .unwrap()
                .call1(&item, &"image/png".into())
                .unwrap()
                .into();

            let blob: Blob = wasm_bindgen_futures::JsFuture::from(blob_promise)
                .await
                .map_err(|_| ())?
                .into();
            outputs.push(blob);
        }
    }

    Ok(outputs)
}
