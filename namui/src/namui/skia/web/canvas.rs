use super::*;
use crate::{namui::render::Matrix3x3, *};

pub(crate) struct Canvas(pub CanvasKitCanvas);
impl Canvas {
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
    pub(crate) fn draw_image_rect_options(
        &self,
        image: &Image,
        src_rect: Rect<Px>,
        dest_rect: Rect<Px>,
        filter_mode: FilterMode,
        mipmap_mode: MipmapMode,
        paint: Option<&Paint>,
    ) {
        let src_rect_lrtb = src_rect.as_ltrb();
        let src_rect_lrtb_array = js_sys::Float32Array::new_with_length(4);
        src_rect_lrtb_array.set_index(0, src_rect_lrtb.left.into());
        src_rect_lrtb_array.set_index(1, src_rect_lrtb.top.into());
        src_rect_lrtb_array.set_index(2, src_rect_lrtb.right.into());
        src_rect_lrtb_array.set_index(3, src_rect_lrtb.bottom.into());

        let dest_rect_lrtb = dest_rect.as_ltrb();
        let dest_rect_lrtb_array = js_sys::Float32Array::new_with_length(4);
        dest_rect_lrtb_array.set_index(0, dest_rect_lrtb.left.into());
        dest_rect_lrtb_array.set_index(1, dest_rect_lrtb.top.into());
        dest_rect_lrtb_array.set_index(2, dest_rect_lrtb.right.into());
        dest_rect_lrtb_array.set_index(3, dest_rect_lrtb.bottom.into());

        self.0.drawImageRectOptions(
            &image.canvas_kit_image,
            src_rect_lrtb_array,
            dest_rect_lrtb_array,
            filter_mode.into_canvas_kit(),
            mipmap_mode.into_canvas_kit(),
            paint.map(|paint| &paint.canvas_kit_paint),
        );
    }
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
    pub(crate) fn rotate(&self, angle: Angle) {
        self.0.rotate(angle.as_degrees(), 0.0, 0.0);
    }
    pub(crate) fn scale(&self, sx: f32, sy: f32) {
        self.0.scale(sx, sy);
    }
}
