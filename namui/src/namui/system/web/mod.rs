mod async_func;
mod event;
mod sync_func;

use super::*;
use crate::*;
pub use async_func::*;
pub use event::{handle_web_event, WebEvent};
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
pub use sync_func::*;
use tokio::sync::Notify;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

static mut ASYNC_FUNCTION_RESULT_NOTIFY_MAP: OnceLock<HashMap<usize, Arc<Notify>>> =
    OnceLock::new();
static mut ASYNC_FUNCTION_RESULT_MAP: OnceLock<HashMap<usize, JsValue>> = OnceLock::new();

pub(super) fn init() -> InitResult {
    unsafe {
        ASYNC_FUNCTION_RESULT_NOTIFY_MAP
            .set(HashMap::new())
            .unwrap();
        ASYNC_FUNCTION_RESULT_MAP.set(HashMap::new()).unwrap();
    }
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis)]
    fn getInitialWindowSize() -> JsValue;

    #[wasm_bindgen(js_namespace = globalThis)]
    fn executeFunctionSyncOnMain(
        args_names: Vec<JsValue>, // Vec<String>
        code: String,
        args: Vec<JsValue>,
    ) -> JsValue;

    #[wasm_bindgen(js_namespace = globalThis)]
    fn startExecuteAsyncFunction(
        args_names: Vec<JsValue>, // Vec<String>
        code: String,
        args: Box<[JsValue]>,
    ) -> usize;

    #[wasm_bindgen(js_namespace = globalThis)]
    fn getAsyncFunctionResult(id: usize) -> JsValue;
}

pub fn location_search() -> String {
    execute_function_sync(
        "
        return window.location.search;
        ",
    )
    .run()
}

pub fn initial_window_size() -> Wh<Px> {
    #[derive(serde::Deserialize)]
    struct Response {
        width: f32,
        height: f32,
    }
    let response: Response = serde_wasm_bindgen::from_value(getInitialWindowSize()).unwrap();

    Wh {
        width: response.width.px(),
        height: response.height.px(),
    }
}
