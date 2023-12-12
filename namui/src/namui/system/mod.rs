pub mod audio;
pub mod cache;
#[cfg(target_family = "wasm")]
pub mod clipboard;
pub mod deep_link;
#[cfg(target_family = "wasm")]
pub mod drag_and_drop;
pub(crate) mod drawer;
pub mod file;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod log;
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
pub mod web;

use crate::*;
use platform_utils::*;

type InitResult = Result<()>;

pub(crate) async fn init() -> InitResult {
    tokio::try_join!(
        skia::init(),
        audio::init(),
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
        deep_link::init(),
        #[cfg(target_family = "wasm")]
        drag_and_drop::init(),
        drawer::init(),
        #[cfg(target_family = "wasm")]
        text_input::init(),
        web::init(),
    )?;

    tokio::try_join!(typeface::init(),)?;

    Ok(())
}
