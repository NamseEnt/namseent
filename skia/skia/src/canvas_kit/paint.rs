use super::*;
use std::sync::Arc;

pub(crate) struct CkPaint {
    // last_set_shader: Arc<Mutex<Option<Arc<CkShader>>>>,
    canvas_kit_paint: CanvasKitPaint,
}
impl CkPaint {
    pub(crate) fn get(paint: &Paint) -> Arc<Self> {
        static CACHE: SerdeLruCache<Paint, CkPaint> = SerdeLruCache::new();
        CACHE.get_or_create(paint, |paint| CkPaint::new(paint))
    }
    pub fn new(paint: &Paint) -> Self {
        let canvas_kit_paint = CanvasKitPaint::new();
        apply_paint_to_canvas_kit(&canvas_kit_paint, paint);

        CkPaint {
            // last_set_shader: Arc::new(Mutex::new(None)),
            canvas_kit_paint,
        }
    }
    // pub(crate) fn set_shader(&self, shader: Option<&Arc<CkShader>>) {
    //     if self.last_set_shader.lock().unwrap().as_ref() == shader {
    //         return;
    //     }
    //     self.canvas_kit_paint
    //         .setShader(shader.map(|shader| &shader.canvas_kit_shader));
    // }
    // pub(crate) fn get_shader(&self) -> Option<Arc<CkShader>> {
    //     self.last_set_shader.lock().unwrap().clone()
    // }
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

    pub(crate) fn canvas_kit(&self) -> &CanvasKitPaint {
        &self.canvas_kit_paint
    }
}

fn apply_paint_to_canvas_kit(canvas_kit_paint: &CanvasKitPaint, paint: &Paint) {
    let &Paint {
        color,
        paint_style,
        anti_alias,
        stroke_width,
        stroke_cap,
        stroke_join,
        // color_filter,
        blend_mode,
    } = paint;
    if let Some(color) = color {
        canvas_kit_paint.setColor(&color.into_float32_array());
    }
    if let Some(style) = paint_style {
        canvas_kit_paint.setStyle(style.into());
    }
    if let Some(anti_alias) = anti_alias {
        canvas_kit_paint.setAntiAlias(anti_alias);
    }
    if let Some(stroke_width) = stroke_width {
        canvas_kit_paint.setStrokeWidth(stroke_width.as_f32());
    }
    if let Some(stroke_cap) = stroke_cap {
        canvas_kit_paint.setStrokeCap(stroke_cap.into());
    }
    if let Some(stroke_join) = stroke_join {
        canvas_kit_paint.setStrokeJoin(stroke_join.into());
    }
    // if let Some(color_filter) = color_filter {
    //     canvas_kit_paint.setColorFilter(&color_filter.0);
    // }
    if let Some(blend_mode) = blend_mode {
        canvas_kit_paint.setBlendMode(blend_mode.into());
    }
}

impl Drop for CkPaint {
    fn drop(&mut self) {
        self.canvas_kit_paint.delete();
    }
}
