mod calculate;
mod canvas;
mod color_filter;
mod font;
mod group_glyph;
mod paint;
mod path;
mod shader;
mod text_blob;
mod typeface;
#[cfg(target_os = "windows")]
mod windows;
// TODO
// mod runtime_effect;

use self::calculate::NativeCalculate;
use crate::*;
use anyhow::Result;
pub(crate) use color_filter::*;
pub(crate) use font::*;
pub(crate) use group_glyph::*;
use namui_type::*;
pub(crate) use paint::*;
pub(crate) use path::*;
pub(crate) use shader::*;
use std::sync::Arc;
pub(crate) use text_blob::*;
pub(crate) use typeface::*;
#[cfg(target_os = "windows")]
pub use windows::*;
// // pub(crate) use runtime_effect::*;

#[cfg(target_os = "windows")]
pub fn init_skia(screen_id: usize, window_wh: Wh<IntPx>) -> Result<NativeSkia> {
    NativeSkia::new(screen_id, window_wh)
}

pub fn init_calculate() -> Result<Arc<impl SkCalculate + Send + Sync>> {
    Ok(Arc::new(NativeCalculate::new()))
}
