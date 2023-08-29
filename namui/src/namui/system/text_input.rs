use super::{platform_utils::web::document, InitResult};
use crate::{SelectionDirection, *};
use std::{
    str::FromStr,
    sync::{Arc, OnceLock},
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlTextAreaElement;

#[derive(Debug)]
struct TextArea {
    element: HtmlTextAreaElement,
}

unsafe impl Send for TextArea {}
unsafe impl Sync for TextArea {}

static TEXT_AREA: OnceLock<Arc<TextArea>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    let element = document()
        .create_element("textarea")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap();

    document().body().unwrap().append_child(&element).unwrap();

    let text_area = Arc::new(TextArea { element });
    TEXT_AREA.set(text_area.clone()).unwrap();

    {
        // NOTE: Below codes from https://github.com/goldfire/CanvasInput/blob/5adbaf00bd42665f3c691796881c7a7a9cf7036c/CanvasInput.js#L126
        let style = text_area.element.style();
        style.set_property("position", "absolute").unwrap();
        style.set_property("opacity", "0").unwrap();
        style.set_property("pointerEvents", "none").unwrap();
        style.set_property("zIndex", "0").unwrap();
        style.set_property("top", "0px").unwrap();
        // hide native blue text cursor on iOS
        style.set_property("transform", "scale(0)").unwrap();
    }

    text_area
        .element
        .add_event_listener_with_callback(
            "input",
            Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
                let target =
                    wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(event.target().unwrap())
                        .unwrap();

                crate::hooks::on_raw_event(RawEvent::TextInputTextUpdated {
                    text: target.value(),
                    selection_direction: {
                        let js_selection_direction = target.selection_direction().unwrap();
                        if let Some(js_selection_direction) = js_selection_direction {
                            if js_selection_direction == "forward" {
                                SelectionDirection::Forward
                            } else if js_selection_direction == "backward" {
                                SelectionDirection::Backward
                            } else {
                                SelectionDirection::None
                            }
                        } else {
                            SelectionDirection::None
                        }
                    },
                    selection_start: target.selection_start().unwrap().unwrap_or_default() as usize,
                    selection_end: target.selection_end().unwrap().unwrap_or_default() as usize,
                })
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    text_area
        .element
        .add_event_listener_with_callback(
            "keydown",
            Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let target =
                    wasm_bindgen::JsCast::dyn_into::<HtmlTextAreaElement>(event.target().unwrap())
                        .unwrap();

                event.stop_immediate_propagation();
                let code = Code::from_str(&event.code()).unwrap();
                crate::keyboard::record_key_down(code);

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

                crate::hooks::on_raw_event(RawEvent::TextInputKeyDown {
                    event: RawTextinputKeyDownEvent {
                        code,
                        text: target.value(),
                        selection_direction: {
                            let js_selection_direction = target.selection_direction().unwrap();
                            if let Some(js_selection_direction) = js_selection_direction {
                                if js_selection_direction == "forward" {
                                    SelectionDirection::Forward
                                } else if js_selection_direction == "backward" {
                                    SelectionDirection::Backward
                                } else {
                                    SelectionDirection::None
                                }
                            } else {
                                SelectionDirection::None
                            }
                        },
                        selection_start: target.selection_start().unwrap().unwrap_or_default()
                            as usize,
                        selection_end: target.selection_end().unwrap().unwrap_or_default() as usize,
                        is_composing: event.is_composing(),
                        prevent_default: Box::new(move || {
                            event.prevent_default();
                        }),
                    },
                })
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    document()
        .add_event_listener_with_callback("selectionchange", {
            let text_area = text_area.clone();
            Closure::wrap(Box::new(move |_: web_sys::Event| {
                crate::hooks::on_raw_event(RawEvent::SelectionChange {
                    text: text_area.element.value(),
                    selection_direction: {
                        let js_selection_direction =
                            text_area.element.selection_direction().unwrap();
                        if let Some(js_selection_direction) = js_selection_direction {
                            if js_selection_direction == "forward" {
                                SelectionDirection::Forward
                            } else if js_selection_direction == "backward" {
                                SelectionDirection::Backward
                            } else {
                                SelectionDirection::None
                            }
                        } else {
                            SelectionDirection::None
                        }
                    },
                    selection_start: text_area
                        .element
                        .selection_start()
                        .unwrap()
                        .unwrap_or_default() as usize,
                    selection_end: text_area
                        .element
                        .selection_end()
                        .unwrap()
                        .unwrap_or_default() as usize,
                });
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref()
        })
        .unwrap();

    Ok(())
}

fn element() -> &'static HtmlTextAreaElement {
    &TEXT_AREA.get().unwrap().element
}

pub(crate) fn set_width(width: Px) {
    element()
        .style()
        .set_property("width", &format!("{}px", width.as_f32()))
        .unwrap();
}

pub(crate) fn set_value(text: &str) {
    element().set_value(text);
}

pub(crate) fn set_selection_range(
    selection_start: usize,
    selection_end: usize,
    selection_direction: SelectionDirection,
) {
    element()
        .set_selection_range_with_direction(
            selection_start as u32,
            selection_end as u32,
            match selection_direction {
                SelectionDirection::Forward => "forward",
                SelectionDirection::Backward => "backward",
                SelectionDirection::None => "none",
            },
        )
        .unwrap();
}

pub(crate) fn focus() {
    element().focus().unwrap();
}
