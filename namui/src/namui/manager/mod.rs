pub mod common;
pub use common::*;
use parking_lot::{ReentrantMutex, ReentrantMutexGuard};
use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::{ops::DerefMut, sync::Arc};

// #[cfg(not(test))]
#[cfg(target_family = "wasm")]
pub mod web;
// #[cfg(not(test))]
#[cfg(target_family = "wasm")]
pub use web::*;

// #[cfg(test)]
// pub mod mock;
// #[cfg(test)]
// pub use mock::*;

pub struct Managers {
    pub mouse_manager: Box<MouseManager>,
    pub font_manager: Box<FontManager>,
    pub keyboard_manager: Box<KeyboardManager>,
    pub screen_manager: Box<ScreenManager>,
    pub image_manager: Arc<ImageManager>,
    pub wheel_manager: Box<WheelManager>,
    pub(crate) text_input_manager: Box<TextInputManager>,
}

use once_cell::sync::OnceCell;
pub(crate) static MANAGERS: OnceCell<ReentrantMutex<RefCell<Managers>>> = OnceCell::new();

pub struct ManagersMutexGuard<'a> {
    guard: ReentrantMutexGuard<'a, RefCell<Managers>>,
}

pub fn managers<'a>() -> ManagersMutexGuard<'a> {
    ManagersMutexGuard {
        guard: MANAGERS.get().expect("managers not initialized").lock(),
    }
}
