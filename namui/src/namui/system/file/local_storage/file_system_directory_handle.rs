use super::file_system_handle::{FileSystemHandle, GetHandleOption};
use crate::file::types::PathLike;
use futures::Future;
use js_sys::AsyncIterator;
use std::pin::Pin;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=FileSystemHandle, js_name=FileSystemDirectoryHandle)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle)"]
    pub type FileSystemDirectoryHandle;

    #[wasm_bindgen(method, js_class="FileSystemDirectoryHandle", js_name=values)]
    #[doc = "The `values` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/values)"]
    fn values_unchecked(this: &FileSystemDirectoryHandle) -> AsyncIterator;

    #[wasm_bindgen(method, catch, js_class="FileSystemDirectoryHandle", js_name=getDirectoryHandle)]
    #[doc = "The `get_directory_handle` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    async fn get_directory_handle_unchecked(
        this: &FileSystemDirectoryHandle,
        name: String,
        options: GetHandleOption,
    ) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object, js_name=FileSystemDirectoryIteratorItem)]
    #[derive(Debug, Clone)]
    pub type FileSystemDirectoryIteratorItem;

    #[wasm_bindgen(method, getter, js_class="FileSystemDirectoryIteratorItem", js_name=done)]
    pub fn done(this: &FileSystemDirectoryIteratorItem) -> bool;

    #[wasm_bindgen(method, getter, js_class="FileSystemDirectoryIteratorItem", js_name=value)]
    pub fn value(this: &FileSystemDirectoryIteratorItem) -> Option<FileSystemHandle>;
}

impl<'a> FileSystemDirectoryHandle {
    pub fn get_directory_handle(
        &'a self,
        name: String,
        options: GetHandleOption,
    ) -> Pin<Box<dyn Future<Output = Result<FileSystemDirectoryHandle, JsValue>> + 'a>> {
        Box::pin(async move {
            let js_value = self.get_directory_handle_unchecked(name, options).await?;
            Ok(js_value.into())
        })
    }

    pub fn get_directory_handle_recursively(
        &'a self,
        path_like: impl PathLike,
        options: GetHandleOption,
    ) -> Pin<Box<dyn Future<Output = Result<FileSystemDirectoryHandle, JsValue>> + 'a>> {
        let path = path_like.path();
        Box::pin(async move {
            let mut cursor = self.clone();
            for directory_name in path.into_iter() {
                if directory_name == "/" {
                    continue;
                }
                cursor = cursor
                    .get_directory_handle(directory_name.to_string_lossy().to_string(), options)
                    .await?;
            }
            Ok(cursor)
        })
    }

    pub fn values(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<FileSystemHandle>, JsValue>> + 'a>> {
        Box::pin(async move {
            let js_async_iterator = self.values_unchecked();
            let mut values = vec![];
            loop {
                let promise = js_async_iterator.next()?;
                let js_value = JsFuture::from(promise).await?;
                let file_system_directory_iterator_item: FileSystemDirectoryIteratorItem =
                    js_value.into();
                if let Some(file_system_handle) = file_system_directory_iterator_item.value() {
                    values.push(file_system_handle);
                    continue;
                }
                break;
            }
            Ok(values)
        })
    }
}
