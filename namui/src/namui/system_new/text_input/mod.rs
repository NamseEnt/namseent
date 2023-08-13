// mod find;
// pub(crate) mod key_down;
// mod mouse_event;
// mod post_render;
// mod selection;

use super::InitResult;
use crate::web::WebEvent;
// use crate::{namui::render::text_input::*, web::SelectionDirection};
// pub(crate) use find::*;
// pub(crate) use mouse_event::*;
// pub use selection::*;
// use std::sync::{MutexGuard, OnceLock};
// use std::{ops::ControlFlow, sync::Mutex};

// #[derive(Debug)]
// struct TextInputSystem {
//     // last_focused_text_input: Option<TextInputCustomData>,
//     dragging_text_input: Option<TextInputCustomData>,
//     focus_requested_text_input_id: Option<Uuid>,
// }

// static TEXT_INPUT_SYSTEM: OnceLock<Mutex<TextInputSystem>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    // TEXT_INPUT_SYSTEM
    //     .set(Mutex::new(TextInputSystem {
    //         // last_focused_text_input: None,
    //         dragging_text_input: None,
    //         focus_requested_text_input_id: None,
    //     }))
    //     .unwrap();

    // TEXT_INPUT_ATOM.init(TextInputCtx {
    //     last_focused_text_input: None,
    //     selection: Selection::None,
    // });

    Ok(())
}

pub(crate) fn on_web_event(web_event: &WebEvent) {
    match web_event {
        _ => {}
    }
}

// fn text_input_system<'a>() -> MutexGuard<'a, TextInputSystem> {
//     TEXT_INPUT_SYSTEM.get().unwrap().lock().unwrap()
// }

// fn text_input_system_mutate<T>(mutate: impl FnOnce(&mut TextInputSystem) -> T) -> T {
//     let mut text_input_system = TEXT_INPUT_SYSTEM.get().unwrap().lock().unwrap();
//     mutate(&mut text_input_system)
// }

// pub fn is_focused(text_input_id: crate::Uuid) -> bool {
//     TEXT_INPUT_ATOM
//         .get()
//         .last_focused_text_input
//         .as_ref()
//         .map(|text_input| text_input.id == (text_input_id))
//         .unwrap_or(false)
// }
// pub fn focused_text_input_id() -> Option<Uuid> {
//     TEXT_INPUT_ATOM
//         .get()
//         .last_focused_text_input
//         .as_ref()
//         .map(|text_input| text_input.id)
// }
// pub fn last_focus_requested_text_input_id() -> Option<Uuid> {
//     text_input_system().focus_requested_text_input_id
// }
// fn get_text_input_xy(rendering_tree: &RenderingTree, id: crate::Uuid) -> Option<Xy<Px>> {
//     let mut return_value = None;

//     rendering_tree.visit_rln(|rendering_tree, util| {
//         match rendering_tree {
//             RenderingTree::Special(special) => match special {
//                 render::SpecialRenderingNode::Custom(custom) => {
//                     if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>() {
//                         if custom_data.id == id {
//                             return_value = Some(util.get_xy());
//                             return ControlFlow::Break(());
//                         }
//                     }
//                 }
//                 _ => {}
//             },
//             _ => {}
//         };
//         ControlFlow::Continue(())
//     });

//     return_value
// }
// pub(crate) fn on_text_element_input(text: String) {
//     let atom = TEXT_INPUT_ATOM.get();
//     let Some(last_focused_text_input) = &atom.last_focused_text_input else {
//         return;
//     };

//     last_focused_text_input
//         .props
//         .event_handler
//         .as_ref()
//         .map(|event_handler| {
//             event_handler
//                 .on_text_updated
//                 .as_ref()
//                 .map(|on_text_updated| {
//                     on_text_updated.invoke(text.clone());
//                 })
//         });
// }
// pub(crate) fn on_selection_change(
//     selection_direction: SelectionDirection,
//     selection_start: usize,
//     selection_end: usize,
//     text: &str,
// ) {
//     let atom = TEXT_INPUT_ATOM.get();
//     let Some(last_focused_text_input) = &atom.last_focused_text_input else {
//         return;
//     };
//     let selection =
//         get_input_element_selection(selection_direction, selection_start, selection_end, text).map(
//             |range| {
//                 let chars_count = last_focused_text_input.props.text.chars().count();
//                 range.start.min(chars_count)..range.end.min(chars_count)
//             },
//         );

//     TEXT_INPUT_ATOM.mutate(move |text_input_ctx| {
//         text_input_ctx.selection = selection;
//     });
// }

// pub fn focus(text_input_id: crate::Uuid) {
//     web::execute_function_sync(
//         "
//         textArea.focus();
//     ",
//     )
//     .run::<()>();
//     text_input_system()
//         .focus_requested_text_input_id
//         .replace(text_input_id);
// }

// /// If you have a problem with blur not working, Make sure that you call blur on the composing text input
// pub fn blur() {
//     web::execute_function_sync(
//         "
//         textArea.blur();
//     ",
//     )
//     .run::<()>();

//     text_input_system_mutate(|text_input_system| {
//         text_input_system.focus_requested_text_input_id.take();
//         // text_input_system.last_focused_text_input.take();
//     });

//     TEXT_INPUT_ATOM.mutate(|x| {
//         x.last_focused_text_input = None;
//     });
// }

// pub(crate) fn get_input_element_selection(
//     selection_direction: SelectionDirection,
//     selection_start: usize,
//     selection_end: usize,
//     text: &str,
// ) -> Selection {
//     let utf16_code_unit_selection = {
//         if selection_direction == SelectionDirection::Backward {
//             selection_end..selection_start
//         } else {
//             selection_start..selection_end
//         }
//     };

//     Selection::from_utf16(Some(utf16_code_unit_selection), text)
// }

// fn get_input_element_selection_sync() -> Selection {
//     #[derive(serde::Deserialize)]
//     struct Output {
//         selection_direction: String,
//         selection_start: usize,
//         selection_end: usize,
//         text: String,
//     }
//     let output = web::execute_function_sync(
//         "
//         return {
//             selection_direction: textArea.selectionDirection,
//             selection_start: textArea.selectionStart,
//             selection_end: textArea.selectionEnd,
//             text: textArea.value,
//         };
//     ",
//     )
//     .run::<Output>();

//     get_input_element_selection(
//         SelectionDirection::try_from(output.selection_direction.as_str()).unwrap(),
//         output.selection_start,
//         output.selection_end,
//         &output.text,
//     )
// }

// // pub(crate) fn get_selection(id: crate::Uuid, text: &str) -> Selection {
// //     let text_input_system = text_input_system();
// //     let selection = &text_input_system.selection;
// //     let Selection::Range(range) = selection else {
// //         return Selection::None;
// //     };

// //     {
// //         let last_focused_text_input = &text_input_system.last_focused_text_input;

// //         if last_focused_text_input.is_none() {
// //             return Selection::None;
// //         }
// //         let last_focused_text_input = last_focused_text_input.as_ref().unwrap();

// //         if last_focused_text_input.id != id {
// //             return Selection::None;
// //         }
// //     }

// //     let chars_count = text.chars().count();
// //     Selection::Range(range.start.min(chars_count)..range.end.min(chars_count))
// // }
