mod event;

use self::event::set_up_event_handler;
use crate::system::InitResult;
use crate::*;
use std::sync::{Arc, RwLock};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
    #[cfg(target_family = "wasm")]
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
        #[cfg(target_family = "wasm")]
        let mouse_cursor = Arc::new(RwLock::new("default".to_string()));

        Self {
            mouse_position,
            #[cfg(target_family = "wasm")]
            mouse_cursor,
        }
    }
}

#[cfg(target_family = "wasm")]
pub fn set_mouse_cursor(cursor: &MouseCursor) {
    todo!()
}

pub fn position() -> Xy<Px> {
    *MOUSE_SYSTEM.mouse_position.read().unwrap()
}
