mod event;
mod post_render;

use self::event::set_up_event_handler;
use super::*;
use crate::{
    namui::{render::MouseCursor, Xy},
    *,
};
pub(crate) use post_render::*;
use std::sync::{Arc, RwLock};
use std::{collections::HashSet, sync::Mutex};
use wasm_bindgen::{prelude::Closure, JsCast};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
    rendering_tree: Arc<Mutex<RenderingTree>>,
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
        let mouse = Self {
            mouse_position: mouse_position.clone(),
            rendering_tree: Arc::new(Mutex::new(RenderingTree::Empty)),
        };

        mouse
    }
}

pub fn set_mouse_cursor(cursor: &MouseCursor) {
    let element = document().body().unwrap();
    element
        .style()
        .set_property("cursor", &cursor.to_css_cursor_value())
        .unwrap();
}

pub fn position() -> Xy<Px> {
    MOUSE_SYSTEM.mouse_position.read().unwrap().clone()
}

impl MouseCursor {
    pub fn to_css_cursor_value(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::TopBottomResize => "ns-resize",
            Self::LeftRightResize => "ew-resize",
            Self::LeftTopRightBottomResize => "nwse-resize",
            Self::RightTopLeftBottomResize => "nesw-resize",
            Self::Text => "text",
            Self::Grab => "grab",
            Self::Move => "move",
            Self::Pointer => "pointer",
            MouseCursor::Custom(_) => "none",
        }
    }
}
