pub mod cache;
pub mod file;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod mouse;
pub mod network;
pub mod platform;
pub mod screen;
pub mod skia;
pub mod time;
pub mod typeface;

use crate::*;

pub(super) fn init_system() -> Result<()> {
    time::init();

    Ok(())
}
