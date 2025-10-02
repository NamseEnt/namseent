use crate::*;
use std::sync::Arc;

// TODO: Remove this system if this system only bnid system::skia's api.
pub(super) async fn init() -> Result<()> {
    Ok(())
}

/// None when font is not found.
pub fn font_metrics(font: &Font) -> Option<FontMetrics> {
    NativeCalculate::font_metrics(font)
}

pub fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    NativeCalculate::group_glyph(font, paint)
}
