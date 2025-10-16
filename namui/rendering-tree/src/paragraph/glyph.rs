use crate::*;

#[derive(Debug)]
pub struct GlyphGroup {
    pub glyphs: Vec<Glyph>,
    pub font: Font,
    pub width: Px,
}

#[derive(Debug)]
pub struct Glyph {
    pub id: GlyphId,
    pub width: Px,
}
