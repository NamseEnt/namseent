use crate::system::platform_utils::web::window;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum WriteError {
    LocalStorageNotFound,
    Unknown(String),
}

pub fn write(key: &str, content: &str) -> Result<(), WriteError> {
    match window().local_storage()? {
        Some(local_storage) => Ok(local_storage.set_item(key, content)?),
        None => Err(WriteError::LocalStorageNotFound),
    }
}

impl From<JsValue> for WriteError {
    fn from(error: JsValue) -> Self {
        WriteError::Unknown(format!("{error:#?}"))
    }
}
