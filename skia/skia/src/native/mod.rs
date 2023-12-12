// mod canvas;
// mod color_filter;
// mod font;
// mod group_glyph;
// mod image;
mod native_skia;
// mod paint;
// mod path;
// TODO
// mod runtime_effect;
// mod shader;
mod surface;
// mod text_blob;
// mod typeface;

// pub(crate) use canvas::*;
// pub(crate) use color_filter::*;
// pub(crate) use font::*;
// pub(crate) use group_glyph::*;
// pub(crate) use image::*;
use namui_type::*;
pub(crate) use native_skia::*;
// pub(crate) use paint::*;
// pub(crate) use path::*;
// // pub(crate) use runtime_effect::*;
// pub(crate) use shader::*;
pub(crate) use surface::*;
// pub(crate) use text_blob::*;
// pub(crate) use typeface::*;
use crate::SkSkia;
use anyhow::Result;
use std::sync::{Arc, Mutex};

#[cfg(feature = "windows")]
pub fn init_skia(
    screen_id: usize,
    window_wh: Wh<IntPx>,
) -> Result<Arc<Mutex<dyn SkSkia + Send + Sync>>> {
    Ok(Arc::new(Mutex::new(NativeSkia::new(screen_id, window_wh)?)))
}
