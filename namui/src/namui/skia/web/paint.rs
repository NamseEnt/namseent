use super::*;
use crate::*;
pub use base::*;

unsafe impl Sync for CanvasKitPaint {}
unsafe impl Send for CanvasKitPaint {}
pub(crate) struct Paint {
    id: Box<[u8]>,
    pub(crate) canvas_kit_paint: CanvasKitPaint,
}
impl Paint {
    pub fn new(
        id: Box<[u8]>,
        color: Option<Color>,
        style: Option<&PaintStyle>,
        anti_alias: Option<bool>,
        stroke_width: Option<Px>,
        stroke_cap: Option<&StrokeCap>,
        stroke_join: Option<&StrokeJoin>,
        color_filter: Option<impl AsRef<ColorFilter>>,
        shader: Option<&IntermediateShader>,
    ) -> Self {
        let canvas_kit_paint = CanvasKitPaint::new();
        if let Some(color) = color {
            canvas_kit_paint.setColor(&color.into_float32_array());
        }
        if let Some(style) = style {
            canvas_kit_paint.setStyle(style.into_canvas_kit());
        }
        if let Some(anti_alias) = anti_alias {
            canvas_kit_paint.setAntiAlias(anti_alias);
        }
        if let Some(stroke_width) = stroke_width {
            canvas_kit_paint.setStrokeWidth(stroke_width.as_f32());
        }
        if let Some(stroke_cap) = stroke_cap {
            canvas_kit_paint.setStrokeCap(stroke_cap.into_canvas_kit());
        }
        if let Some(stroke_join) = stroke_join {
            canvas_kit_paint.setStrokeJoin(stroke_join.into_canvas_kit());
        }
        if let Some(color_filter) = color_filter {
            canvas_kit_paint.setColorFilter(&color_filter.as_ref().0);
        }
        if let Some(shader) = shader {
            canvas_kit_paint.setShader(&shader.into_shader().canvas_kit_shader);
        }

        Paint {
            id,
            canvas_kit_paint,
        }
    }
    pub fn get_stroke_cap(&self) -> StrokeCap {
        let stroke_cap = self.canvas_kit_paint.getStrokeCap();

        match stroke_cap.value() {
            value if STROKE_CAP_BUTT_VALUE.eq(&value) => StrokeCap::Butt,
            value if STROKE_CAP_ROUND_VALUE.eq(&value) => StrokeCap::Round,
            value if STROKE_CAP_SQUARE_VALUE.eq(&value) => StrokeCap::Square,
            value => panic!("Unknown stroke_cap value: {}", value),
        }
    }
    pub fn get_stroke_join(&self) -> StrokeJoin {
        let stroke_join = self.canvas_kit_paint.getStrokeJoin();

        match stroke_join.value() {
            value if STROKE_JOIN_BEVEL_VALUE.eq(&value) => StrokeJoin::Bevel,
            value if STROKE_JOIN_MITER_VALUE.eq(&value) => StrokeJoin::Miter,
            value if STROKE_JOIN_ROUND_VALUE.eq(&value) => StrokeJoin::Round,
            value => panic!("Unknown stroke_join value: {}", value),
        }
    }
    pub fn get_stroke_width(&self) -> Px {
        px(self.canvas_kit_paint.getStrokeWidth())
    }
    pub fn get_stroke_miter(&self) -> Px {
        px(self.canvas_kit_paint.getStrokeMiter())
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        self.canvas_kit_paint.delete();
    }
}

impl std::fmt::Debug for Paint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paint")
    }
}

impl Clone for Paint {
    fn clone(&self) -> Self {
        Paint {
            id: self.id.clone(),
            canvas_kit_paint: self.canvas_kit_paint.copy(),
        }
    }
}

impl std::hash::Hash for Paint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Paint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Paint {}
