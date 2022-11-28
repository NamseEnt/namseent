use super::file_system_handle::FileSystemHandle;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{File, WritableStream};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=FileSystemHandle, js_name=FileSystemFileHandle)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle)"]
    pub type FileSystemFileHandle;

    #[wasm_bindgen(method, catch, js_class="FileSystemFileHandle", js_name=getFile)]
    #[doc = "The `getFile` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile)"]
    async fn get_file_unchecked(this: &FileSystemFileHandle) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_class="FileSystemFileHandle", js_name=createWritable)]
    #[doc = "The `createWritable` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    async fn create_writable_unchecked(this: &FileSystemFileHandle) -> Result<JsValue, JsValue>;
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

impl FileSystemFileHandle {
    pub async fn get_file(&self) -> Result<File, JsValue> {
        let js_value = self.get_file_unchecked().await?;
        let file: File = js_value.dyn_into()?;
        Ok(file)
    }

    pub async fn create_writable(&self) -> Result<FileSystemWritableFileStream, JsValue> {
        let js_value = self.create_writable_unchecked().await?;
        let writable_file_stream: FileSystemWritableFileStream = js_value.dyn_into()?;
        Ok(writable_file_stream)
    }
}
