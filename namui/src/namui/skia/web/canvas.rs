use crate::{namui, XywhRect};

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
            match filter_mode {
                FilterMode::Linear => canvas_kit().FilterMode().Linear(),
                FilterMode::Nearest => canvas_kit().FilterMode().Nearest(),
            },
            match mipmap_mode {
                MipmapMode::None => canvas_kit().MipmapMode().None(),
                MipmapMode::Linear => canvas_kit().MipmapMode().Linear(),
                MipmapMode::Nearest => canvas_kit().MipmapMode().Nearest(),
            },
            paint.map(|p| &p.0),
        );
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.0.delete();
    }
}
