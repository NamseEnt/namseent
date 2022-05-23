use crate::namui::render::text_input::*;
use crate::namui::*;
use std::{
    ops::{ControlFlow, Range},
    sync::Mutex,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlInputElement;

pub struct TextInputManager {
    last_focused_text_input_id: Mutex<Option<String>>,
}
const TEXT_INPUT_ELEMENT_ID: &str = "text-input";

impl TextInputManager {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let element = document.create_element("input").unwrap();
        document.body().unwrap().append_child(&element).unwrap();

        let input_element = wasm_bindgen::JsCast::dyn_into::<HtmlInputElement>(element).unwrap();
        input_element.set_type("text");
        input_element.set_id(TEXT_INPUT_ELEMENT_ID);

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

        Self {
            last_focused_text_input_id: Mutex::new(None),
        }
    }
    pub fn on_mouse_down(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        let mut last_focused_text_input_id = self.last_focused_text_input_id.lock().unwrap();

        let custom_data =
            self.find_front_text_input_on_mouse(&namui_context.rendering_tree, raw_mouse_event);
        if custom_data.is_none() {
            crate::log!("No text input found on mouse down");
            last_focused_text_input_id.take();
            crate::event::send(TextInputEvent::Blur);
            return;
        }
        crate::log!("Text input found on mouse down");
        let custom_data = custom_data.unwrap();

        last_focused_text_input_id.replace(custom_data.text_input.id.clone());

        let selection = custom_data
            .text_input
            .get_selection_on_mouse_down(&custom_data.props, raw_mouse_event.xy.x);
        crate::log!("Selection: {:?}", selection);

        // 어떻게 업데이트 할 것인지.

        let event = TextInputEvent::Focus(TextInputFocus {
            id: custom_data.text_input.id.clone(),
            selection,
        });
        crate::event::send(event);
    }
    pub fn on_mouse_move(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }
    pub fn on_mouse_up(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }
    fn find_front_text_input_on_mouse(
        &self,
        rendering_tree: &RenderingTree,
        raw_mouse_event: &RawMouseEvent,
    ) -> Option<TextInputCustomData> {
        let mut return_value: Option<TextInputCustomData> = None;

        rendering_tree.visit_rln(|rendering_tree, utils| match rendering_tree {
            RenderingTree::Special(special) => match special {
                render::SpecialRenderingNode::Custom(custom) => {
                    if let Some(custom_data) = custom.data.downcast_ref::<TextInputCustomData>() {
                        let is_custom_in_mouse = (utils.is_xy_in)(&raw_mouse_event.xy);

                        if is_custom_in_mouse {
                            return_value = Some(custom_data.clone());
                            return ControlFlow::Break(());
                        }
                    }
                    ControlFlow::Continue(())
                }
                _ => ControlFlow::Continue(()),
            },
            _ => ControlFlow::Continue(()),
        });

        return_value
    }
}
