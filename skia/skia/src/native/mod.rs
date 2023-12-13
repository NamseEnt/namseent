mod canvas;
mod color_filter;
mod font;
mod group_glyph;
mod image;
mod native_skia;
mod paint;
mod path;
mod shader;
mod surface;
mod text_blob;
mod typeface;
// TODO
// mod runtime_effect;

use crate::SkSkia;
use anyhow::Result;
pub(crate) use canvas::*;
pub(crate) use color_filter::*;
pub(crate) use font::*;
pub(crate) use group_glyph::*;
pub(crate) use image::*;
use namui_type::*;
pub(crate) use native_skia::*;
pub(crate) use paint::*;
pub(crate) use path::*;
pub(crate) use shader::*;
use std::sync::{Arc, Mutex};
pub(crate) use surface::*;
pub(crate) use text_blob::*;
pub(crate) use typeface::*;
// // pub(crate) use runtime_effect::*;

#[cfg(feature = "windows")]
pub fn init_skia(
    screen_id: usize,
    window_wh: Wh<IntPx>,
) -> Result<Arc<Mutex<impl SkSkia + Send + Sync>>> {
    Ok(Arc::new(Mutex::new(NativeSkia::new(screen_id, window_wh)?)))
}
