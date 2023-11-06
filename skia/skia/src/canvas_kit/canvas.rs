use super::*;
use crate::*;
use namui_type::*;

pub(crate) struct CkCanvas {
    canvas_kit_canvas: CanvasKitCanvas,
}
impl CkCanvas {
    pub(crate) fn new(canvas_kit_canvas: CanvasKitCanvas) -> CkCanvas {
        CkCanvas { canvas_kit_canvas }
    }
}

impl SkCanvas for CkCanvas {
    fn clear(&self, color: Color) {
        self.canvas_kit_canvas.clear(&color.to_float32_array());
    }
    fn draw_text_blob(&self, glyph_ids: Vec<usize>, xy: Xy<Px>, font: &Font, paint: &Paint) {
        let Some(text_blob) = CkTextBlob::from_glyph_ids(glyph_ids, font) else {
            return;
        };

        self.canvas_kit_canvas.drawTextBlob(
            text_blob.canvas_kit(),
            xy.x.into(),
            xy.y.into(),
            CkPaint::get(paint).canvas_kit(),
        );
    }
    fn draw_path(&self, path: &Path, paint: &Paint) {
        self.canvas_kit_canvas.drawPath(
            CkPath::get(path).canvas_kit(),
            CkPaint::get(paint).canvas_kit(),
        );
    }
    fn draw_line(&self, from: Xy<Px>, to: Xy<Px>, paint: &Paint) {
        self.canvas_kit_canvas.drawLine(
            from.x.as_f32(),
            from.y.as_f32(),
            to.x.as_f32(),
            to.y.as_f32(),
            CkPaint::get(paint).canvas_kit(),
        );
    }
    fn translate(&self, dx: Px, dy: Px) {
        self.canvas_kit_canvas.translate(dx.into(), dy.into());
    }
    fn save(&self) {
        self.canvas_kit_canvas.save();
    }
    fn clip_path(&self, path: &Path, clip_op: ClipOp, do_anti_alias: bool) {
        self.canvas_kit_canvas.clipPath(
            CkPath::get(path).canvas_kit(),
            clip_op.into(),
            do_anti_alias,
        );
    }
    fn restore(&self) {
        self.canvas_kit_canvas.restore();
    }
    #[allow(dead_code)]
    fn get_matrix(&self) -> Matrix3x3 {
        let total_matrix = self.canvas_kit_canvas.getTotalMatrix();
        Matrix3x3::from_slice([
            [total_matrix[0], total_matrix[1], total_matrix[2]],
            [total_matrix[3], total_matrix[4], total_matrix[5]],
            [total_matrix[6], total_matrix[7], total_matrix[8]],
        ])
    }
    fn set_matrix(&self, matrix: Matrix3x3) {
        let current_matrix = self.canvas_kit_canvas.getTotalMatrix();
        let inverted = canvas_kit().Matrix().invert(&current_matrix);
        self.canvas_kit_canvas.concat(&inverted);
        self.canvas_kit_canvas.concat(&matrix.into_linear_slice());
    }
    fn transform(&self, matrix: Matrix3x3) {
        self.canvas_kit_canvas.concat(&matrix.into_linear_slice());
    }
    fn rotate(&self, angle: Angle) {
        self.canvas_kit_canvas.rotate(angle.as_degrees(), 0.0, 0.0);
    }
    fn scale(&self, sx: f32, sy: f32) {
        self.canvas_kit_canvas.scale(sx, sy);
    }

    fn draw_image(
        &self,
        image_source: &ImageSource,
        src_rect: Rect<Px>,
        dest_rect: Rect<Px>,
        paint: &Option<Paint>,
    ) {
        let Some(image) = CkImage::get(image_source) else {
            return;
        };

        let mut paint = paint.clone().unwrap_or_default();

        let image_shader = image.get_default_shader();

        let next_shader = if let Some(super_shader) = &paint.shader {
            super_shader.blend(BlendMode::Plus, &image_shader)
        } else {
            image_shader
        };

        paint = paint.set_shader(next_shader);

        self.save();
        self.transform(
            Matrix3x3::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
                * Matrix3x3::from_scale(
                    dest_rect.width() / src_rect.width(),
                    dest_rect.height() / src_rect.height(),
                ),
        );

        self.draw_path(&Path::new().add_rect(src_rect), &paint);

        self.restore();
    }
}
