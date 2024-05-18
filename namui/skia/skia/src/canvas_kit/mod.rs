mod calculate;
mod canvas;
mod ck_skia;
mod color_filter;
mod font;
mod group_glyph;
mod image;
mod paint;
mod path;
mod shader;
mod surface;
mod text_blob;
mod typeface;
mod utils;
// TODO
// mod runtime_effect;

pub(crate) use calculate::*;
pub(crate) use canvas::*;
use canvas_kit_wasm_bindgen::*;
pub(crate) use ck_skia::*;
pub(crate) use color_filter::*;
pub(crate) use font::*;
pub(crate) use group_glyph::*;
pub(crate) use image::*;
use namui_type::*;
pub(crate) use paint::*;
pub(crate) use path::*;
// pub(crate) use runtime_effect::*;
use crate::SkSkia;
use crate::*;
use anyhow::Result;
pub(crate) use shader::*;
use std::sync::{Arc, Mutex};
pub(crate) use surface::*;
pub(crate) use text_blob::*;
pub(crate) use typeface::*;
use utils::*;
use web_sys::HtmlCanvasElement;

#[cfg(feature = "wasm")]
pub async fn init_skia(
    canvas_element: &HtmlCanvasElement,
) -> Result<Arc<Mutex<dyn SkSkia + Send + Sync>>> {
    Ok(Arc::new(Mutex::new(CkSkia::new(canvas_element).await?)))
}

// pub fn init_skia(
//     screen_id: usize,
//     window_wh: Wh<IntPx>,
// ) -> Result<Arc<RwLock<impl SkSkia + Send + Sync>>> {
//     Ok(Arc::new(RwLock::new(NativeSkia::new(
//         screen_id, window_wh,
//     )?)))
// }

pub fn init_calculate() -> Result<Arc<impl SkCalculate + Send + Sync>> {
    Ok(Arc::new(CkCalculate::new()))
}
