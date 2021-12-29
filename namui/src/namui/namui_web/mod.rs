use super::common::{FpsInfo, NamuiContext, NamuiImpl};
use super::manager::*;
use super::skia::{canvas_kit, CanvasKit, Surface};
use super::Namui;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, HtmlCanvasElement};
mod fetch;
pub use fetch::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

use once_cell::sync::OnceCell;
static MANAGERS: OnceCell<Mutex<Managers>> = OnceCell::new();
pub static CANVAS_KIT: OnceCell<Arc<CanvasKit>> = OnceCell::new();

pub fn get_managers() -> std::sync::MutexGuard<'static, Managers> {
    MANAGERS
        .get()
        .expect("managers not initialized")
        .lock()
        .unwrap()
}

impl NamuiImpl for Namui {
    fn init() -> NamuiContext {
        let canvas_kit = canvas_kit();
        let canvas_element = make_canvas_element().unwrap();
        let canvas_kit_surface = canvas_kit.MakeCanvasSurface(&canvas_element).unwrap();
        let surface = Surface::new(canvas_kit_surface);
        CANVAS_KIT.set(Arc::new(canvas_kit));

        if MANAGERS
            .set(Mutex::new(Managers {
                mouse_manager: Box::new(MouseManager::new(&canvas_element)),
                font_manager: Box::new(FontManager::new()),
                keyboard_manager: Box::new(KeyboardManager::new()),
                screen_manager: Box::new(ScreenManager::new()),
                image_manager: ImageManager::new(),
                wheel_manager: Box::new(WheelManager::new()),
            }))
            .is_err()
        {
            panic!("fail to initialize managers");
        }

        NamuiContext {
            surface,
            fps_info: FpsInfo {
                fps: 0,
                frame_count: 0,
                last_60_frame_time: Namui::now(),
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
