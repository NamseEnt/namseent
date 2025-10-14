pub mod audio;
pub mod cache;
pub mod file;
pub mod image;
pub mod keyboard;
pub mod mouse;
pub mod network;
pub mod platform;
pub mod screen;
pub mod time;
pub mod typeface;
#[cfg(target_os = "wasi")]
pub mod wasi;
// pub mod clipboard;
// #[cfg(target_os = "wasi")]
// pub mod deep_link;
// #[cfg(target_os = "wasi")]
// pub mod drag_and_drop;

use crate::*;
use std::sync::atomic::AtomicBool;

type InitResult = Result<()>;

static SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(super) fn init_system() -> InitResult {
    #[cfg(target_os = "wasi")]
    wasi::init()?;

    // audio::init()?;
    // cache::init()?;
    // file::init()?;
    // image::init()?;
    keyboard::init()?;
    network::init()?;
    screen::init()?;
    time::init()?;

    // #[cfg(target_os = "wasi")]
    // futures::try_join!(
    //     deep_link::init(),
    //     drag_and_drop::init(),
    //     web::init(),
    // )?;

    mouse::init()?;
    typeface::init()?;

    SYSTEM_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn system_initialized() -> bool {
    SYSTEM_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst)
}
