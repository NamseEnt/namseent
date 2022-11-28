use super::file_system_handle::FileSystemHandle;
use futures::Future;
use js_sys::Promise;
use std::pin::Pin;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, WritableStream};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=FileSystemHandle, js_name=FileSystemFileHandle)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle)"]
    pub type FileSystemFileHandle;

    #[wasm_bindgen(method, js_class="FileSystemFileHandle", js_name=getFile)]
    #[doc = "The `getFile` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile)"]
    fn get_file_unchecked(this: &FileSystemFileHandle) -> Promise;

    #[wasm_bindgen(method, js_class="FileSystemFileHandle", js_name=createWritable)]
    #[doc = "The `createWritable` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    fn create_writable_unchecked(this: &FileSystemFileHandle) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=WritableStream, js_name=FileSystemWritableFileStream)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemWritableFileStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream)"]
    pub type FileSystemWritableFileStream;
}

impl<'a> FileSystemFileHandle {
    pub fn get_file(&'a self) -> Pin<Box<dyn Future<Output = Result<File, JsValue>> + 'a>> {
        Box::pin(async move {
            let promise = self.get_file_unchecked();
            let js_value = JsFuture::from(promise).await?;
            let file: File = js_value.dyn_into()?;
            Ok(file)
        })
    }

    pub fn create_writable(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<FileSystemWritableFileStream, JsValue>> + 'a>> {
        Box::pin(async move {
            let promise = self.create_writable_unchecked();
            let js_value = JsFuture::from(promise).await?;
            let writable_file_stream: FileSystemWritableFileStream = js_value.dyn_into()?;
            Ok(writable_file_stream)
        })
    }
}
