use super::InitResult;
use crate::*;
use namui_type::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;

#[derive(Debug)]
pub struct RawFileDropEvent {
    pub files: Vec<File>,
    pub global_xy: Xy<Px>,
}

pub(super) async fn init() -> InitResult {
    let document = window().unwrap().document().unwrap();

    document
        .add_event_listener_with_callback(
            "dragover",
            Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
                event.prevent_default();
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    document
        .add_event_listener_with_callback(
            "drop",
            Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
                event.prevent_default();
                crate::hooks::on_raw_event(RawEvent::FileDrop {
                    xy: Xy::new(event.client_x().px(), event.client_y().px()),
                    data_transfer: event.data_transfer(),
                });
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    Ok(())
}
