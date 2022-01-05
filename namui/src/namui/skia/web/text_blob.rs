use crate::namui;

use super::*;

unsafe impl Sync for CanvasKitTextBlob {}
unsafe impl Send for CanvasKitTextBlob {}
pub struct TextBlob(pub CanvasKitTextBlob);
impl TextBlob {
    pub fn from_text(string: &str, font: &Font) -> Self {
        TextBlob(canvas_kit().TextBlob().MakeFromText(string, &font.0))
    }
}
impl Drop for TextBlob {
    fn drop(&mut self) {
        self.0.delete();
    }
}
