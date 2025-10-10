use crate::*;

pub struct GlyphGroup {
    pub glyphs: Vec<Glyph>,
    pub font: Font,
    pub width: Px,
}

pub struct Glyph {
    pub id: GlyphId,
    pub width: Px,
}
