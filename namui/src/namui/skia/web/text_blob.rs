use crate::namui;

use super::*;

pub struct TextBlob(pub CanvasKitTextBlob);
impl TextBlob {
    pub fn from_text(string: &str, font: &Font) -> Self {
        TextBlob(canvas_kit().TextBlob().MakeFromText(string, &font.0))
    }
}
impl Drop for TextBlob {
    fn drop(&mut self) {
        namui::log("Dropping textBlob".to_string());
        self.0.delete();
    }
}
