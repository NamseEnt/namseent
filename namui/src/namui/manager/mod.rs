pub mod common;
use std::sync::Arc;

pub use common::*;

#[cfg(not(test))]
#[cfg(target_family = "wasm")]
pub mod web;
#[cfg(not(test))]
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(test)]
pub mod mock;
#[cfg(test)]
pub use mock::*;

pub struct Managers {
    pub mouse_manager: Box<MouseManager>,
    pub font_manager: Box<FontManager>,
    pub keyboard_manager: Box<KeyboardManager>,
    pub screen_manager: Box<ScreenManager>,
    pub image_manager: Arc<ImageManager>,
    pub wheel_manager: Box<WheelManager>,
}
