pub use super::base::*;
use super::*;

unsafe impl Sync for CanvasKitFont {}
unsafe impl Send for CanvasKitFont {}
pub struct Font {
    pub(crate) id: String,
    pub(crate) canvas_kit_font: CanvasKitFont,
    pub(crate) size: i16,
}

impl Font {
    pub fn generate_id(typeface: &Typeface, size: i16) -> String {
        format!("{}-{}", typeface.id, size)
    }
    pub fn new(typeface: &Typeface, size: i16) -> Self {
        Font {
            id: Self::generate_id(typeface, size),
            canvas_kit_font: CanvasKitFont::new(&typeface.canvas_kit_typeface, size),
            size,
        }
    }
    pub fn get_glyph_ids(&self, text: &str) -> Box<GlyphIds> {
        let canvas_kit_font = &self.canvas_kit_font;
        canvas_kit_font.getGlyphIDs(text)
    }
    pub(crate) fn get_glyph_widths(&self, glyph_ids: &GlyphIds, paint: Option<&Paint>) -> Vec<f32> {
        let canvas_kit_font = &self.canvas_kit_font;
        let widths = canvas_kit_font.getGlyphWidths(glyph_ids, paint.map(|p| &p.0));
        widths.to_vec()
    }

    pub fn get_metrics(&self) -> FontMetrics {
        let canvas_kit_font = &self.canvas_kit_font;
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
    pub(crate) fn get_glyph_bounds(
        &self,
        glyph_ids: &GlyphIds,
        paint: Option<&Paint>,
    ) -> Vec<LtrbRect> {
        let canvas_kit_font = &self.canvas_kit_font;
        let bound_items = canvas_kit_font
            .getGlyphBounds(glyph_ids, paint.map(|p| &p.0))
            .to_vec();

        let mut iter = bound_items.iter().peekable();
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
        self.canvas_kit_font.delete();
    }
}

impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
