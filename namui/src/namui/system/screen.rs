use super::{platform_utils::web::window, InitResult};
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};

pub(super) async fn init() -> InitResult {
    let window = window();
    let screen_size = Arc::new(RwLock::new((
        window.inner_width().unwrap().as_f64().unwrap() as i16,
        window.inner_height().unwrap().as_f64().unwrap() as i16,
    )));

    window
        .add_event_listener_with_callback(
            "resize",
            Closure::wrap(Box::new(move || {
                let window = super::platform_utils::web::window();
                let mut screen_size = screen_size.write().unwrap();
                *screen_size = (
                    window.inner_width().unwrap().as_f64().unwrap() as i16,
                    window.inner_height().unwrap().as_f64().unwrap() as i16,
                );
                crate::event::send(crate::event::NamuiEvent::ScreenResize(crate::Wh {
                    width: screen_size.0,
                    height: screen_size.1,
                }));
            }) as Box<dyn FnMut()>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();
    Ok(())
}

pub fn size() -> crate::Wh<i16> {
    let window = window();
    crate::Wh {
        width: window.inner_width().unwrap().as_f64().unwrap() as i16,
        height: window.inner_height().unwrap().as_f64().unwrap() as i16,
    }
}
