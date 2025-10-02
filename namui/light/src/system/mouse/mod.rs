mod cursor;
mod wasm;

use crate::*;
use std::sync::{Arc, OnceLock, RwLock};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<Px>>>,
    _mouse_cursor: Arc<RwLock<String>>,
}

lazy_static::lazy_static! {
    static ref MOUSE_SYSTEM: Arc<MouseSystem> = Arc::new(MouseSystem::new());
}

static STANDARD_CURSOR_SPRITE_SET: OnceLock<StandardCursorSpriteSet> = OnceLock::new();

pub(crate) async fn init() -> Result<()> {
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
