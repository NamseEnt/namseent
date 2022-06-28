use super::common::NamuiImpl;
use super::manager::*;
use super::skia::{canvas_kit, CanvasKit, Surface};
use super::Namui;
use once_cell::sync::OnceCell;
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

pub(crate) fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub static CANVAS_KIT: OnceCell<Arc<CanvasKit>> = OnceCell::new();

static MOCKED_NOW: OnceCell<Mutex<Duration>> = OnceCell::new();

pub mod mock {
    use super::*;
    use std::time::Duration;

    pub fn set_now(now: Duration) {
        let mut mocked_now = MOCKED_NOW.get_or_init(|| Mutex::new(now)).lock().unwrap();
        *mocked_now = now;
    }
}

impl NamuiImpl for Namui {
    fn init() -> crate::NamuiContext {
        console_error_panic_hook::set_once();

        let canvas_kit = canvas_kit();
        let canvas_element = make_canvas_element().unwrap();
        let canvas_kit_surface = canvas_kit.MakeCanvasSurface(&canvas_element).unwrap();
        let surface = Surface::new(canvas_kit_surface);
        CANVAS_KIT.set(Arc::new(canvas_kit)).unwrap();

        set_managers(Managers {
            mouse_manager: MouseManager::new(&canvas_element),
            font_manager: FontManager::new(),
            keyboard_manager: KeyboardManager::new(),
            screen_manager: ScreenManager::new(),
            image_manager: ImageManager::new(),
            wheel_manager: WheelManager::new(),
            text_input_manager: TextInputManager::new(),
        });
        crate::NamuiContext::new(surface)
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
        let mocked_now = MOCKED_NOW
            .get_or_init(|| Mutex::new(Duration::ZERO))
            .lock()
            .unwrap();
        *mocked_now
    }
}

fn make_canvas_element() -> Result<HtmlCanvasElement, Element> {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element("canvas").unwrap();
    let canvas_element = wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element);
    match canvas_element {
        Ok(canvas_element) => {
            let screen_size = crate::screen::size();
            canvas_element.set_width(screen_size.width as u32);
            canvas_element.set_height(screen_size.height as u32);
            let _ = canvas_element.style().set_property("width", "100%");
            let _ = canvas_element.style().set_property("height", "100%");
            canvas_element.set_id("canvas");
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
