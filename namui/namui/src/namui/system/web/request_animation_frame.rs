use crate::{system::platform_utils::window, RawEvent};
use wasm_bindgen::{prelude::Closure, JsCast};

// Remove this function if you don't need RequestAnimationFrame.
pub fn request_animation_frame() {
    window()
        .request_animation_frame(
            Closure::wrap(Box::new(move || {
                crate::hooks::on_raw_event(RawEvent::AnimationFrame);
            }) as Box<dyn FnMut()>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();
}
