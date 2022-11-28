use super::get_root_directory::get_root_directory;
use crate::{file::types::PathLike, simple_error_impl};
use js_sys::Uint8Array;
use std::path::PathBuf;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

const CHUNK_SIZE: usize = 4 * 1024; // 4 KiB

#[derive(Debug)]
pub enum WriteError {
    FileNotFound(String),
    PathShouldBeAbsolute(String),
    DirNotFound(String),
    Other(String),
}
simple_error_impl!(WriteError);

pub async fn write(path_like: impl PathLike, content: impl AsRef<[u8]>) -> Result<(), WriteError> {
    let file_path = path_like.path();
    if !file_path.has_root() {
        return Err(WriteError::PathShouldBeAbsolute(
            file_path.to_string_lossy().to_string(),
        ));
    }
    let file_name = match file_path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => return Err(WriteError::FileNotFound(format!("{file_path:?}"))),
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
        .await
        .map_err(|error| WriteError::DirNotFound(format!("{error:?}")))?;
    let file_handle = parent_directory_handle
        .get_file_handle(
            file_name,
            super::file_system_handle::GetHandleOption { create: true },
        )
        .await
        .map_err(|error| WriteError::FileNotFound(format!("{error:?}")))?;

    let file_stream = file_handle.create_writable().await?;
    let writer = file_stream.get_writer()?;
    for chunk in content.as_ref().chunks(CHUNK_SIZE) {
        let unit8array = Uint8Array::new_with_length(CHUNK_SIZE as u32);
        unit8array.copy_from(chunk);
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
