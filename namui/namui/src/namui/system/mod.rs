pub mod cache;
pub mod file;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod log;
#[cfg(target_os = "windows")]
pub mod media;
pub mod mouse;
pub mod network;
pub mod platform;
pub mod screen;
pub mod skia;
pub mod time;
pub mod typeface;
// #[cfg(target_os = "wasi")]
// pub mod clipboard;
// #[cfg(target_os = "wasi")]
// pub mod deep_link;
// #[cfg(target_os = "wasi")]
// pub mod drag_and_drop;

use crate::*;
use std::sync::atomic::AtomicBool;

type InitResult = Result<()>;

static SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(super) async fn init_system() -> InitResult {
    futures::try_join!(
        cache::init(),
        file::init(),
        font::init(),
        image::init(),
        keyboard::init(),
        log::init(),
        mouse::init(),
        network::init(),
        screen::init(),
        time::init(),
    )?;

    skia::init()?;

    // #[cfg(target_os = "wasi")]
    // futures::try_join!(
    //     deep_link::init(),
    //     drag_and_drop::init(),
    //     web::init(),
    // )?;

    tokio::try_join!(typeface::init())?;
    #[cfg(target_os = "windows")]
    tokio::try_join!(media::init())?; // todo: join this with typeface

    SYSTEM_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn system_initialized() -> bool {
    SYSTEM_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst)
}
