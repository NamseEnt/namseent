pub mod cache;
#[cfg(target_family = "wasm")]
pub mod clipboard;
#[cfg(target_family = "wasm")]
pub mod deep_link;
#[cfg(target_family = "wasm")]
pub mod drag_and_drop;
pub(crate) mod drawer;
pub mod file;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod log;
pub mod media;
pub mod mouse;
pub mod network;
pub(crate) mod panick;
mod platform_utils;
pub mod screen;
pub(crate) mod skia;
#[cfg(target_family = "wasm")]
pub(crate) mod text_input;
pub mod time;
pub(crate) mod typeface;
#[cfg(target_family = "wasm")]
pub mod web;

use crate::*;
#[cfg(target_family = "wasm")]
use platform_utils::*;
use std::sync::atomic::AtomicBool;

type InitResult = Result<()>;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(crate) async fn init() -> InitResult {
    futures::try_join!(
        media::init(),
        cache::init(),
        file::init(),
        font::init(),
        image::init(),
        keyboard::init(),
        log::init(),
        mouse::init(),
        network::init(),
        panick::init(),
        screen::init(),
        time::init(),
        drawer::init(),
    )?;

    futures::try_join!(skia::init())?;

    #[cfg(target_family = "wasm")]
    futures::try_join!(
        deep_link::init(),
        drag_and_drop::init(),
        text_input::init(),
        web::init(),
    )?;

    tokio::try_join!(typeface::init(),)?;

    INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

async fn wait_for_system_init() {
    while !INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
}
