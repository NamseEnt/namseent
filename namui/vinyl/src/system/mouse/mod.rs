mod cursor;
#[cfg(not(target_os = "wasi"))]
mod non_wasm;
#[cfg(target_os = "wasi")]
mod wasm;

#[cfg(not(target_os = "wasi"))]
pub(crate) use non_wasm::*;
#[cfg(target_os = "wasi")]
pub(crate) use wasm::*;

use crate::system::InitResult;
use crate::*;
#[cfg(not(target_os = "wasi"))]
use std::collections::HashSet;
use std::sync::{Arc, OnceLock, RwLock};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
    _mouse_cursor: Arc<RwLock<String>>,
    #[cfg(not(target_os = "wasi"))]
    pressing_buttons: Arc<RwLock<HashSet<MouseButton>>>,
}

lazy_static::lazy_static! {
    static ref MOUSE_SYSTEM: Arc<MouseSystem> = Arc::new(MouseSystem::new());
}

static STANDARD_CURSOR_SPRITE_SET: OnceLock<StandardCursorSpriteSet> = OnceLock::new();

pub(crate) async fn init() -> InitResult {
    lazy_static::initialize(&MOUSE_SYSTEM);
    STANDARD_CURSOR_SPRITE_SET
        .set(cursor::load_default_cursor_set().await?)
        .map_err(|_| anyhow::anyhow!("STANDARD_CURSOR_SPRITE_SET already initialized"))?;

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
            #[cfg(not(target_os = "wasi"))]
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

pub(crate) fn standard_cursor_sprite_set() -> &'static StandardCursorSpriteSet {
    STANDARD_CURSOR_SPRITE_SET.get().unwrap()
}
