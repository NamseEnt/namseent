mod async_function;
mod function;

use super::*;
use crate::*;
pub use async_function::*;
pub use function::*;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

pub(super) async fn init() -> InitResult {
    Ok(())
}
