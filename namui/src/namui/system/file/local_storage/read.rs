use crate::system::platform_utils::web::window;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum ReadError {
    NotFound(String),
    LocalStorageNotFound,
    Unknown(String),
}

pub fn read(key: &str) -> Result<String, ReadError> {
    match window().local_storage()? {
        Some(local_storage) => match local_storage.get_item(key)? {
            Some(content) => Ok(content),
            None => Err(ReadError::NotFound(format!("File not found: {key}"))),
        },
        None => Err(ReadError::LocalStorageNotFound),
    }
}

impl From<JsValue> for ReadError {
    fn from(error: JsValue) -> Self {
        ReadError::Unknown(format!("{error:#?}"))
    }
}
