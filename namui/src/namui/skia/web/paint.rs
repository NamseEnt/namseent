use super::*;
use crate::namui;
pub use base::*;

pub struct Paint(pub(crate) CanvasKitPaint);
impl Paint {
    pub fn new() -> Self {
        Paint(CanvasKitPaint::new())
    }
    pub fn set_color(self, color: Color) -> Self {
        self.0.setColor(&color.into_float32_array());
        self
    }

    pub fn set_style(self, style: PaintStyle) -> Self {
        let canvas_kit_paint_style = match style {
            PaintStyle::Fill => canvas_kit().PaintStyle().Fill(),
            PaintStyle::Stroke => canvas_kit().PaintStyle().Stroke(),
        };
        self.0.setStyle(canvas_kit_paint_style);
        self
    }
    pub fn set_anti_alias(self, value: bool) -> Self {
        self.0.setAntiAlias(value);
        self
    }
    pub fn set_stroke_width(self, width: f32) -> Self {
        self.0.setStrokeWidth(width);
        self
    }
    pub fn get_stroke_cap(&self) -> StrokeCap {
        let stroke_cap = self.0.getStrokeCap();

        let butt_value = &STROKE_CAP_BUTT_VALUE;
        let round_value = &STROKE_CAP_ROUND_VALUE;
        let square_value = &STROKE_CAP_SQUARE_VALUE;

        match stroke_cap.value() {
            butt_value => StrokeCap::Butt,
            round_value => StrokeCap::Round,
            square_value => StrokeCap::Square,
        }
    }
    pub fn get_stroke_join(&self) -> StrokeJoin {
        let stroke_join = self.0.getStrokeJoin();

        let bevel_value = &STROKE_JOIN_BEVEL_VALUE;
        let miter_value = &STROKE_JOIN_MITER_VALUE;
        let round_value = &STROKE_JOIN_ROUND_VALUE;

        match stroke_join.value() {
            bevel_value => StrokeJoin::Bevel,
            miter_value => StrokeJoin::Miter,
            round_value => StrokeJoin::Round,
        }
    }
    pub fn get_stroke_width(&self) -> f32 {
        self.0.getStrokeWidth()
    }
    pub fn get_stroke_miter(&self) -> f32 {
        self.0.getStrokeMiter()
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        self.0.delete();
    }
}

impl std::fmt::Debug for Paint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paint")
    }
}
