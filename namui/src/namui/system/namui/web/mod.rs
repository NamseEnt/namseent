use super::skia::{canvas_kit, CanvasKit, Surface};
use super::system::*;
use crate::NamuiSystem;
use once_cell::sync::OnceCell;
use std::sync::Arc;
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

pub(crate) struct NamuiSystemWeb;

impl NamuiSystem for NamuiSystemWeb {
    fn init(self) -> crate::NamuiContext {
        console_error_panic_hook::set_once();
        prevent_context_menu_open();

        let canvas_kit = canvas_kit();
        let canvas_element = make_canvas_element().unwrap();
        let canvas_kit_surface = canvas_kit.MakeCanvasSurface(&canvas_element).unwrap();
        let surface = Surface::new(canvas_kit_surface);
        if CANVAS_KIT.set(Arc::new(canvas_kit)).is_err() {
            panic!("CANVAS_KIT already initialized");
        }

        sets(Systems {
            mouse: MouseSystem::new(&canvas_element),
            font: FontSystem::new(),
            keyboard: KeyboardSystem::new(),
            screen: ScreenSystem::new(),
            image: ImageSystem::new(),
            wheel: WheelSystem::new(),
            text_input: TextInputSystem::new(),
        });

        crate::NamuiContext::new(Arc::new(self), surface)
    }

    fn request_animation_frame(&self, callback: impl FnOnce() + 'static) {
        let callback = Closure::once(callback).into_js_value().unchecked_ref();
        window()
            .request_animation_frame(callback)
            .expect("request_animation_frame failed");
    }
    fn log(&self, format: String) {
        log(&format);
    }
    fn now(&self) -> Duration {
        Duration::from_millis(window().performance().unwrap().now() as u64)
    }
}

fn prevent_context_menu_open() {
    let document = web_sys::window().unwrap().document().unwrap();
    let on_context_menu = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        event.prevent_default();
    }) as Box<dyn FnMut(_)>);

    document.set_oncontextmenu(Some(on_context_menu.as_ref().unchecked_ref()));

    on_context_menu.forget();
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
            canvas_element.style().set_property("width", "100%");
            canvas_element.style().set_property("height", "100%");
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
