use crate::*;
use futures::Future;
use js_sys::*;
use namui_type::*;
use std::pin::Pin;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{Blob, BlobPropertyBag};

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "navigator", "clipboard"])]
    fn writeText(text: &str) -> Promise;
    #[wasm_bindgen(js_namespace = ["window", "navigator", "clipboard"], js_name = "write")]
    fn write_(items: &Array) -> Promise;
    #[wasm_bindgen(js_namespace = ["window", "navigator", "clipboard"], js_name = "read")]
    fn read_() -> Promise;

    #[wasm_bindgen(js_name = "ClipboardItem")]
    type ClipboardItem_;
    #[wasm_bindgen(constructor, js_class = "ClipboardItem")]
    fn new(data: Object) -> ClipboardItem_;

    #[wasm_bindgen(method, structural, js_name = "getType", js_class = "ClipboardItem")]
    fn get_type_(this: &ClipboardItem_, type_: &str) -> Promise;
}

pub async fn write<MimeBytesPairs, Mime, Bytes>(data: MimeBytesPairs) -> Result<()>
where
    MimeBytesPairs: IntoIterator<Item = (Mime, Bytes)>,
    Mime: AsRef<str>,
    Bytes: AsRef<[u8]>,
{
    let clipboard_item_data = js_sys::Object::new();
    for (mime, bytes) in data {
        let uint8_array = {
            let bytes: &[u8] = bytes.as_ref();
            let uint8_array = Uint8Array::new_with_length(bytes.len() as u32);
            uint8_array.copy_from(bytes);
            uint8_array
        };
        let uint8_array_sequence = {
            let array = js_sys::Array::new();
            array.push(&uint8_array.into());
            array
        };
        let blob = {
            let mut option = BlobPropertyBag::new();
            option.type_(mime.as_ref());
            Blob::new_with_u8_array_sequence_and_options(&uint8_array_sequence.into(), &option)
                .unwrap()
        };
        Reflect::set(&clipboard_item_data, &mime.as_ref().into(), &blob.into()).unwrap();
    }

    let clipboard_item = ClipboardItem_::new(clipboard_item_data);
    let clipboard_items = {
        let array = js_sys::Array::new();
        array.push(&clipboard_item.into());
        array
    };
    let promise = write_(&clipboard_items);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            bail!("error: failed to write to clipboard {:#?}", error)
        }
    }
}

pub async fn read() -> Result<Vec<impl ClipboardItem>> {
    let promise = read_();
    let items: Array = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| anyhow!("Fail to read clipboard"))?
        .into();
    Ok(items
        .iter()
        .map(|item| item.dyn_into::<ClipboardItem_>().unwrap())
        .collect())
}

pub trait ClipboardItem {
    fn types(&self) -> Vec<String>;
    fn get_type<Mime>(&self, mime: Mime) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, ()>> + '_>>
    where
        Mime: ToString;
}

impl ClipboardItem for ClipboardItem_ {
    fn types(&self) -> Vec<String> {
        let types: Array = Reflect::get(self, &"types".into()).unwrap().into();
        types
            .iter()
            .map(|type_| type_.as_string().unwrap())
            .collect()
    }

    fn get_type<Mime>(&self, mime: Mime) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, ()>> + '_>>
    where
        Mime: ToString,
    {
        let mime = mime.to_string();
        Box::pin(async move {
            let blob: Blob = {
                let promise = self.get_type_(mime.as_ref());
                wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|_| ())?
                    .into()
            };
            let array_buffer = {
                let promise = blob.array_buffer();
                let array_buffer: ArrayBuffer = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|_| ())?
                    .into();
                Uint8Array::new(&array_buffer)
            };

            Ok(array_buffer.to_vec())
        })
    }
}

pub async fn write_text(text: impl AsRef<str>) -> Result<()> {
    let text = text.as_ref();
    let promise = writeText(text);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("error: failed to write text to clipboard")),
    }
}

pub async fn write_image(image: &Image) -> Result<()> {
    let type_ = "image/png";
    let png_bytes = crate::system::drawer::encode_loaded_image_to_png(image).await;
    let blob_parts = {
        let array = js_sys::Array::new();
        array.push(&Uint8Array::from(png_bytes.as_ref()).into());
        array
    };

    let mut blob_options = web_sys::BlobPropertyBag::new();
    blob_options.type_(type_);
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)
        .map_err(|_| anyhow!("error: failed to create blob"))?;

    let clipboard_item_data = {
        let object = js_sys::Object::new();
        Reflect::set(&object, &type_.into(), &blob.into()).unwrap();
        object
    };
    let clipboard_item = ClipboardItem_::new(clipboard_item_data);
    let clipboard_items = {
        let array = js_sys::Array::new();
        array.push(&clipboard_item.into());
        array
    };
    let promise = write_(&clipboard_items);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("error: failed to write image to clipboard")),
    }
}

// TODO
// pub async fn read_images() -> Result<Vec<Arc<Image>>, ()> {
//     let mut outputs = Vec::new();

//     for blob in read_image_blobs().await?.into_iter() {
//         let image = crate::system::image::blob_to_image(blob).await;
//         outputs.push(image);
//     }

//     Ok(outputs)
// }

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

    let promise = read_();
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
