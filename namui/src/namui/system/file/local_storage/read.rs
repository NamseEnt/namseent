use super::get_root_directory::get_root_directory;
use crate::{file::types::PathLike, simple_error_impl};
use js_sys::Uint8Array;
use std::path::PathBuf;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub enum ReadError {
    FileNotFound(String),
    PathShouldBeAbsolute(String),
    DirNotFound(String),
    Other(String),
}
simple_error_impl!(ReadError);

pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>, ReadError> {
    let file_path = path_like.path();
    if !file_path.has_root() {
        return Err(ReadError::PathShouldBeAbsolute(format!("{file_path:?}")));
    }
    let file_name = match file_path.file_name() {
        Some(file_name) => format!("{file_name:?}"),
        None => return Err(ReadError::FileNotFound(format!("{file_path:?}"))),
    };
    let parent_directory_path = match file_path.parent().as_deref() {
        Some(path) => path.to_path_buf(),
        None => PathBuf::new(),
    };
    let root = get_root_directory().await?;
    let parent_directory_handle = root
        .get_directory_handle_recursively(
            parent_directory_path,
            crate::file::local_storage::file_system_handle::GetHandleOption { create: false },
        )
        .await
        .map_err(|error| ReadError::DirNotFound(format!("{error:?}")))?;
    let file_handle = parent_directory_handle
        .get_file_handle(
            file_name,
            super::file_system_handle::GetHandleOption { create: false },
        )
        .await
        .map_err(|error| ReadError::FileNotFound(format!("{error:?}")))?;
    let file = file_handle.get_file().await?;
    let js_value = JsFuture::from(file.array_buffer()).await?;
    let array_buffer = js_value.dyn_into::<js_sys::ArrayBuffer>()?;
    let uint8array = Uint8Array::new(&array_buffer);
    Ok(uint8array.to_vec())
}

impl From<JsValue> for ReadError {
    fn from(error: JsValue) -> Self {
        Self::Other(format!("{error:?}"))
    }
}
