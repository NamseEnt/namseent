use crate::{
    namui::skia::{Shader, StrokeJoin},
    BlendMode, Color, ColorFilter, Paint, PaintStyle, Px, StrokeCap,
};
use once_cell::sync::OnceCell;
use ordered_float::OrderedFloat;
use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq)]
pub struct PaintBuilder {
    color: Option<Color>,
    paint_style: Option<PaintStyle>,
    anti_alias: Option<bool>,
    stroke_width: Option<Px>,
    stroke_cap: Option<StrokeCap>,
    stroke_join: Option<StrokeJoin>,
    color_filter: Option<(Color, BlendMode)>,
    shader: Option<Arc<Shader>>,
}

#[derive(Clone, PartialEq)]
pub struct PaintBuilderWithoutShader {
    color: Option<Color>,
    paint_style: Option<PaintStyle>,
    anti_alias: Option<bool>,
    stroke_width: Option<Px>,
    stroke_cap: Option<StrokeCap>,
    stroke_join: Option<StrokeJoin>,
    color_filter: Option<(Color, BlendMode)>,
}

static PAINT_CACHE: OnceCell<Mutex<lru::LruCache<PaintBuilderWithoutShader, Arc<Paint>>>> =
    OnceCell::new();
static COLOR_FILTER_CACHE: OnceCell<Mutex<lru::LruCache<(Color, BlendMode), Arc<ColorFilter>>>> =
    OnceCell::new();

impl PaintBuilder {
    pub(crate) fn build(&self) -> Arc<Paint> {
        self.get_or_create_paint()
    }
    pub fn new() -> Self {
        Self {
            color: None,
            paint_style: None,
            anti_alias: None,
            stroke_width: None,
            stroke_cap: None,
            stroke_join: None,
            color_filter: None,
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
    pub fn set_color_filter(mut self, color: Color, blend_mode: BlendMode) -> Self {
        self.color_filter = Some((color, blend_mode));
        self
    }
    pub fn set_shader(mut self, make_shader: Arc<Shader>) -> Self {
        self.shader = Some(make_shader);
        self
    }

    fn get_or_create_paint(&self) -> Arc<Paint> {
        let paint_builder_without_shader = PaintBuilderWithoutShader {
            color: self.color,
            paint_style: self.paint_style,
            anti_alias: self.anti_alias,
            stroke_width: self.stroke_width,
            stroke_cap: self.stroke_cap,
            stroke_join: self.stroke_join,
            color_filter: self.color_filter,
        };
        let mut cache = PAINT_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap();
        let paint = match cache.get(&paint_builder_without_shader) {
            Some(paint) => paint.clone(),
            None => {
                let paint = self.create_paint();
                cache.put(paint_builder_without_shader, paint.clone());
                paint
            }
        };

        paint.set_shader(self.shader.as_ref());
        paint
    }

    fn create_paint(&self) -> Arc<Paint> {
        let id = self.generate_id();
        let paint = Paint::new(
            id,
            self.color,
            self.paint_style.as_ref(),
            self.anti_alias,
            self.stroke_width,
            self.stroke_cap.as_ref(),
            self.stroke_join.as_ref(),
            self.color_filter
                .as_ref()
                .map(|(color, blend_mode)| Self::get_or_create_color_filter(*color, *blend_mode)),
        );

        Arc::new(paint)
    }
    fn get_or_create_color_filter(color: Color, blend_mode: BlendMode) -> Arc<ColorFilter> {
        let mut cache = COLOR_FILTER_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap();
        let key = (color.clone(), blend_mode.clone());
        match cache.get(&key) {
            Some(color_filter) => color_filter.clone(),
            None => {
                let color_filter = Arc::new(ColorFilter::blend(color, blend_mode));
                cache.put(key, color_filter.clone());
                color_filter
            }
        }
    }

    fn generate_id(&self) -> Box<[u8]> {
        bincode::serialize(&(
            &self.color,
            &self.paint_style,
            &self.anti_alias,
            &self.stroke_width,
            &self.stroke_cap,
            &self.color_filter,
        ))
        .unwrap()
        .into()
    }
}

impl Hash for PaintBuilderWithoutShader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.paint_style.hash(state);
        self.anti_alias.hash(state);
        self.stroke_width
            .map(|value| OrderedFloat(value.as_f32()))
            .hash(state);
        self.stroke_cap.hash(state);
        self.color_filter.hash(state);
    }
}

impl std::cmp::Eq for PaintBuilderWithoutShader {}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn paint_builder_generate_id_should_unique_per_setting() {
        let paint_builder_1 = PaintBuilder::new().set_color(Color::BLACK);
        let paint_builder_2 = PaintBuilder::new().set_color(Color::BLACK);
        let paint_builder_3 = PaintBuilder::new().set_color(Color::RED);

        assert_eq!(paint_builder_1.generate_id(), paint_builder_2.generate_id());
        assert_ne!(paint_builder_1.generate_id(), paint_builder_3.generate_id());
    }
}
