mod event;

use self::event::set_up_event_handler;
use super::*;
use crate::*;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
    mouse_cursor: Arc<RwLock<String>>,
}

lazy_static::lazy_static! {
    static ref MOUSE_SYSTEM: Arc<MouseSystem> = Arc::new(MouseSystem::new());
}

pub(crate) async fn init() -> InitResult {
    lazy_static::initialize(&MOUSE_SYSTEM);
    set_up_event_handler();
    Ok(())
}

impl MouseSystem {
    fn new() -> Self {
        let mouse_position = Arc::new(RwLock::new(Xy::<Px> {
            x: px(0.0),
            y: px(0.0),
        }));
        let mouse_cursor = Arc::new(RwLock::new("default".to_string()));
        let mouse = Self {
            mouse_position,
            mouse_cursor,
        };

        mouse
    }
}

pub fn set_mouse_cursor(cursor: &MouseCursor) {
    let cursor = cursor.to_css_cursor_value();
    let cursor_changed = { MOUSE_SYSTEM.mouse_cursor.read().unwrap().ne(cursor) };
    if !cursor_changed {
        return;
    }
    let element = document().body().unwrap();
    element.style().set_property("cursor", cursor).unwrap();
    *MOUSE_SYSTEM.mouse_cursor.write().unwrap() = cursor.to_string();
}

pub fn position() -> Xy<Px> {
    MOUSE_SYSTEM.mouse_position.read().unwrap().clone()
}
