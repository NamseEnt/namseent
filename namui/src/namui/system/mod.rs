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
// pub mod wheel;

use futures::try_join;
use lazy_static::lazy_static;
use platform_utils::*;
use std::error::Error;

pub(crate) async fn init() -> Result<(), Box<dyn Error>> {
    try_join!(file::init(), typeface::init())?;

    Ok(())
}
