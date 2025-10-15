pub mod audio;
pub mod keyboard;
pub mod mouse;
pub mod network;
pub mod screen;
pub mod time;

use crate::*;
use std::sync::atomic::AtomicBool;

type InitResult = Result<()>;

static SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(super) fn init_system() -> InitResult {
    keyboard::init()?;
    network::init()?;
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
