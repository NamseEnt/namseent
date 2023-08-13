use super::InitResult;
use crate::*;
use namui_skia::*;
use std::sync::{Arc, OnceLock};

static SKIA: OnceLock<Arc<dyn SkSkia + Send + Sync>> = OnceLock::new();

pub(super) async fn init() -> InitResult {
    let skia = namui_skia::init_skia(None);
    SKIA.set(skia).map_err(|_| unreachable!()).unwrap();

    Ok(())
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    SKIA.get().unwrap().load_typeface(typeface_name, bytes);
}

pub(crate) fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    SKIA.get().unwrap().group_glyph(font, paint)
}
