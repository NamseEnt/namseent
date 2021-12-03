mod engine;
use engine::*;
mod utils;

use engine::draw::*;
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
    start_engine(State { value: 0 }, render_start);
}

render_func!(start, State, state, {
    return render![render_text(state), render_text(state)];
});

render_func!(text, State, state, {
    state.value += 1;

    render![RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Text(TextDrawCommand {
                text: format!("{}", state.value),
            })],
        }],
    }]
});
