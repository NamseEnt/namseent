use crate::*;

#[derive(Debug, PartialEq, Clone, Eq, Hash, State)]
pub struct Font {
    pub size: IntPx,
    pub name: String,
}

impl Font {
    pub fn groups(&self, text: &str, paint: &Paint) -> Vec<GlyphGroup> {
        // TODO: Handle fallback font
        let Some(native_font) = NativeFont::get(self) else {
            return vec![];
        };

        let glyph_ids = native_font.glyph_ids(text);
        let glyph_widths = native_font.glyph_widths(glyph_ids.clone(), paint);

        let glyphs = glyph_ids
            .into_iter()
            .zip(glyph_widths)
            .map(|(id, width)| Glyph { id, width })
            .collect::<Vec<_>>();

        let width = glyphs.iter().map(|glyph| glyph.width).sum();

        vec![GlyphGroup {
            font: self.clone(),
            glyphs,
            width,
        }]
    }

    pub fn width(&self, text: &str, paint: &Paint) -> Px {
        self.groups(text, paint)
            .into_iter()
            .map(|group| group.width)
            .sum()
    }

    pub fn widths(&self, text: &str, paint: &Paint) -> Vec<Px> {
        self.groups(text, paint)
            .into_iter()
            .flat_map(|group| group.glyphs.into_iter().map(|glyph| glyph.width))
            .collect()
    }

    pub fn font_metrics(&self) -> FontMetrics {
        match NativeFont::get(self) {
            Some(font) => font.metrics,
            None => FontMetrics::default(),
        }
    }

    pub fn bounds(&self, text: &str, paint: &Paint) -> Vec<Rect<Px>> {
        let Some(native_font) = NativeFont::get(self) else {
            return vec![];
        };

        let glyph_ids = native_font.glyph_ids(text);
        native_font.glyph_bounds(glyph_ids.clone(), paint)
    }

    pub fn bound(&self, text: &str, paint: &Paint) -> Rect<Px> {
        self.bounds(text, paint)
            .into_iter()
            .fold(Rect::default(), |a, b| {
                a.get_minimum_rectangle_containing(b)
            })
    }
}
