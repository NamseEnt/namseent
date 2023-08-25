mod event;

use self::event::set_up_event_handler;
use super::*;
use crate::*;
use std::collections::HashSet;
use std::ops::ControlFlow;
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
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
        };

        mouse
    }
}

pub(crate) fn update_mouse_cursor(rendering_tree: &RenderingTree) {
    let mouse_position = position();
    let mut cursor = MouseCursor::Default;

    rendering_tree.visit_rln(|node, utils| {
        if let RenderingTree::Special(special) = node {
            if let SpecialRenderingNode::MouseCursor(mouse_cursor) = special {
                if Visit::xy_in(node, mouse_position, utils.ancestors) {
                    cursor = (*mouse_cursor.cursor).clone();
                    return ControlFlow::Break(());
                }
            };
        };
        ControlFlow::Continue(())
    });

    let element = document().body().unwrap();
    element
        .style()
        .set_property("cursor", &cursor.to_css_cursor_value())
        .unwrap();
}

pub fn position() -> Xy<Px> {
    MOUSE_SYSTEM.mouse_position.read().unwrap().clone()
}
