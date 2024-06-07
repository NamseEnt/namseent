use crate::*;
use std::fmt::Debug;

pub trait GroupGlyph: Debug {
    fn groups(&self, text: &str) -> Vec<GlyphGroup>;
    fn width(&self, text: &str) -> Px;
    fn widths(&self, text: &str) -> Vec<Px>;
    fn font_metrics(&self) -> FontMetrics;
    fn bounds(&self, text: &str) -> Vec<Rect<Px>>;
    fn bound(&self, text: &str) -> Rect<Px>;
}

impl GroupGlyph for &dyn GroupGlyph {
    fn groups(&self, text: &str) -> Vec<GlyphGroup> {
        (*self).groups(text)
    }

    fn width(&self, text: &str) -> Px {
        (*self).width(text)
    }

    fn widths(&self, text: &str) -> Vec<Px> {
        (*self).widths(text)
    }

    fn font_metrics(&self) -> FontMetrics {
        (*self).font_metrics()
    }

    fn bounds(&self, text: &str) -> Vec<Rect<Px>> {
        (*self).bounds(text)
    }

    fn bound(&self, text: &str) -> Rect<Px> {
        (*self).bound(text)
    }
}

pub struct GlyphGroup {
    pub glyphs: Vec<Glyph>,
    pub font: Font,
    pub width: Px,
}

pub struct Glyph {
    pub id: GlyphId,
    pub width: Px,
}
