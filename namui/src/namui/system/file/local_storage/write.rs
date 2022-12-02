use super::get_root_directory::get_root_directory;
use crate::{file::types::PathLike, simple_error_impl};
use js_sys::Uint8Array;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub enum WriteError {
    InvalidFileName(String),
    PathShouldBeAbsolute(String),
    Other(String),
}
simple_error_impl!(WriteError);

pub async fn write(path_like: impl PathLike, content: impl AsRef<[u8]>) -> Result<(), WriteError> {
    let file_path = path_like.path();
    if !file_path.has_root() {
        return Err(WriteError::PathShouldBeAbsolute(format!("{file_path:?}")));
    }
    let file_name = match file_path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => return Err(WriteError::InvalidFileName(format!("{file_path:?}"))),
    };
    let parent_directory_path = match file_path.parent().as_deref() {
        Some(path) => path.to_path_buf(),
        None => PathBuf::new(),
    };
    let root = get_root_directory().await?;
    let parent_directory_handle = root
        .get_directory_handle_recursively(
            parent_directory_path,
            crate::file::local_storage::file_system_handle::GetHandleOption { create: true },
        )
        .await?;
    let file_handle = parent_directory_handle
        .get_file_handle(
            file_name,
            super::file_system_handle::GetHandleOption { create: true },
        )
        .await?;

    let file_stream = file_handle.create_writable().await?;
    let writer = file_stream.get_writer()?;
    unsafe {
        let unit8array = Uint8Array::view(content.as_ref());
        JsFuture::from(writer.write_with_chunk(&unit8array)).await?;
    }
    JsFuture::from(writer.close()).await?;
    Ok(())
}

impl From<JsValue> for WriteError {
    fn from(error: JsValue) -> Self {
        Self::Other(format!("{error:?}"))
    }
}
