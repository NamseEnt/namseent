use super::InitResult;
use crate::*;
use std::sync::Arc;

// TODO: Remove this system if this system only bnid system::skia's api.
pub(super) async fn init() -> InitResult {
    Ok(())
}

/// None when font is not found.
pub fn font_metrics(font: &Font) -> Option<FontMetrics> {
    crate::system::skia::font_metrics(font)
}

pub fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    crate::system::skia::group_glyph(font, paint)
}
