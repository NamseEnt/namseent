use crate::namui::render::text_input::*;
use crate::namui::*;
use std::{ops::ControlFlow, sync::Mutex};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::{Event, HtmlInputElement};

pub struct TextInputManager {
    last_focused_text_input_id: Mutex<Option<String>>,
    dragging_text_input_id: Mutex<Option<String>>,
}
const TEXT_INPUT_ELEMENT_ID: &str = "text-input";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(func: &::js_sys::Function, timeout: f32);
}

impl TextInputManager {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let element = document.create_element("input").unwrap();
        document.body().unwrap().append_child(&element).unwrap();

        let input_element = wasm_bindgen::JsCast::dyn_into::<HtmlInputElement>(element).unwrap();
        input_element.set_type("text");
        input_element.set_id(TEXT_INPUT_ELEMENT_ID);

        input_element
            .add_event_listener_with_callback(
                "input",
                Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
                    let target =
                        wasm_bindgen::JsCast::dyn_into::<HtmlInputElement>(event.target().unwrap())
                            .unwrap();
                    managers().text_input_manager.on_text_element_input(&target);
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
                    managers().text_input_manager.on_selection_change();
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
    fn get_input_element() -> HtmlInputElement {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.get_element_by_id(TEXT_INPUT_ELEMENT_ID).unwrap();
        wasm_bindgen::JsCast::dyn_into::<HtmlInputElement>(element).unwrap()
    }
    pub fn on_mouse_down(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        let input_element = Self::get_input_element();
        let mut last_focused_text_input_id = self.last_focused_text_input_id.lock().unwrap();

        let custom_data =
            Self::find_front_text_input_on_mouse(&namui_context.rendering_tree, raw_mouse_event);

        *self.dragging_text_input_id.lock().unwrap() = custom_data
            .as_ref()
            .map(|custom_data| custom_data.text_input.id.clone())
            .clone();

        if let Some(last_focused_text_input_id) = &*last_focused_text_input_id {
            let is_last_focused_text_input_not_clicked = custom_data
                .as_ref()
                .and_then(|custom_data| {
                    last_focused_text_input_id
                        .eq(&custom_data.text_input.id)
                        .then(|| ())
                })
                .is_none();
            if is_last_focused_text_input_not_clicked {
                crate::event::send(text_input::Event::Blur(text_input::Blur {
                    id: last_focused_text_input_id.clone(),
                }));
            }
        }

        *last_focused_text_input_id = custom_data
            .as_ref()
            .map(|custom_data| custom_data.text_input.id.clone());

        if custom_data.is_none() {
            input_element.blur().unwrap();
            return;
        }
        let custom_data = custom_data.unwrap();

        Self::update_focus_with_mouse_movement(
            &custom_data,
            input_element,
            raw_mouse_event.xy.x,
            false,
        );
    }
    pub fn on_mouse_move(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        let dragging_text_input_id = self.dragging_text_input_id.lock().unwrap();
        if dragging_text_input_id.is_none() {
            return;
        }
        let dragging_text_input_id = dragging_text_input_id.as_ref().unwrap();

        let custom_data =
            Self::find_text_input_by_id(&namui_context.rendering_tree, dragging_text_input_id);
        if custom_data.is_none() {
            return;
        }
        let custom_data = custom_data.unwrap();

        Self::update_focus_with_mouse_movement(
            &custom_data,
            Self::get_input_element(),
            raw_mouse_event.xy.x,
            true,
        );
    }
    pub fn on_mouse_up(&self, _namui_context: &NamuiContext, _raw_mouse_event: &RawMouseEvent) {
        *self.dragging_text_input_id.lock().unwrap() = None;
    }
    pub fn is_focused(&self, text_input_id: &str) -> bool {
        let last_focused_text_input_id = self.last_focused_text_input_id.lock().unwrap();
        last_focused_text_input_id
            .as_ref()
            .map(|id| id.eq(text_input_id))
            .unwrap_or(false)
    }
    fn update_focus_with_mouse_movement(
        custom_data: &TextInputCustomData,
        input_element: HtmlInputElement,
        mouse_x: f32,
        is_mouse_move: bool,
    ) {
        let selection = custom_data.text_input.get_selection_on_mouse_movement(
            &custom_data.props,
            mouse_x,
            is_mouse_move,
        );
        let selection_direction = match &selection {
            Some(selection) => {
                if selection.start <= selection.end {
                    "forward"
                } else {
                    "backward"
                }
            }
            None => "none",
        };

        input_element.set_value(&custom_data.props.text_param.text);
        input_element
            .set_selection_range_with_direction(
                selection
                    .as_ref()
                    .map_or(0, |selection| selection.start.min(selection.end) as u32),
                selection
                    .as_ref()
                    .map_or(0, |selection| selection.start.max(selection.end) as u32),
                selection_direction,
            )
            .unwrap();

        input_element.focus().unwrap();

        let event = text_input::Event::Focus(Focus {
            id: custom_data.text_input.id.clone(),
            selection,
        });
        crate::event::send(event);
    }
    fn find_text_input_by_id(
        rendering_tree: &RenderingTree,
        id: &str,
    ) -> Option<TextInputCustomData> {
        let mut return_value: Option<TextInputCustomData> = None;

        rendering_tree.visit_rln(|rendering_tree, _| {
            match rendering_tree {
                RenderingTree::Special(special) => match special {
                    render::SpecialRenderingNode::Custom(custom) => {
                        if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>()
                        {
                            if custom_data.text_input.id == id {
                                return_value = Some(custom_data.clone());
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
    fn find_front_text_input_on_mouse(
        rendering_tree: &RenderingTree,
        raw_mouse_event: &RawMouseEvent,
    ) -> Option<TextInputCustomData> {
        let mut return_value: Option<TextInputCustomData> = None;

        rendering_tree.visit_rln(|rendering_tree, utils| {
            match rendering_tree {
                RenderingTree::Special(special) => match special {
                    render::SpecialRenderingNode::Custom(custom) => {
                        if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>()
                        {
                            let is_custom_in_mouse = utils.is_xy_in(&raw_mouse_event.xy);

                            if is_custom_in_mouse {
                                return_value = Some(custom_data.clone());
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
    fn on_text_element_input(&self, input_element: &HtmlInputElement) {
        let text = input_element.value();
        let last_focused_text_input_id = self.last_focused_text_input_id.lock().unwrap();
        if last_focused_text_input_id.is_none() {
            return;
        }
        let last_focused_text_input_id = last_focused_text_input_id.as_ref().unwrap();
        let selection = Self::get_selection(&input_element);

        crate::event::send(text_input::Event::TextUpdated(TextUpdated {
            id: last_focused_text_input_id.clone(),
            text: text.to_string(),
            selection,
        }))
    }
    fn get_selection(input_element: &HtmlInputElement) -> text_input::Selection {
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

    fn on_selection_change(&self) {
        let text_input_id = self.last_focused_text_input_id.lock().unwrap().clone();
        if text_input_id.is_none() {
            return;
        }
        let text_input_id = text_input_id.unwrap();

        let input_element = Self::get_input_element();

        let selection = Self::get_selection(&input_element);

        crate::event::send(text_input::Event::SelectionUpdated(
            text_input::SelectionUpdated {
                id: text_input_id,
                selection,
            },
        ));
    }
}
