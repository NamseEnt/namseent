use super::*;
use crate::namui;
pub use base::*;

unsafe impl Sync for CanvasKitColorFilter {}
unsafe impl Send for CanvasKitColorFilter {}
pub(crate) struct ColorFilter(pub(crate) CanvasKitColorFilter);
impl ColorFilter {
    pub fn from(canvas_kit_color_filter: CanvasKitColorFilter) -> Self {
        ColorFilter(canvas_kit_color_filter)
    }
    pub fn blend(color: &Color, blend_mode: &BlendMode) -> ColorFilter {
        let color_array = color.into_float32_array();
        let canvas_kit_color_filter = canvas_kit()
            .ColorFilter()
            .MakeBlend(&color_array, &blend_mode.into_canvas_kit());
        ColorFilter::from(canvas_kit_color_filter)
    }
}

impl Drop for ColorFilter {
    fn drop(&mut self) {
        self.0.delete();
    }
}

impl std::fmt::Debug for ColorFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ColorFilter")
    }
}
