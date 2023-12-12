#[cfg(not(target_family = "wasm"))]
mod non_wasm;
#[cfg(target_family = "wasm")]
mod wasm;

use super::InitResult;
use crate::*;
use namui_skia::SkSkia;
use std::sync::{Arc, Mutex, OnceLock};

#[cfg(not(target_family = "wasm"))]
pub(crate) use non_wasm::*;
#[cfg(target_family = "wasm")]
use wasm::*;

static SKIA: OnceLock<Arc<Mutex<dyn SkSkia + Send + Sync>>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    let skia = init_skia().await?;
    SKIA.set(skia).map_err(|_| unreachable!()).unwrap();

    Ok(())
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    SKIA.get()
        .unwrap()
        .lock()
        .unwrap()
        .load_typeface(typeface_name, bytes);
}

pub(crate) fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    SKIA.get().unwrap().lock().unwrap().group_glyph(font, paint)
}

pub(crate) fn path_contains_xy(path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
    SKIA.get()
        .unwrap()
        .lock()
        .unwrap()
        .path_contains_xy(path, paint, xy)
}

pub(crate) fn path_bounding_box(path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
    SKIA.get()
        .unwrap()
        .lock()
        .unwrap()
        .path_bounding_box(path, paint)
}

pub(crate) fn font_metrics(font: &Font) -> Option<FontMetrics> {
    SKIA.get().unwrap().lock().unwrap().font_metrics(font)
}
