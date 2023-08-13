use super::get_root_directory::get_root_directory;
use crate::file::types::*;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum MakeDirError {
    PathShouldBeAbsolute(String),
    Other(String),
}

pub async fn make_dir(path_like: impl PathLike) -> Result<(), MakeDirError> {
    let directory_path = path_like.path();
    if !directory_path.has_root() {
        return Err(MakeDirError::PathShouldBeAbsolute(format!(
            "{directory_path:?}"
        )));
    }
    let root = get_root_directory().await?;
    root.get_directory_handle_recursively(
        directory_path.clone(),
        crate::file::local_storage::file_system_handle::GetHandleOption { create: true },
    )
    .await?;
    Ok(())
}

impl From<JsValue> for MakeDirError {
    fn from(error: JsValue) -> Self {
        Self::Other(format!("{error:?}"))
    }
}
