pub mod audio;
pub mod keyboard;
pub mod kv_store;
pub mod mouse;
pub mod screen;
pub mod time;

use crate::*;
pub use audio::AudioAsset;
pub use audio::AudioGroup;
pub use audio::{Audio, AudioListener};

type InitResult = Result<()>;

static SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(super) fn init_system() -> InitResult {
    keyboard::init()?;
    screen::init()?;
    time::init()?;
    mouse::init()?;

    SYSTEM_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn system_initialized() -> bool {
    SYSTEM_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst)
}
