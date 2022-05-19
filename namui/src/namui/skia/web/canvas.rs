use crate::{namui, XywhRect};

use super::*;

pub(crate) struct Canvas(pub CanvasKitCanvas);
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
            clip_op.into_canvas_kit(),
            do_anti_alias,
        );
    }
    pub(crate) fn restore(&self) {
        self.0.restore();
    }
    pub(crate) fn draw_image_rect_options(
        &self,
        image: &Image,
        src_rect: &XywhRect<f32>,
        dest_rect: &XywhRect<f32>,
        filter_mode: FilterMode,
        mipmap_mode: MipmapMode,
        paint: Option<&Paint>,
    ) {
        let src_rect_lrtb = src_rect.into_ltrb();
        let src_rect_lrtb_array = js_sys::Float32Array::new_with_length(4);
        src_rect_lrtb_array.set_index(0, src_rect_lrtb.left as f32);
        src_rect_lrtb_array.set_index(1, src_rect_lrtb.top as f32);
        src_rect_lrtb_array.set_index(2, src_rect_lrtb.right as f32);
        src_rect_lrtb_array.set_index(3, src_rect_lrtb.bottom as f32);

        let dest_rect_lrtb = dest_rect.into_ltrb();
        let dest_rect_lrtb_array = js_sys::Float32Array::new_with_length(4);
        dest_rect_lrtb_array.set_index(0, dest_rect_lrtb.left as f32);
        dest_rect_lrtb_array.set_index(1, dest_rect_lrtb.top as f32);
        dest_rect_lrtb_array.set_index(2, dest_rect_lrtb.right as f32);
        dest_rect_lrtb_array.set_index(3, dest_rect_lrtb.bottom as f32);

        self.0.drawImageRectOptions(
            &image.canvas_kit_image,
            src_rect_lrtb_array,
            dest_rect_lrtb_array,
            filter_mode.into_canvas_kit(),
            mipmap_mode.into_canvas_kit(),
            paint.map(|p| &p.0),
        );
    }
    pub(crate) fn get_matrix(&self) -> [[f32; 3]; 3] {
        let total_matrix = self.0.getTotalMatrix();
        return [
            [total_matrix[0], total_matrix[1], total_matrix[2]],
            [total_matrix[3], total_matrix[4], total_matrix[5]],
            [total_matrix[6], total_matrix[7], total_matrix[8]],
        ];
    }
    pub(crate) fn set_matrix(&self, matrix: &[[f32; 3]; 3]) {
        let current_matrix = self.0.getTotalMatrix();
        let inverted = canvas_kit().Matrix().invert(&current_matrix);
        self.0.concat(&inverted);
        self.0.concat(&[
            matrix[0][0],
            matrix[0][1],
            matrix[0][2],
            matrix[1][0],
            matrix[1][1],
            matrix[1][2],
            matrix[2][0],
            matrix[2][1],
            matrix[2][2],
        ]);
    }
    pub(crate) fn concat_matrix(&self, matrix: &[[f32; 3]; 3]) {
        self.0.concat(&[
            matrix[0][0],
            matrix[0][1],
            matrix[0][2],
            matrix[1][0],
            matrix[1][1],
            matrix[1][2],
            matrix[2][0],
            matrix[2][1],
            matrix[2][2],
        ]);
    }

    pub(crate) fn draw_text(&self, string: &str, x: f32, y: f32, paint: &Paint, font: &Font) {
        self.0
            .drawText(string, x, y, &paint.0, &font.canvas_kit_font);
    }
    pub(crate) fn rotate(&self, radian: f32) {
        self.0
            .rotate(radian / (2.0 * std::f32::consts::PI) * 360.0, 0.0, 0.0);
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.0.delete();
    }
}
