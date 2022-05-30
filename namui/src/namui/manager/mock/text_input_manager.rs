use crate::namui::*;
use wasm_bindgen::{prelude::Closure, JsCast};

pub struct TextInputManager {}

impl TextInputManager {
    pub fn new() -> Self {
        Self {}
    }
    pub fn on_mouse_down(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }

    pub fn on_mouse_move(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }

    pub fn on_mouse_up(&self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }
    pub fn is_focused(&self, text_input_id: &str) -> bool {
        todo!();
    }
}
