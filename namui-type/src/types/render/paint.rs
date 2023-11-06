use crate::*;

#[type_derives(Default)]
pub struct Paint {
    pub color: Option<Color>,
    pub paint_style: Option<PaintStyle>,
    pub anti_alias: Option<bool>,
    pub stroke_width: Option<Px>,
    pub stroke_cap: Option<StrokeCap>,
    pub stroke_join: Option<StrokeJoin>,
    pub stroke_miter: Option<Px>,
    pub color_filter: Option<ColorFilter>,
    pub blend_mode: Option<BlendMode>,
    pub shader: Option<Shader>,
}

impl Paint {
    pub fn new() -> Self {
        Self {
            color: None,
            paint_style: None,
            anti_alias: None,
            stroke_width: None,
            stroke_cap: None,
            stroke_join: None,
            stroke_miter: None,
            color_filter: None,
            blend_mode: None,
            shader: None,
        }
    }
    pub fn set_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn set_style(mut self, style: PaintStyle) -> Self {
        self.paint_style = Some(style);
        self
    }
    pub fn set_anti_alias(mut self, value: bool) -> Self {
        self.anti_alias = Some(value);
        self
    }
    pub fn set_stroke_width(mut self, width: Px) -> Self {
        self.stroke_width = Some(width);
        self
    }
    pub fn set_stroke_cap(mut self, cap: StrokeCap) -> Self {
        self.stroke_cap = Some(cap);
        self
    }
    pub fn set_stroke_join(mut self, join: StrokeJoin) -> Self {
        self.stroke_join = Some(join);
        self
    }
    pub fn set_color_filter(mut self, color_filter: ColorFilter) -> Self {
        self.color_filter = Some(color_filter);
        self
    }
    pub fn set_blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.blend_mode = Some(blend_mode);
        self
    }
    pub fn set_shader(mut self, shader: Shader) -> Self {
        self.shader = Some(shader);
        self
    }
}
