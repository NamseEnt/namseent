pub mod cache;
// #[cfg(target_family = "wasm")]
// pub mod clipboard;
// #[cfg(target_family = "wasm")]
// pub mod deep_link;
// #[cfg(target_family = "wasm")]
// pub mod drag_and_drop;
pub mod file;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod log;
#[cfg(not(target_family = "wasm"))]
pub mod media;
pub mod mouse;
pub mod network;
mod platform_utils;
pub mod screen;
pub(crate) mod skia;
// #[cfg(target_family = "wasm")]
// pub(crate) mod text_input;
pub mod time;
pub mod typeface;
#[cfg(target_family = "wasm")]
pub mod web;

use crate::*;
#[cfg(target_family = "wasm")]
use platform_utils::*;
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

    futures::try_join!(skia::init())?;

    // #[cfg(target_family = "wasm")]
    // futures::try_join!(
    //     deep_link::init(),
    //     drag_and_drop::init(),
    //     text_input::init(),
    //     web::init(),
    // )?;

    tokio::try_join!(typeface::init())?;
    #[cfg(not(target_family = "wasm"))]
    tokio::try_join!(media::init())?; // todo: join this with typeface

    SYSTEM_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

pub(crate) fn take_main_thread() {
    screen::take_main_thread();
}

#[allow(dead_code)]
pub(crate) fn system_initialized() -> bool {
    SYSTEM_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst)
}
