#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

use super::InitResult;
use namui_skia::SkSkia;
use namui_type::*;
#[cfg(not(target_family = "wasm"))]
pub(crate) use non_wasm::*;
use std::sync::{Arc, OnceLock, RwLock};
#[cfg(target_family = "wasm")]
use wasm::*;

static SKIA: OnceLock<Arc<RwLock<dyn SkSkia + Send + Sync>>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    let skia = init_skia().await?;
    SKIA.set(skia).map_err(|_| unreachable!()).unwrap();

    Ok(())
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    SKIA.get()
        .unwrap()
        .read()
        .unwrap()
        .load_typeface(typeface_name, bytes);
}

pub(crate) fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    SKIA.get().unwrap().read().unwrap().group_glyph(font, paint)
}

pub(crate) fn path_contains_xy(path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
    SKIA.get()
        .unwrap()
        .read()
        .unwrap()
        .path_contains_xy(path, paint, xy)
}

pub(crate) fn path_bounding_box(path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
    SKIA.get()
        .unwrap()
        .read()
        .unwrap()
        .path_bounding_box(path, paint)
}

pub(crate) fn font_metrics(font: &Font) -> Option<FontMetrics> {
    SKIA.get().unwrap().read().unwrap().font_metrics(font)
}
