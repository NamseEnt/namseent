use super::*;
use std::sync::Arc;

pub(crate) struct NativePaint {
    skia_paint: skia_safe::Paint,
}
impl NativePaint {
    pub(crate) fn get(paint: &Paint) -> Arc<Self> {
        static NATIVE_PAINT_CACHE: SerdeLruCache<Paint, NativePaint> = SerdeLruCache::new();
        NATIVE_PAINT_CACHE.get_or_create(paint, Self::new)
    }
    fn new(paint: &Paint) -> Self {
        NativePaint {
            skia_paint: new_skia_paint(paint),
        }
    }

    pub(crate) fn skia(&self) -> &skia_safe::Paint {
        &self.skia_paint
    }
}

fn new_skia_paint(paint: &Paint) -> skia_safe::Paint {
    let mut skia_paint = skia_safe::Paint::new(skia_safe::Color4f::from(paint.color), None);
    let &Paint {
        color: _,
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
    if let Some(style) = paint_style {
        skia_paint.set_style(style.into());
    }
    if let Some(anti_alias) = anti_alias {
        skia_paint.set_anti_alias(anti_alias);
    }
    if let Some(stroke_width) = stroke_width {
        skia_paint.set_stroke_width(stroke_width.as_f32());
    }
    if let Some(stroke_cap) = stroke_cap {
        skia_paint.set_stroke_cap(stroke_cap.into());
    }
    if let Some(stroke_join) = stroke_join {
        skia_paint.set_stroke_join(stroke_join.into());
    }
    if let Some(stroke_miter) = stroke_miter {
        skia_paint.set_stroke_miter(stroke_miter.as_f32());
    }
    if let Some(_color_filter) = color_filter {
        todo!()
        // let ck_color_filter = CkColorFilter::get(color_filter);
        // skia_paint.set_color_filter(ck_color_filter.skia());
    }
    if let Some(blend_mode) = blend_mode {
        skia_paint.set_blend_mode(blend_mode.into());
    }
    if let Some(_shader) = shader {
        todo!()
        // let ck_shader = CkShader::get(shader);
        // skia_paint.set_shader(Some(ck_shader.skia()));
    }

    skia_paint
}
