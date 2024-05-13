#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

use super::InitResult;
use namui_skia::{
    Font, FontMetrics, GroupGlyph, Image, ImageInfo, Paint, RenderingTree, SkCalculate,
};
use std::sync::{Arc, OnceLock};

use crate::spawn_blocking;
use anyhow::Result;
#[cfg(not(target_family = "wasm"))]
use non_wasm as inner;
#[cfg(target_family = "wasm")]
use wasm as inner;

static SK_CALCULATE: OnceLock<Arc<dyn SkCalculate + Send + Sync>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    #[cfg(not(target_family = "wasm"))]
    inner::init_skia().await?;

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

pub(crate) async fn load_typeface(typeface_name: &str, bytes: &[u8]) -> Result<()> {
    tokio::try_join!(
        async move { spawn_blocking(|| sk_calculate().load_typeface(typeface_name, bytes)).await? },
        inner::load_typeface(typeface_name, bytes)
    )?;

    Ok(())
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

pub(crate) async fn load_image_from_raw(
    bytes: &[u8],
    image_info: Option<ImageInfo>,
    encoded: bool,
) -> Result<Image> {
    inner::load_image_from_raw(bytes, image_info, encoded).await
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    inner::request_draw_rendering_tree(rendering_tree)
}
