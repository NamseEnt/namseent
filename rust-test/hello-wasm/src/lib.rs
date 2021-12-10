mod engine;
mod utils;
use wasm_bindgen::prelude::*;

use crate::engine::{RectParam, RectStroke, RectStyle};

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
    engine::start(State { value: 0 }, render_start).await;
}

fn render_start(state: &mut State) -> engine::Rendering {
    return render![render_text(state, 0)];
}

fn render_text(state: &mut State, index: i32) -> engine::Rendering {
    let engine_state = engine::state();
    let mouse_x = engine_state.mouse_position.x;
    let color = engine::Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    let a = engine::text(engine::TextParam {
        x: 100.0 + 100.0 * index as f32,
        y: 100.0,
        align: engine::TextAlign::Left,
        baseline: engine::TextBaseline::Top,
        font_type: engine::FontType {
            font_weight: engine::FontWeight::_400,
            language: engine::Language::Ko,
            serif: false,
            size: 16,
        },
        style: engine::TextStyle {
            color,
            background: None,
            border: None,
            drop_shadow: None,
        },
        text: format!("mouse: {}", mouse_x),
    });
    let b = engine::rect(RectParam {
        x: 200.0,
        y: 200.0,
        width: 100.0,
        height: 100.0,
        id: None,
        style: RectStyle {
            stroke: Some(RectStroke {
                color: engine::Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                width: 1.0,
                border_position: engine::BorderPosition::Inside,
            }),
            ..Default::default()
        },
    });
    render![a, b]

    // let font = engine::

    // render![engine::RenderingData {
    //     draw_calls: vec![engine::DrawCall {
    //         commands: vec![engine::DrawCommand::Text(engine::TextDrawCommand {
    //             text: format!(
    //                 "{}, {}, mouseX: {}",
    //                 index,
    //                 state.value,
    //                 mouse_x.to_string()
    //             ),
    //             x: 100,
    //             y: 100,
    //             align: engine::TextAlign::Left,
    //             baseline: engine::TextBaseline::Top,
    //             font,
    //             paint,
    //         })],
    //     }],
    // }]
}
