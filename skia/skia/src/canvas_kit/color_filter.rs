use super::*;

use namui_type::{BlendMode, Color};

pub(crate) struct CkColorFilter(pub(crate) CanvasKitColorFilter);
impl CkColorFilter {
    pub fn from(canvas_kit_color_filter: CanvasKitColorFilter) -> Self {
        CkColorFilter(canvas_kit_color_filter)
    }
    pub fn blend(color: Color, blend_mode: BlendMode) -> CkColorFilter {
        let color_array = color.into_float32_array();
        let canvas_kit_color_filter = canvas_kit()
            .ColorFilter()
            .MakeBlend(&color_array, blend_mode.into());
        CkColorFilter::from(canvas_kit_color_filter)
    }
}

impl Drop for CkColorFilter {
    fn drop(&mut self) {
        self.0.delete();
    }
}

impl std::fmt::Debug for CkColorFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColorFilter")
    }
}
