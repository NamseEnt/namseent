use super::get_root_directory::get_root_directory;
use crate::file::types::*;
use reqwest::Url;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum ReadDirError {
    DirNotFound(String),
    PathShouldBeAbsolute(String),
    InvalidUrl(String),
    Other(String),
}

pub async fn read_dir(path_like: impl PathLike) -> Result<Vec<Dirent>, ReadDirError> {
    let directory_path = path_like.path();
    if !directory_path.has_root() {
        return Err(ReadDirError::PathShouldBeAbsolute(format!(
            "{directory_path:?}"
        )));
    }
    let root = get_root_directory().await?;
    let directory_handle = root
        .get_directory_handle_recursively(
            directory_path.clone(),
            crate::file::local_storage::file_system_handle::GetHandleOption { create: false },
        )
        .await
        .map_err(|error| ReadDirError::DirNotFound(format!("{error:?}")))?;
    let mut entries = vec![];
    for entry in directory_handle.values().await?.into_iter() {
        let entry_path = directory_path.join(entry.name());
        let url_string = format!("local-storage:{}", entry_path.display());
        let url =
            Url::parse(&url_string).map_err(|error| ReadDirError::InvalidUrl(error.to_string()))?;
        match entry.kind() {
            super::file_system_handle::FileSystemHandleKind::Directory => {
                entries.push(Dirent::Directory(url))
            }
            super::file_system_handle::FileSystemHandleKind::File => {
                entries.push(Dirent::File(url))
            }
            _ => continue,
        }
    }
    Ok(entries)
}

impl From<JsValue> for ReadDirError {
    fn from(error: JsValue) -> Self {
        Self::Other(format!("{error:?}"))
    }
}
