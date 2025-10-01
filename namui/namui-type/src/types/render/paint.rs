use crate::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Default, Hash, Eq)]
pub struct Paint {
    pub color: Color,
    pub paint_style: Option<PaintStyle>,
    pub anti_alias: Option<bool>,
    pub stroke_width: Px,
    pub stroke_cap: Option<StrokeCap>,
    pub stroke_join: Option<StrokeJoin>,
    pub stroke_miter: Px,
    pub color_filter: Option<ColorFilter>,
    pub blend_mode: Option<BlendMode>,
    pub shader: Option<Box<Shader>>,
    pub mask_filter: Option<MaskFilter>,
    pub image_filter: Option<Box<ImageFilter>>,
}

impl Paint {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
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
        self.stroke_width = width;
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
        self.shader = Some(Box::new(shader));
        self
    }
    pub fn set_mask_filter(mut self, mask_filter: MaskFilter) -> Self {
        self.mask_filter = Some(mask_filter);
        self
    }
    pub fn set_image_filter(mut self, image_filter: ImageFilter) -> Self {
        self.image_filter = Some(Box::new(image_filter));
        self
    }
}
