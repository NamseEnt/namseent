use super::*;

unsafe impl Sync for CanvasKitTextBlob {}
unsafe impl Send for CanvasKitTextBlob {}
pub struct TextBlob(pub CanvasKitTextBlob);
impl TextBlob {
    pub fn from_text(string: &str, font: &Font) -> Self {
        TextBlob(
            canvas_kit()
                .TextBlob()
                .MakeFromText(string, &font.canvas_kit_font),
        )
    }
    pub fn from_glyph_ids(glyph_ids: &GlyphIds, font: &Font) -> Self {
        TextBlob(
            canvas_kit()
                .TextBlob()
                .MakeFromGlyphs(glyph_ids, &font.canvas_kit_font),
        )
    }
}
impl Drop for TextBlob {
    fn drop(&mut self) {
        self.0.delete();
    }
}
