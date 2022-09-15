mod find;
mod key_down;
mod mouse_event;

use super::InitResult;
use crate::namui::*;
use crate::namui::{namui_context::NamuiContext, render::text_input::*};
pub(crate) use find::*;
pub(crate) use key_down::*;
pub(crate) use mouse_event::*;
use std::str::FromStr;
use std::{ops::ControlFlow, sync::Mutex};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, HtmlTextAreaElement};

struct TextInputSystem {
    last_focused_text_input_id: Mutex<Option<String>>,
    dragging_text_input_id: Mutex<Option<String>>,
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
        let document = web_sys::window().unwrap().document().unwrap();

        let element = document.create_element("textarea").unwrap();
        document.body().unwrap().append_child(&element).unwrap();

        let input_element = wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(element).unwrap();
        input_element.set_id(TEXT_INPUT_ELEMENT_ID);

        input_element
            .add_event_listener_with_callback(
                "input",
                Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
                    let target = wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(
                        event.target().unwrap(),
                    )
                    .unwrap();
                    system::text_input::on_text_element_input(&target);
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        input_element
            .add_event_listener_with_callback(
                "keydown",
                Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                    let code = Code::from_str(&event.code()).unwrap();
                    // NOTE: Not support page up/down yet.
                    if [
                        Code::ArrowUp,
                        Code::ArrowDown,
                        Code::Home,
                        Code::End,
                        Code::PageUp,
                        Code::PageDown,
                    ]
                    .contains(&code)
                    {
                        event.prevent_default();
                    }
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        {
            // NOTE: Below codes from https://github.com/goldfire/CanvasInput/blob/5adbaf00bd42665f3c691796881c7a7a9cf7036c/CanvasInput.js#L126
            let style = input_element.style();
            style.set_property("position", "absolute").unwrap();
            style.set_property("opacity", "0").unwrap();
            style.set_property("pointerEvents", "none").unwrap();
            style.set_property("zIndex", "0").unwrap();
            style.set_property("top", "0px").unwrap();
            // hide native blue text cursor on iOS
            style.set_property("transform", "scale(0)").unwrap();
        }

        document
            .add_event_listener_with_callback(
                "selectionchange",
                Closure::wrap(Box::new(move |_: Event| {
                    on_selection_change();
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        Self {
            last_focused_text_input_id: Mutex::new(None),
            dragging_text_input_id: Mutex::new(None),
        }
    }
}
fn get_input_element() -> HtmlTextAreaElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.get_element_by_id(TEXT_INPUT_ELEMENT_ID).unwrap();
    wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(element).unwrap()
}
pub fn is_focused(text_input_id: &str) -> bool {
    let last_focused_text_input_id = TEXT_INPUT_SYSTEM.last_focused_text_input_id.lock().unwrap();
    last_focused_text_input_id
        .as_ref()
        .map(|id| id.eq(text_input_id))
        .unwrap_or(false)
}
fn get_text_input_xy(rendering_tree: &RenderingTree, id: &str) -> Option<Xy<Px>> {
    let mut return_value = None;

    rendering_tree.visit_rln(|rendering_tree, util| {
        match rendering_tree {
            RenderingTree::Special(special) => match special {
                render::SpecialRenderingNode::Custom(custom) => {
                    if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>() {
                        if custom_data.text_input.id == id {
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
    let last_focused_text_input_id = TEXT_INPUT_SYSTEM.last_focused_text_input_id.lock().unwrap();
    if last_focused_text_input_id.is_none() {
        return;
    }
    let last_focused_text_input_id = last_focused_text_input_id.as_ref().unwrap();
    let selection = get_selection(&input_element);

    crate::event::send(text_input::Event::TextUpdated {
        id: last_focused_text_input_id.clone(),
        text: text.to_string(),
        selection,
    })
}
fn get_selection(input_element: &HtmlTextAreaElement) -> text_input::Selection {
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
}

fn on_selection_change() {
    let text_input_id = TEXT_INPUT_SYSTEM
        .last_focused_text_input_id
        .lock()
        .unwrap()
        .clone();
    if text_input_id.is_none() {
        return;
    }
    let text_input_id = text_input_id.unwrap();

    let input_element = get_input_element();

    let selection = get_selection(&input_element);

    crate::event::send(text_input::Event::SelectionUpdated {
        id: text_input_id,
        selection,
    });
}

pub fn focus(text_input_id: &str) {
    crate::log!("Focus");
    let input_element = get_input_element();
    input_element.focus().unwrap();
    TEXT_INPUT_SYSTEM
        .last_focused_text_input_id
        .lock()
        .unwrap()
        .replace(text_input_id.to_string());
}
