use crate::namui::{self, namui_state::NamuiState, render::MouseCursor, NamuiInternal, Xy};
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;

pub struct MouseManager {}

impl MouseManager {
    pub fn new(element: &HtmlElement) -> Self {
        Self {}
    }
    pub fn mouse_position(&self) -> Xy<i16> {
        Xy { x: 0, y: 0 }
    }
    pub fn set_mouse_cursor(&self, cursor: &MouseCursor) {}
}
