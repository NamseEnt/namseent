pub mod common;
use std::sync::Arc;

pub use common::*;

#[cfg(target_family = "wasm")]
pub mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

pub struct Managers {
    pub mouse_manager: Box<MouseManager>,
    pub font_manager: Box<FontManager>,
    pub keyboard_manager: Box<KeyboardManager>,
    pub screen_manager: Box<ScreenManager>,
    pub image_manager: Arc<ImageManager>,
}
