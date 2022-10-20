use super::*;
use crate::{namui::render::Matrix3x3, *};

pub(crate) struct Canvas(pub CanvasKitCanvas);
impl Canvas {
    pub fn clear(&self, color: Color) {
        self.0.clear(&color.into_float32_array());
    }
    pub fn draw_text_blob(&self, text_blob: &TextBlob, x: Px, y: Px, paint: &Paint) {
        self.0
            .drawTextBlob(&text_blob.0, x.into(), y.into(), &paint.canvas_kit_paint);
    }
    pub fn draw_path(&self, path: &Path, paint: &Paint) {
        self.0
            .drawPath(&path.canvas_kit_path, &paint.canvas_kit_paint);
    }
    pub fn translate(&self, dx: Px, dy: Px) {
        self.0.translate(dx.into(), dy.into());
    }
    pub(crate) fn save(&self) {
        self.0.save();
    }
    pub(crate) fn clip_path(&self, path: &Path, clip_op: &ClipOp, do_anti_alias: bool) {
        self.0.clipPath(
            &path.canvas_kit_path,
            clip_op.into_canvas_kit(),
            do_anti_alias,
        );
    }
    pub(crate) fn restore(&self) {
        self.0.restore();
    }
    #[allow(dead_code)]
    pub(crate) fn get_matrix(&self) -> Matrix3x3 {
        let total_matrix = self.0.getTotalMatrix();
        return Matrix3x3::from_slice([
            [total_matrix[0], total_matrix[1], total_matrix[2]],
            [total_matrix[3], total_matrix[4], total_matrix[5]],
            [total_matrix[6], total_matrix[7], total_matrix[8]],
        ]);
    }
    pub(crate) fn set_matrix(&self, matrix: Matrix3x3) {
        let current_matrix = self.0.getTotalMatrix();
        let inverted = canvas_kit().Matrix().invert(&current_matrix);
        self.0.concat(&inverted);
        self.0.concat(&matrix.into_linear_slice());
    }
    pub(crate) fn transform(&self, matrix: Matrix3x3) {
        self.0.concat(&matrix.into_linear_slice());
    }
    pub(crate) fn rotate(&self, angle: Angle) {
        self.0.rotate(angle.as_degrees(), 0.0, 0.0);
    }
    pub(crate) fn scale(&self, sx: f32, sy: f32) {
        self.0.scale(sx, sy);
    }
}
