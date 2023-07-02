pub mod audio;
pub mod cache;
pub mod clipboard;
pub mod deep_link;
pub mod drag_and_drop;
pub mod file;
pub mod font;
pub(crate) mod graphics;
pub mod image;
pub mod keyboard;
pub mod log;
pub mod mouse;
pub mod network;
mod platform_utils;
pub(crate) mod render;
pub mod screen;
pub mod text_input;
pub mod time;
pub(crate) mod typeface;

use futures::try_join;
use platform_utils::*;
pub use render::last_rendering_tree;
use std::error::Error;

type InitResult = Result<(), Box<dyn Error>>;

pub(crate) async fn init() -> InitResult {
    try_join!(
        audio::init(),
        cache::init(),
        file::init(),
        font::init(),
        graphics::init(),
        image::init(),
        keyboard::init(),
        log::init(),
        mouse::init(),
        network::init(),
        screen::init(),
        text_input::init(),
        time::init(),
        typeface::init(),
        deep_link::init(),
        drag_and_drop::init(),
        render::init(),
    )?;

    Ok(())
}
