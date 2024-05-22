#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(not(target_family = "wasm"))]
use non_wasm as inner;
#[cfg(target_family = "wasm")]
use wasm as inner;

#[cfg(not(target_family = "wasm"))]
pub(crate) use inner::*;

use super::InitResult;
use anyhow::Result;
use namui_skia::{Font, FontMetrics, GroupGlyph, Image, Paint, RenderingTree, SkCalculate};
use std::sync::{Arc, OnceLock};

static SK_CALCULATE: OnceLock<Arc<dyn SkCalculate + Send + Sync>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    inner::init().await?;

    let calculate = namui_skia::init_calculate()?;
    SK_CALCULATE
        .set(calculate)
        .map_err(|_| unreachable!())
        .unwrap();

    Ok(())
}

pub(crate) fn sk_calculate() -> &'static dyn SkCalculate {
    SK_CALCULATE.get().unwrap().as_ref()
}

pub(crate) async fn load_typeface(typeface_name: String, bytes: Vec<u8>) -> Result<()> {
    inner::load_typeface(typeface_name, bytes).await
}

pub(crate) fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    sk_calculate().group_glyph(font, paint)
}

pub(crate) fn font_metrics(font: &Font) -> Option<FontMetrics> {
    sk_calculate().font_metrics(font)
}

pub(crate) async fn load_image_from_url(url: impl AsRef<str>) -> Result<Image> {
    inner::load_image_from_url(url).await
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    inner::request_draw_rendering_tree(rendering_tree)
}
