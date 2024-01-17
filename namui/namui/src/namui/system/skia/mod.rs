#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

use super::InitResult;
use namui_skia::{SkCalculate, SkSkia};
use namui_type::*;
#[cfg(not(target_family = "wasm"))]
pub(crate) use non_wasm::*;
use std::sync::{Arc, OnceLock, RwLock};
#[cfg(target_family = "wasm")]
use wasm::*;

static SKIA: OnceLock<Arc<RwLock<dyn SkSkia + Send + Sync>>> = OnceLock::new();
static SK_CALCULATE: OnceLock<Arc<dyn SkCalculate + Send + Sync>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    let skia = init_skia().await?;
    SKIA.set(skia).map_err(|_| unreachable!()).unwrap();

    let calculate = init_calculate().await?;
    SK_CALCULATE
        .set(calculate)
        .map_err(|_| unreachable!())
        .unwrap();

    Ok(())
}

fn sk_calculate() -> &'static dyn SkCalculate {
    SK_CALCULATE.get().unwrap().as_ref()
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    sk_calculate().load_typeface(typeface_name, bytes);
}

pub(crate) fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    sk_calculate().group_glyph(font, paint)
}

pub(crate) fn path_contains_xy(path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
    sk_calculate().path_contains_xy(path, paint, xy)
}

pub(crate) fn path_bounding_box(path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
    sk_calculate().path_bounding_box(path, paint)
}

pub(crate) fn font_metrics(font: &Font) -> Option<FontMetrics> {
    sk_calculate().font_metrics(font)
}

/// Encoded image
pub(crate) fn load_image(image_source: &ImageSource, bytes: &[u8]) -> ImageInfo {
    sk_calculate().load_image(image_source, bytes)
}

/// Raw image
pub(crate) fn load_image2(image_info: ImageInfo, bytes: &mut [u8]) -> ImageHandle {
    sk_calculate().load_image_from_raw(image_info, bytes)
}
