mod canvas_kit;
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlCanvasElement};

use crate::engine::engine_common::{EngineContext, EngineImpl, Surface};

use super::{
    device::WebMouseManager,
    engine_common::{FpsInfo, Render},
    Xy,
};

impl Surface for canvas_kit::Surface {}
pub struct Engine;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

impl EngineImpl for Engine {
    fn init<TState>(state: TState, render: Render<TState>) -> EngineContext<TState> {
        let canvas = make_canvas().unwrap();
        let surface = make_surface(&canvas).unwrap();

        EngineContext {
            state,
            render,
            surface: Box::new(surface),
            fps_info: FpsInfo {
                fps: 0,
                frame_count: 0,
                last_60_frame_time: Engine::now(),
            },
            mouse_manager: Box::new(WebMouseManager::new(&canvas)),
        }
    }

    fn request_animation_frame(callback: Box<dyn FnOnce()>) {
        let closure = Closure::once(callback);

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    fn log(format: String) {
        log(&format);
    }
    fn now() -> Duration {
        Duration::from_millis(window().performance().unwrap().now() as u64)
    }
}

fn make_canvas() -> Result<HtmlCanvasElement, Element> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("canvas").unwrap();
    let canvas = wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element);

    match canvas {
        Ok(canvas) => {
            canvas.set_width(1920);
            canvas.set_height(1080);

            document.body().unwrap().append_child(&canvas).unwrap();

            Result::Ok(canvas)
        }
        Err(e) => Result::Err(e),
    }
}

fn make_surface(canvas: &HtmlCanvasElement) -> Result<canvas_kit::Surface, String> {
    return Ok(canvas_kit::MakeCanvasSurface(&canvas).unwrap());
}
