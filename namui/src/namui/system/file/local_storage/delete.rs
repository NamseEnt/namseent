use crate::system::platform_utils::web::window;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum DeleteError {
    LocalStorageNotFound,
    Unknown(String),
}

pub fn delete(key: &str) -> Result<(), DeleteError> {
    match window().local_storage()? {
        Some(local_storage) => Ok(local_storage.delete(key)?),
        None => Err(DeleteError::LocalStorageNotFound),
    }
}

impl From<JsValue> for DeleteError {
    fn from(error: JsValue) -> Self {
        DeleteError::Unknown(format!("{error:#?}"))
    }
}
