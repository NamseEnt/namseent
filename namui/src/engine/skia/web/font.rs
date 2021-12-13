use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::engine;

pub use super::base::*;
use super::*;

unsafe impl Sync for CanvasKitFont {}
unsafe impl Send for CanvasKitFont {}
pub struct Font(pub Box<CanvasKitFont>);

impl Font {
    pub fn new(typeface: &Typeface, size: &i16) -> Self {
        Font(Box::new(CanvasKitFont::new(&typeface.0, size.clone())))
    }
    pub fn get_glyph_ids(&self, text: &str) -> Box<GlyphIds> {
        let canvas_kit_font = &self.0;
        canvas_kit_font.getGlyphIDs(text)
    }
    pub fn get_glyph_widths(&self, glyph_ids: &GlyphIds, paint: Option<&Paint>) -> Vec<f32> {
        let canvas_kit_font = &self.0;
        let widths = canvas_kit_font.getGlyphWidths(glyph_ids, paint.map(|p| &p.0));
        widths.to_vec()
    }

    pub fn get_metrics(&self) -> FontMetrics {
        let canvas_kit_font = &self.0;
        let canvas_kit_font_metrics = &canvas_kit_font.getMetrics();
        let bounds = canvas_kit_font_metrics.bounds().map(|numbers| LtrbRect {
            left: numbers[0],
            top: numbers[1],
            right: numbers[2],
            bottom: numbers[3],
        });

        FontMetrics {
            ascent: canvas_kit_font_metrics.ascent(),
            descent: canvas_kit_font_metrics.descent(),
            leading: canvas_kit_font_metrics.leading(),
            bounds,
        }
    }
    pub fn get_glyph_bounds(&self, glyph_ids: &GlyphIds, paint: Option<&Paint>) -> Vec<LtrbRect> {
        let canvas_kit_font = &self.0;
        let boundItems = canvas_kit_font
            .getGlyphBounds(glyph_ids, paint.map(|p| &p.0))
            .to_vec();

        let mut iter = boundItems.iter().peekable();
        let mut bounds = Vec::new();

        while iter.peek().is_some() {
            bounds.push(LtrbRect {
                left: *iter.next().unwrap(),
                top: *iter.next().unwrap(),
                right: *iter.next().unwrap(),
                bottom: *iter.next().unwrap(),
            });
        }

        bounds
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        engine::log("Dropping font".to_string());
        self.0.delete();
    }
}
