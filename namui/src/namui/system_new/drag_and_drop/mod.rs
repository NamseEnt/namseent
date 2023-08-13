use super::InitResult;
use crate::File;
use namui_type::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;

#[derive(Debug)]
pub struct RawFileDropEvent {
    pub files: Vec<File>,
    pub global_xy: Xy<Px>,
}

pub(super) async fn init() -> InitResult {
    // let document = window().unwrap().document().unwrap();

    // document
    //     .add_event_listener_with_callback(
    //         "dragover",
    //         Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
    //             event.prevent_default();
    //         }) as Box<dyn FnMut(_)>)
    //         .into_js_value()
    //         .unchecked_ref(),
    //     )
    //     .unwrap();

    // document
    //     .add_event_listener_with_callback(
    //         "drop",
    //         Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
    //             event.prevent_default();
    //             let Some(data_transfer) = event.data_transfer() else {
    //                 return;
    //             };

    //             let items = data_transfer.items();

    //             if items.length() == 0 {
    //                 return;
    //             }

    //             let mut files = Vec::with_capacity(items.length() as usize);
    //             for index in 0..items.length() {
    //                 let web_file = items.get(index).unwrap().get_as_file().unwrap().unwrap();
    //                 let file = File::new(web_file);
    //                 files.push(file);
    //             }

    //             let file_drop_event = RawFileDropEvent {
    //                 files,
    //                 global_xy: Xy::new(event.client_x().px(), event.client_y().px()),
    //             };

    //             crate::event::send(NamuiEvent::FileDrop(file_drop_event));
    //         }) as Box<dyn FnMut(_)>)
    //         .into_js_value()
    //         .unchecked_ref(),
    //     )
    //     .unwrap();

    Ok(())
}
