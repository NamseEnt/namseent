use super::engine_common::{EngineContext, EngineImpl, FpsInfo};
use super::manager::*;
use super::skia::{canvas_kit, Surface};
use super::{Engine, RenderingTree};
use std::sync::Mutex;
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlCanvasElement};

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

use once_cell::sync::OnceCell;
static MANAGERS: OnceCell<Mutex<Managers>> = OnceCell::new();

pub fn get_managers() -> std::sync::MutexGuard<'static, Managers> {
    MANAGERS
        .get()
        .expect("managers not initialized")
        .lock()
        .unwrap()
}

impl EngineImpl for Engine {
    fn init() -> EngineContext {
        let canvas_kit = canvas_kit();
        let canvas_element = make_canvas_element().unwrap();
        let canvas_kit_surface = canvas_kit.MakeCanvasSurface(&canvas_element).unwrap();
        let surface = Surface::new(canvas_kit_surface);

        if MANAGERS
            .set(Mutex::new(Managers {
                mouse_manager: Box::new(MouseManager::new(&canvas_element)),
                font_manager: Box::new(FontManager::new()),
                keyboard_manager: Box::new(KeyboardManager::new()),
            }))
            .is_err()
        {
            panic!("fail to initialize managers");
        }

        EngineContext {
            surface,
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
