use super::{platform_utils::web::*, InitResult};
use crate::namui::skia::{canvas_kit, Surface};
use std::sync::{Arc, Mutex, MutexGuard};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;

struct GraphicsSystem {
    pub surface: Mutex<Surface>,
}

lazy_static::lazy_static! {
    static ref GRAPHICS_SYSTEM: Arc<GraphicsSystem> = Arc::new(GraphicsSystem::new());
}

pub(crate) async fn init() -> InitResult {
    lazy_static::initialize(&GRAPHICS_SYSTEM);
    Ok(())
}

impl GraphicsSystem {
    fn new() -> Self {
        make_canvas_element();

        Self {
            surface: Mutex::new(new_surface()),
        }
    }
}

fn new_surface() -> Surface {
    let canvas_kit_surface = canvas_kit()
        .MakeWebGLCanvasSurface(&canvas_element())
        .expect("fail to make canvas surface");
    Surface::new(canvas_kit_surface)
}

fn make_canvas_element() -> HtmlCanvasElement {
    let element = document().create_element("canvas").unwrap();
    let canvas_element = wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element).unwrap();

    let screen_size = screen_size();
    canvas_element.set_width(screen_size.width as u32);
    canvas_element.set_height(screen_size.height as u32);
    canvas_element
        .style()
        .set_property("width", "100%")
        .expect("fail to set width");
    canvas_element
        .style()
        .set_property("height", "100%")
        .expect("fail to set height");
    canvas_element.set_id("canvas");
    document()
        .body()
        .unwrap()
        .append_child(&canvas_element)
        .unwrap();

    canvas_element
}

pub fn request_animation_frame(callback: impl FnOnce() + 'static) {
    window()
        .request_animation_frame(Closure::once(callback).into_js_value().unchecked_ref())
        .expect("request_animation_frame failed");
}

/// NOTE: Do not save surface as variable to prevent re-locking.
pub(crate) fn surface<'a>() -> MutexGuard<'a, Surface> {
    GRAPHICS_SYSTEM.surface.lock().unwrap()
}

pub(crate) fn resize_surface(screen_size: crate::Wh<i16>) {
    let canvas_element = canvas_element();
    canvas_element.set_width(screen_size.width as u32);
    canvas_element.set_height(screen_size.height as u32);

    *GRAPHICS_SYSTEM.surface.lock().unwrap() = new_surface();
}
