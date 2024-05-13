use crate::system::{platform_utils::web::window, InitResult};
use crate::*;
use wasm_bindgen::{prelude::Closure, JsCast};

pub(crate) async fn init() -> InitResult {
    let window = window();

    window
        .add_event_listener_with_callback(
            "resize",
            Closure::wrap(Box::new(move || {
                crate::hooks::on_raw_event(RawEvent::ScreenResize { wh: size() });
            }) as Box<dyn FnMut()>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    animation_frame_tick();

    Ok(())
}

fn animation_frame_tick() {
    crate::hooks::on_raw_event(RawEvent::ScreenRedraw {});

    window()
        .request_animation_frame(
            Closure::wrap(Box::new(animation_frame_tick) as Box<dyn FnMut()>)
                .into_js_value()
                .unchecked_ref(),
        )
        .unwrap();
}

pub fn size() -> crate::Wh<IntPx> {
    let window = window();
    crate::Wh {
        width: (window.inner_width().unwrap().as_f64().unwrap() as i32).int_px(),
        height: (window.inner_height().unwrap().as_f64().unwrap() as i32).int_px(),
    }
}

pub(crate) fn take_main_thread() {
    // Do nothing
}
