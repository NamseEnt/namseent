use crate::{
    namui::skia::StrokeJoin, BlendMode, Color, ColorFilter, Paint, PaintStyle, StrokeCap, Xy,
};
use once_cell::sync::OnceCell;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct PaintBuilder {
    color: Option<Color>,
    paint_style: Option<PaintStyle>,
    anti_alias: Option<bool>,
    stroke_width: Option<f32>,
    stroke_cap: Option<StrokeCap>,
    stroke_join: Option<StrokeJoin>,
    color_filter: Option<(Color, BlendMode)>,
}

static PAINT_CACHE: OnceCell<Mutex<lru::LruCache<PaintBuilder, Arc<Paint>>>> = OnceCell::new();
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
    pub fn set_stroke_width(mut self, width: f32) -> Self {
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
    pub fn set_color_filter(mut self, color: &Color, blend_mode: &BlendMode) -> Self {
        self.color_filter = Some((color.clone(), blend_mode.clone()));
        self
    }

    fn get_or_create_paint(&self) -> Arc<Paint> {
        let mut cache = PAINT_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap();
        match cache.get(self) {
            Some(paint) => paint.clone(),
            None => {
                let paint = self.create_paint();
                cache.put(self.clone(), paint.clone());
                paint
            }
        }
    }

    fn create_paint(&self) -> Arc<Paint> {
        let mut paint = Paint::new();

        if let Some(color) = self.color {
            paint = paint.set_color(color);
        }
        if let Some(style) = &self.paint_style {
            paint = paint.set_style(style);
        }
        if let Some(value) = self.anti_alias {
            paint = paint.set_anti_alias(value);
        }
        if let Some(width) = self.stroke_width {
            paint = paint.set_stroke_width(width);
        }
        if let Some(cap) = &self.stroke_cap {
            paint = paint.set_stroke_cap(cap);
        }
        if let Some(join) = &self.stroke_join {
            paint = paint.set_stroke_join(join);
        }
        if let Some((color, blend_mode)) = &self.color_filter {
            let color_filter = Self::get_or_create_color_filter(&color, &blend_mode);
            paint = paint.set_color_filter(&color_filter);
        }

        Arc::new(paint)
    }
    fn get_or_create_color_filter(color: &Color, blend_mode: &BlendMode) -> Arc<ColorFilter> {
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
}

impl Hash for PaintBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.paint_style.hash(state);
        self.anti_alias.hash(state);
        self.stroke_width
            .map(|value| OrderedFloat(value))
            .hash(state);
        self.stroke_cap.hash(state);
        self.color_filter.hash(state);
    }
}

impl std::cmp::Eq for PaintBuilder {}
