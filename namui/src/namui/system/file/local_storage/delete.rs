use super::{file_system_directory_handle::DeleteOption, get_root_directory::get_root_directory};
use crate::{file::types::PathLike, simple_error_impl};
use std::path::PathBuf;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum DeleteError {
    InvalidFileName(String),
    PathShouldBeAbsolute(String),
    DirNotFound(String),
    Other(String),
}
simple_error_impl!(DeleteError);

pub async fn delete(path_like: impl PathLike) -> Result<(), DeleteError> {
    let file_path = path_like.path();
    if !file_path.has_root() {
        return Err(DeleteError::PathShouldBeAbsolute(format!("{file_path:?}")));
    }
    let target_entry_name = match file_path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => return Err(DeleteError::InvalidFileName(format!("{file_path:?}"))),
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
        .map_err(|error| DeleteError::DirNotFound(format!("{error:?}")))?;
    parent_directory_handle
        .remove_entry(target_entry_name, DeleteOption { recursive: true })
        .await?;
    Ok(())
}

impl From<JsValue> for DeleteError {
    fn from(error: JsValue) -> Self {
        Self::Other(format!("{error:?}"))
    }
}
