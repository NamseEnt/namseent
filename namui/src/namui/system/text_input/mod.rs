mod find;
mod key_down;
mod mouse_event;
mod post_render;
mod selection;

use super::InitResult;
use crate::namui::render::text_input::*;
use crate::namui::*;
pub(crate) use find::*;
pub(crate) use key_down::*;
pub(crate) use mouse_event::*;
pub(crate) use post_render::*;
pub use selection::*;
use std::str::FromStr;
use std::{ops::ControlFlow, sync::Mutex};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, HtmlTextAreaElement};

struct TextInputSystem {
    last_focused_text_input: Mutex<Option<TextInputCustomData>>,
    dragging_text_input: Mutex<Option<TextInputCustomData>>,
    focus_requested_text_input_id: Mutex<Option<Uuid>>,
}
const TEXT_INPUT_ELEMENT_ID: &str = "text-input";

lazy_static::lazy_static! {
    static ref TEXT_INPUT_SYSTEM: Arc<TextInputSystem> = Arc::new(TextInputSystem::new());
}

pub(super) async fn init() -> InitResult {
    lazy_static::initialize(&TEXT_INPUT_SYSTEM);
    Ok(())
}

impl TextInputSystem {
    fn new() -> Self {
        // let document = web_sys::window().unwrap().document().unwrap();

        // let element = document.create_element("textarea").unwrap();
        // document.body().unwrap().append_child(&element).unwrap();

        // let input_element = wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(element).unwrap();
        // input_element.set_id(TEXT_INPUT_ELEMENT_ID);

        // input_element
        //     .add_event_listener_with_callback(
        //         "input",
        //         Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
        //             let target = wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(
        //                 event.target().unwrap(),
        //             )
        //             .unwrap();

        //             system::text_input::on_text_element_input(&target);
        //         }) as Box<dyn FnMut(_)>)
        //         .into_js_value()
        //         .unchecked_ref(),
        //     )
        //     .unwrap();

        // input_element
        //     .add_event_listener_with_callback(
        //         "keydown",
        //         Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        //             event.stop_immediate_propagation();
        //             let code = Code::from_str(&event.code()).unwrap();
        //             crate::keyboard::record_key_down(code);

        //             // NOTE: Not support page up/down yet.
        //             if [
        //                 Code::ArrowUp,
        //                 Code::ArrowDown,
        //                 Code::Home,
        //                 Code::End,
        //                 Code::PageUp,
        //                 Code::PageDown,
        //             ]
        //             .contains(&code)
        //             {
        //                 event.prevent_default();
        //             }
        //             on_key_down(code, event);
        //         }) as Box<dyn FnMut(_)>)
        //         .into_js_value()
        //         .unchecked_ref(),
        //     )
        //     .unwrap();

        // {
        //     // NOTE: Below codes from https://github.com/goldfire/CanvasInput/blob/5adbaf00bd42665f3c691796881c7a7a9cf7036c/CanvasInput.js#L126
        //     let style = input_element.style();
        //     style.set_property("position", "absolute").unwrap();
        //     style.set_property("opacity", "0").unwrap();
        //     style.set_property("pointerEvents", "none").unwrap();
        //     style.set_property("zIndex", "0").unwrap();
        //     style.set_property("top", "0px").unwrap();
        //     // hide native blue text cursor on iOS
        //     style.set_property("transform", "scale(0)").unwrap();
        // }

        // document
        //     .add_event_listener_with_callback(
        //         "selectionchange",
        //         Closure::wrap(Box::new(move |_: Event| {
        //             on_selection_change();
        //         }) as Box<dyn FnMut(_)>)
        //         .into_js_value()
        //         .unchecked_ref(),
        //     )
        //     .unwrap();

        Self {
            last_focused_text_input: Mutex::new(None),
            dragging_text_input: Mutex::new(None),
            focus_requested_text_input_id: Mutex::new(None),
        }
    }
}
fn get_input_element() -> HtmlTextAreaElement {
    todo!()
    // let document = web_sys::window().unwrap().document().unwrap();
    // let element = document.get_element_by_id(TEXT_INPUT_ELEMENT_ID).unwrap();
    // wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(element).unwrap()
}
pub fn is_focused(text_input_id: crate::Uuid) -> bool {
    let last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();
    last_focused_text_input
        .as_ref()
        .map(|text_input| text_input.id == (text_input_id))
        .unwrap_or(false)
}
pub fn focused_text_input_id() -> Option<Uuid> {
    TEXT_INPUT_SYSTEM
        .last_focused_text_input
        .lock()
        .unwrap()
        .as_ref()
        .map(|text_input| text_input.id)
}
pub fn last_focus_requested_text_input_id() -> Option<Uuid> {
    TEXT_INPUT_SYSTEM
        .focus_requested_text_input_id
        .lock()
        .unwrap()
        .clone()
}
fn get_text_input_xy(rendering_tree: &RenderingTree, id: crate::Uuid) -> Option<Xy<Px>> {
    let mut return_value = None;

    rendering_tree.visit_rln(|rendering_tree, util| {
        match rendering_tree {
            RenderingTree::Special(special) => match special {
                render::SpecialRenderingNode::Custom(custom) => {
                    if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>() {
                        if custom_data.id == id {
                            return_value = Some(util.get_xy());
                            return ControlFlow::Break(());
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        };
        ControlFlow::Continue(())
    });

    return_value
}
fn on_text_element_input(input_element: &HtmlTextAreaElement) {
    let text = input_element.value();
    let last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();
    if last_focused_text_input.is_none() {
        return;
    }
    let last_focused_text_input = last_focused_text_input.as_ref().unwrap();

    last_focused_text_input
        .props
        .event_handler
        .as_ref()
        .map(|event_handler| {
            event_handler
                .on_text_updated
                .as_ref()
                .map(|on_text_updated| {
                    on_text_updated.invoke(text.clone());
                })
        });

    crate::event::send(text_input::Event::TextUpdated {
        id: last_focused_text_input.id.clone(),
        text,
    })
}
fn on_selection_change() {
    let last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();
    if last_focused_text_input.is_none() {
        return;
    }
    let last_focused_text_input = last_focused_text_input.as_ref().unwrap();

    let input_element = get_input_element();
    let selection = get_input_element_selection(&input_element).map(|selection| {
        let chars_count = last_focused_text_input.props.text.chars().count();
        selection.start.min(chars_count)..selection.end.min(chars_count)
    });

    crate::event::send(text_input::Event::SelectionUpdated {
        id: last_focused_text_input.id.clone(),
        selection,
    });
}

pub fn focus(text_input_id: crate::Uuid) {
    let input_element = get_input_element();
    input_element.focus().unwrap();
    TEXT_INPUT_SYSTEM
        .focus_requested_text_input_id
        .lock()
        .unwrap()
        .replace(text_input_id);
}

/// If you have a problem with blur not working, Make sure that you call blur on the composing text input
pub fn blur() {
    let input_element = get_input_element();
    input_element.blur().unwrap();
    TEXT_INPUT_SYSTEM
        .focus_requested_text_input_id
        .lock()
        .unwrap()
        .take();
    TEXT_INPUT_SYSTEM
        .last_focused_text_input
        .lock()
        .unwrap()
        .take();
}

fn get_input_element_selection(input_element: &HtmlTextAreaElement) -> Selection {
    let utf16_code_unit_selection = {
        let selection_start = input_element.selection_start().unwrap();
        if selection_start.is_none() {
            None
        } else {
            let selection_start = selection_start.unwrap() as usize;
            let selection_end = input_element.selection_end().unwrap().unwrap() as usize;
            let selection_direction = input_element.selection_direction().unwrap().unwrap();

            if selection_direction.eq("backward") {
                Some(selection_end..selection_start)
            } else {
                Some(selection_start..selection_end)
            }
        }
    };

    Selection::from_utf16(utf16_code_unit_selection, input_element.value().as_str())
}

pub(crate) fn get_selection(id: crate::Uuid, text: &str) -> Selection {
    let input_element = get_input_element();
    let selection = get_input_element_selection(&input_element);
    let Selection::Range(range) = selection else {
        return Selection::None;
    };

    {
        let last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();

        if last_focused_text_input.is_none() {
            return Selection::None;
        }
        let last_focused_text_input = last_focused_text_input.as_ref().unwrap();

        if last_focused_text_input.id != id {
            return Selection::None;
        }
    }

    let chars_count = text.chars().count();
    Selection::Range(range.start.min(chars_count)..range.end.min(chars_count))
}
