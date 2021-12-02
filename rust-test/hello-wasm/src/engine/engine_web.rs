mod canvas_kit;
use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Rc,
    time::{Duration, Instant},
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlCanvasElement;

use crate::engine::engine_common::{EngineContext, EngineImpl, Surface};

use super::engine_common::FpsInfo;

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
    fn init<TState>(state: TState) -> EngineContext<TState> {
        let surface = make_surface().unwrap();

        EngineContext {
            state,
            surface: Box::new(surface),
            fps_info: FpsInfo {
                fps: 0,
                frame_count: 0,
                last_60_frame_time: Engine::now(),
            },
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

fn make_surface() -> Result<canvas_kit::Surface, String> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("canvas").unwrap();
    let canvas = match wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element) {
        Ok(canvas) => canvas,
        Err(_) => panic!("Canvas element not found"),
    };
    return Ok(canvas_kit::MakeCanvasSurface(&canvas).unwrap());
}
