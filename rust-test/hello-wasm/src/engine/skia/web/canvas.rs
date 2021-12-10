use crate::engine;

use super::*;

pub struct Canvas(pub CanvasKitCanvas);
impl Canvas {
    pub fn draw_text_blob(&self, text_blob: &TextBlob, x: f32, y: f32, paint: &Paint) {
        self.0.drawTextBlob(&text_blob.0, x, y, &paint.0);
    }
    pub fn draw_path(&self, path: &Path, paint: &Paint) {
        self.0.drawPath(&path.canvas_kit_path, &paint.0);
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        engine::log("Dropping canvas".to_string());
        self.0.delete();
    }
}
