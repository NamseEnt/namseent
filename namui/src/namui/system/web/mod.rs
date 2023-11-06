mod async_function;
mod event_listener;
mod function;

use super::*;
use crate::*;
pub use async_function::*;
pub use event_listener::*;
pub use function::*;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

pub(super) async fn init() -> InitResult {
    event_listener::init();
    Ok(())
}
