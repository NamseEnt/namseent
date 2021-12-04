pub mod canvas_kit;
use async_trait::*;
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlCanvasElement};

use crate::engine::engine_common::{EngineContext, EngineImpl, Surface};

use super::{
    engine_common::{FpsInfo, Render},
    manager::{WebMouseManager, WebTypefaceManager},
    Canvas, Xy,
};

impl Surface for canvas_kit::CanvasKitSurface {
    fn flush(&self) {
        self.flush();
    }
}
impl Canvas for canvas_kit::Canvas {}

pub struct Engine;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);

    #[wasm_bindgen(js_namespace = globalThis, js_name = getCanvasKit)]
    fn get_canvas_kit() -> canvas_kit::CanvasKit;
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[async_trait]
impl EngineImpl for Engine {
    async fn init<TState: std::marker::Send>(
        state: TState,
        render: Render<TState>,
    ) -> EngineContext<TState> {
        let canvas_kit = get_canvas_kit();
        let canvas_element = make_canvas_element().unwrap();
        let surface = make_surface(&canvas_kit, &canvas_element).unwrap();
        let canvas = surface.getCanvas();

        EngineContext {
            state,
            render,
            surface: Box::new(surface),
            canvas: Box::new(canvas),
            fps_info: FpsInfo {
                fps: 0,
                frame_count: 0,
                last_60_frame_time: Engine::now(),
            },
            mouse_manager: Box::new(WebMouseManager::new(&canvas_element)),
            typeface_manager: Box::new(WebTypefaceManager::new(&canvas_kit)),
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

fn make_canvas_element() -> Result<HtmlCanvasElement, Element> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("canvas").unwrap();
    let canvas_element = wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element);

    match canvas_element {
        Ok(canvas_element) => {
            canvas_element.set_width(1920);
            canvas_element.set_height(1080);

            document
                .body()
                .unwrap()
                .append_child(&canvas_element)
                .unwrap();

            Result::Ok(canvas_element)
        }
        Err(e) => Result::Err(e),
    }
}

fn make_surface(
    canvas_kit: &canvas_kit::CanvasKit,
    canvas: &HtmlCanvasElement,
) -> Option<canvas_kit::CanvasKitSurface> {
    return canvas_kit.MakeCanvasSurface(&canvas);
}
