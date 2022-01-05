use super::super::types::{Color, FontMetrics, GlyphIds};
use super::super::{Canvas, Font, Paint, Surface, TextBlob};
use std::rc::Rc;

pub(crate) struct Skia {
    pub surface: Surface,
}

// pub trait SurfaceImpl {
//     fn flush(&self);
//     fn canvas(&self) -> &Canvas;
// }

// pub trait FontImpl {
//     fn get_glyph_ids(&self, text: &str) -> Box<GlyphIds>;
//     fn get_glyph_widths(&self, glyph_ids: &GlyphIds, paint: Option<Rc<Paint>>) -> Vec<f32>;
//     fn get_metrics(&self) -> FontMetrics;
// }

// pub trait TextBlobImpl {
//     fn from_text(string: &str, font: &Font) -> Self;
// }

// pub trait CanvasImpl {
//     fn draw_text_blob(&self, textBlob: &TextBlob, x: f32, y: f32, paint: &Paint);
// }

// pub trait PaintImpl {
//     fn new() -> Self;
//     fn set_color(&self, color: Color);
// }

// pub trait TypefaceImpl {
//     fn new(bytes: &Vec<u8>) -> Self;
// }
