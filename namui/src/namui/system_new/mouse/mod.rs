pub(crate) mod event;

use self::event::set_up_event_handler;
use super::*;
use crate::*;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

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

pub fn set_mouse_cursor(cursor: &MouseCursor) {
    todo!()
    // let element = document().body().unwrap();
    // element
    //     .style()
    //     .set_property("cursor", &cursor.to_css_cursor_value())
    //     .unwrap();
}

pub fn position() -> Xy<Px> {
    MOUSE_SYSTEM.mouse_position.read().unwrap().clone()
}
