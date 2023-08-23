use super::*;
use std::sync::Arc;

pub(crate) struct CkPaint {
    canvas_kit_paint: CanvasKitPaint,
}
impl CkPaint {
    pub(crate) fn get(paint: &Paint) -> Arc<Self> {
        static CK_PAINT_CACHE: SerdeLruCache<Paint, CkPaint> = SerdeLruCache::new();
        CK_PAINT_CACHE.get_or_create(paint, |paint| Self::new(paint))
    }
    fn new(paint: &Paint) -> Self {
        let canvas_kit_paint = CanvasKitPaint::new();
        apply_paint_to_canvas_kit(&canvas_kit_paint, paint);

        CkPaint { canvas_kit_paint }
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
        stroke_miter,
        color_filter,
        blend_mode,
        ref shader,
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
    if let Some(stroke_miter) = stroke_miter {
        canvas_kit_paint.setStrokeMiter(stroke_miter.as_f32());
    }
    if let Some(color_filter) = color_filter {
        let ck_color_filter = CkColorFilter::get(color_filter);
        canvas_kit_paint.setColorFilter(&ck_color_filter.canvas_kit());
    }
    if let Some(blend_mode) = blend_mode {
        canvas_kit_paint.setBlendMode(blend_mode.into());
    }
    if let Some(shader) = shader {
        let ck_shader = CkShader::get(shader);
        canvas_kit_paint.setShader(Some(&ck_shader.canvas_kit()));
    }
}

impl Drop for CkPaint {
    fn drop(&mut self) {
        self.canvas_kit_paint.delete();
    }
}
