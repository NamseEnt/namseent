use crate::engine;

use super::*;
pub use base::*;

pub struct Paint(pub(crate) CanvasKitPaint);
impl Paint {
    pub fn new() -> Self {
        Paint(CanvasKitPaint::new())
    }
    pub fn set_color(&self, color: &Color) {
        self.0.setColor(&color.into_float32_array());
    }

    pub fn set_style(&self, style: &PaintStyle) {
        let canvas_kit_paint_style = match style {
            PaintStyle::Fill => canvas_kit().PaintStyle().Fill(),
            PaintStyle::Stroke => canvas_kit().PaintStyle().Stroke(),
        };
        self.0.setStyle(canvas_kit_paint_style);
    }
    pub fn set_anti_alias(&self, value: bool) {
        self.0.setAntiAlias(value);
    }
    pub fn set_stroke_width(&self, width: f32) {
        self.0.setStrokeWidth(width);
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        engine::log("Dropping paint".to_string());
        self.0.delete();
    }
}

impl std::fmt::Debug for Paint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paint")
    }
}
