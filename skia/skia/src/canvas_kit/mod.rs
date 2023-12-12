mod canvas;
mod ck_skia;
mod color_filter;
mod font;
mod group_glyph;
mod image;
mod paint;
mod path;
// TODO
// mod runtime_effect;
mod shader;
mod surface;
mod text_blob;
mod typeface;

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
use anyhow::Result;
pub(crate) use shader::*;
use std::sync::{Arc, Mutex};
pub(crate) use surface::*;
pub(crate) use text_blob::*;
pub(crate) use typeface::*;
use web_sys::HtmlCanvasElement;

#[cfg(feature = "wasm")]
pub fn init_skia(
    canvas_element: Option<&HtmlCanvasElement>,
) -> Result<Arc<Mutex<dyn SkSkia + Send + Sync>>> {
    Ok(Arc::new(Mutex::new(CkSkia::new(canvas_element))))
}
