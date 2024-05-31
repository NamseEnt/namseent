#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

use crate::system::InitResult;
use crate::*;
#[cfg(not(target_os = "wasi"))]
pub(crate) use non_wasm::*;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
#[cfg(target_os = "wasi")]
pub use wasm::*;

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
    _mouse_cursor: Arc<RwLock<String>>,
}

lazy_static::lazy_static! {
    static ref MOUSE_SYSTEM: Arc<MouseSystem> = Arc::new(MouseSystem::new());
}

pub(crate) async fn init() -> InitResult {
    lazy_static::initialize(&MOUSE_SYSTEM);

    Ok(())
}

impl MouseSystem {
    fn new() -> Self {
        Self {
            mouse_position: Arc::new(RwLock::new(Xy::<Px> {
                x: px(0.0),
                y: px(0.0),
            })),
            _mouse_cursor: Arc::new(RwLock::new("default".to_string())),
            #[cfg(not(target_family = "wasm"))]
            pressing_buttons: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

pub fn set_mouse_cursor(_cursor: &MouseCursor) {
    todo!()
}

pub fn position() -> Xy<Px> {
    *MOUSE_SYSTEM.mouse_position.read().unwrap()
}
