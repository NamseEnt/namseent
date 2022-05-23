use crate::namui::*;
use wasm_bindgen::{prelude::Closure, JsCast};

pub struct TextInputManager {}

impl TextInputManager {
    pub fn new() -> Self {
        Self {}
    }
    pub fn on_mouse_down(&mut self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }

    pub fn on_mouse_move(&mut self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }

    pub fn on_mouse_up(&mut self, namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
        todo!()
    }
}
