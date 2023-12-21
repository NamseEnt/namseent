mod async_function;
mod event_listener;
mod function;
mod request_animation_frame;

use super::*;
use crate::*;
pub use async_function::*;
pub use event_listener::*;
pub use function::*;
pub use request_animation_frame::*;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

pub(super) async fn init() -> InitResult {
    event_listener::init();
    Ok(())
}
