pub mod file;
pub mod font;
pub(crate) mod graphics;
pub mod image;
pub mod keyboard;
pub mod log;
pub mod mouse;
pub mod network;
mod platform_utils;
pub mod screen;
pub mod text_input;
pub mod time;
pub(crate) mod typeface;
mod wheel;

use futures::try_join;
use platform_utils::*;
use std::error::Error;

type InitResult = Result<(), Box<dyn Error>>;

pub(crate) async fn init() -> InitResult {
    try_join!(
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
        wheel::init(),
    )?;

    Ok(())
}
