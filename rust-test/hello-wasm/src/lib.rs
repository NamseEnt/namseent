mod engine;
use engine::*;
mod utils;

use engine::start_engine;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn test(value: &JsValue);
}

struct State {
    value: i32,
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
    start_engine(State { value: 0 }, |state| {
        state.value += 1;
        engine::Engine::log(state.value.to_string());
        None
    });
}
