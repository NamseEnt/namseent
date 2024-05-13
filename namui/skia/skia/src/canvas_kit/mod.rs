mod calculate;
#[cfg(feature = "wasm-drawer")]
mod canvas;
#[cfg(feature = "wasm-drawer")]
mod ck_skia;
#[cfg(feature = "wasm-drawer")]
mod color_filter;
mod font;
mod group_glyph;
#[cfg(feature = "wasm-drawer")]
mod image;
mod paint;
mod path;
#[cfg(feature = "wasm-drawer")]
mod shader;
#[cfg(feature = "wasm-drawer")]
mod surface;
#[cfg(feature = "wasm-drawer")]
#[cfg(feature = "wasm-drawer")]
mod text_blob;
mod typeface;
mod utils;
// TODO
// mod runtime_effect;

#[cfg(feature = "wasm-drawer")]
pub use crate::SkSkia;
use crate::*;
use anyhow::Result;
pub(crate) use calculate::*;
#[cfg(feature = "wasm-drawer")]
pub(crate) use canvas::*;
use canvas_kit_wasm_bindgen::*;
#[cfg(feature = "wasm-drawer")]
pub use ck_skia::*;
#[cfg(feature = "wasm-drawer")]
pub(crate) use color_filter::*;
pub(crate) use font::*;
pub(crate) use group_glyph::*;
#[cfg(feature = "wasm-drawer")]
pub(crate) use image::*;
#[cfg(feature = "wasm-drawer")]
use namui_type::*;
pub(crate) use paint::*;
pub(crate) use path::*;
#[cfg(feature = "wasm-drawer")]
pub(crate) use shader::*;
use std::sync::Arc;
#[cfg(feature = "wasm-drawer")]
pub(crate) use surface::*;
#[cfg(feature = "wasm-drawer")]
pub(crate) use text_blob::*;
pub(crate) use typeface::*;
use utils::*;

#[cfg(feature = "wasm-drawer")]
pub async fn init_skia(canvas_element: &web_sys::HtmlCanvasElement) -> Result<CkSkia> {
    CkSkia::new(canvas_element).await
}

pub fn init_calculate() -> Result<Arc<impl SkCalculate + Send + Sync>> {
    Ok(Arc::new(CkCalculate::new()))
}
