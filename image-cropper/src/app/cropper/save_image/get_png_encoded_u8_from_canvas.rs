use js_sys::Uint8Array;
use tokio::sync::oneshot::channel;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Blob, HtmlCanvasElement};

pub async fn get_png_encoded_u8_from_canvas(canvas: &HtmlCanvasElement) -> Vec<u8> {
    let (sender, receiver) = channel::<Vec<u8>>();
    canvas
        .to_blob_with_type(
            Closure::once(Box::new(move |blob: Blob| {
                spawn_local(async move {
                    let u8_array = Uint8Array::new(
                        &JsFuture::from(blob.array_buffer())
                            .await
                            .expect("failed to read File as ArrayBuffer"),
                    );
                    sender
                        .send(u8_array.to_vec())
                        .expect("failed to send array buffer");
                })
            }) as Box<dyn FnOnce(Blob)>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
            "image/png",
        )
        .expect("failed to create blob url from canvas");
    receiver
        .await
        .expect("canvas_to_png_encoded_u8 did not receive data")
}
