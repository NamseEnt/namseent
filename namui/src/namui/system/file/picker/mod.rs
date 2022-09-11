use crate::system::platform_utils::web::document;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{FileList, InputEvent};

pub struct File {
    inner: web_sys::File,
}

/// NOTE: This would not emit any events if user cancels the file selection and closes the picker.
pub async fn open() -> Box<[File]> {
    let input = document()
        .create_element("input")
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    input.set_type("file");
    input.set_multiple(true);

    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        input.set_onchange(Some(
            Closure::wrap(Box::new(move |event: InputEvent| {
                let target = &event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap();
                let files = target.files().unwrap();
                resolve.call1(&JsValue::UNDEFINED, files.as_ref()).unwrap();
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        ));
    });

    input.click();

    let files = JsFuture::from(promise)
        .await
        .unwrap()
        .dyn_into::<FileList>()
        .unwrap();

    let mut result = Vec::new();
    for index in 0..files.length() {
        let file = files.item(index).unwrap();
        result.push(File { inner: file });
    }

    result.into_boxed_slice()
}

impl File {
    pub fn name(&self) -> String {
        self.inner.name()
    }
    pub async fn content(&self) -> Box<[u8]> {
        let array_buffer = JsFuture::from(self.inner.array_buffer()).await.unwrap();
        let typed_array = js_sys::Uint8Array::new(&array_buffer);
        typed_array.to_vec().into_boxed_slice()
    }
}
