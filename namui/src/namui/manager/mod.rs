pub mod common;
pub use common::*;
use once_cell::sync::OnceCell;
use parking_lot::{ReentrantMutex, ReentrantMutexGuard};
use std::sync::Arc;

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
    pub mouse_manager: MouseManager,
    pub font_manager: FontManager,
    pub keyboard_manager: KeyboardManager,
    pub screen_manager: ScreenManager,
    pub image_manager: ImageManager,
    pub wheel_manager: WheelManager,
    pub(crate) text_input_manager: TextInputManager,
}

pub(crate) static MANAGERS: OnceCell<ReentrantMutex<Arc<Managers>>> = OnceCell::new();
pub fn managers() -> ReentrantMutexGuard<'static, Arc<Managers>> {
    MANAGERS.get().expect("managers not initialized").lock()
}

pub(crate) fn set_managers(managers: Managers) {
    if MANAGERS
        .set(ReentrantMutex::new(Arc::new(managers)))
        .is_err()
    {
        panic!("managers already initialized");
    }
}
