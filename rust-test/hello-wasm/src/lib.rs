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

static STATE: State = State { value: 0 };

#[wasm_bindgen]
pub async fn greet() {
    start_engine(State { value: 0 }, render_start).await;
}

fn render_start(engine_state: &EngineState, state: &mut State) -> Rendering {
    return render![
        render_text2(engine_state, state, 0),
        render_text2(engine_state, state, 1)
    ];
}

// render_func!(start, State, state, {
//     return render![render_text2(state, 0), render_text2(state, 1)];
// });

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

fn render_text2(engine_state: &EngineState, state: &mut State, index: i32) -> Rendering {
    let mouse_x = engine_state.mouse_position.x;

    render![RenderingData {
        draw_calls: vec![DrawCall {
            commands: vec![DrawCommand::Text(TextDrawCommand {
                text: format!(
                    "{}, {}, mouseX: {}",
                    index,
                    state.value,
                    mouse_x.to_string()
                ),
            })],
        }],
    }]
}
