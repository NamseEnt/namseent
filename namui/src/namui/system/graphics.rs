use super::{platform_utils::web::*, InitResult};
use crate::namui::skia::{canvas_kit, ColorSpace, Surface};
use js_sys::Promise;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex, MutexGuard};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlCanvasElement;

struct GraphicsSystem {
    pub surface: Mutex<Surface>,
}

static GRAPHICS_SYSTEM: OnceCell<Arc<GraphicsSystem>> = OnceCell::new();

pub(crate) async fn init() -> InitResult {
    let graphics_system = GraphicsSystem::new().await;
    GRAPHICS_SYSTEM
        .set(Arc::new(graphics_system))
        .map_err(|_| "Failed to set GRAPHICS_SYSTEM.")?;
    Ok(())
}

impl GraphicsSystem {
    async fn new() -> Self {
        make_canvas_element();

        Self {
            surface: Mutex::new(new_surface().await),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = getGpuDevice)]
    fn get_gpu_device() -> Promise;
}

async fn new_surface() -> Surface {
    let canvas_element = canvas_element();

    let device = JsFuture::from(get_gpu_device())
        .await
        .expect("getGpuDevice failed");

    let device_context = canvas_kit()
        .MakeGPUDeviceContext(device)
        .expect("MakeGPUDeviceContext failed");

    let canvas_context = canvas_kit()
        .MakeGPUCanvasContext(device_context, canvas_element)
        .expect("MakeGPUCanvasContext failed");

    let canvas_kit_surface = canvas_kit()
        .MakeGPUCanvasSurface(
            canvas_context,
            ColorSpace::SRGB.into_canvas_kit(),
            None,
            None,
        )
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
    GRAPHICS_SYSTEM.get().unwrap().surface.lock().unwrap()
}

pub(crate) fn resize_surface(screen_size: crate::Wh<i16>) {
    let canvas_element = canvas_element();
    canvas_element.set_width(screen_size.width as u32);
    canvas_element.set_height(screen_size.height as u32);

    spawn_local(async move {
        *GRAPHICS_SYSTEM.get().unwrap().surface.lock().unwrap() = new_surface().await;
    })
}
