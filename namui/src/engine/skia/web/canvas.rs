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
    pub fn translate(&self, dx: f32, dy: f32) {
        self.0.translate(dx, dy);
    }

    pub(crate) fn save(&self) {
        self.0.save();
    }

    pub(crate) fn clip_path(&self, path: &Path, clip_op: &ClipOp, do_anti_alias: bool) {
        self.0.clipPath(
            &path.canvas_kit_path,
            match clip_op {
                ClipOp::Intersect => canvas_kit().ClipOp().Intersect(),
                ClipOp::Difference => canvas_kit().ClipOp().Difference(),
            },
            do_anti_alias,
        );
    }

    pub(crate) fn restore(&self) {
        self.0.restore();
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        engine::log("Dropping canvas".to_string());
        self.0.delete();
    }
}
