use super::*;
use crate::*;
use namui_type::*;

pub(crate) struct NativeCanvas<'a> {
    canvas: &'a skia_safe::Canvas,
}
impl NativeCanvas<'_> {
    pub(crate) fn new(canvas: &skia_safe::Canvas) -> NativeCanvas {
        NativeCanvas { canvas }
    }
}

impl SkCanvas for NativeCanvas<'_> {
    fn clear(&self, color: Color) {
        self.canvas.clear(color);
    }
    fn draw_text_blob(&self, glyph_ids: Vec<usize>, xy: Xy<Px>, font: &Font, paint: &Paint) {
        let Some(text_blob) = NativeTextBlob::from_glyph_ids(glyph_ids, font) else {
            return;
        };

        self.canvas
            .draw_text_blob(text_blob.skia(), xy, NativePaint::get(paint).skia());
    }
    fn draw_path(&self, path: &Path, paint: &Paint) {
        self.canvas
            .draw_path(NativePath::get(path).skia(), NativePaint::get(paint).skia());
    }
    fn draw_line(&self, from: Xy<Px>, to: Xy<Px>, paint: &Paint) {
        self.canvas
            .draw_line(from, to, NativePaint::get(paint).skia());
    }
    fn translate(&self, dx: Px, dy: Px) {
        self.canvas
            .translate(skia_safe::Point::new(dx.as_f32(), dy.as_f32()));
    }
    fn save(&self) {
        self.canvas.save();
    }
    fn clip_path(&self, path: &Path, clip_op: ClipOp, do_anti_alias: bool) {
        self.canvas.clip_path(
            NativePath::get(path).skia(),
            Some(clip_op.into()),
            do_anti_alias,
        );
    }
    fn restore(&self) {
        self.canvas.restore();
    }
    #[allow(dead_code)]
    fn get_matrix(&self) -> Matrix3x3 {
        let total_matrix = self.canvas.local_to_device_as_3x3();
        Matrix3x3::from_slice([
            [total_matrix[0], total_matrix[1], total_matrix[2]],
            [total_matrix[3], total_matrix[4], total_matrix[5]],
            [total_matrix[6], total_matrix[7], total_matrix[8]],
        ])
    }
    fn set_matrix(&self, matrix: Matrix3x3) {
        self.canvas.set_matrix(&namui_matrix_to_skia_matrix(matrix));
    }
    fn transform(&self, matrix: Matrix3x3) {
        self.canvas.concat_44(&namui_matrix_to_skia_matrix(matrix));
    }
    fn rotate(&self, angle: Angle) {
        self.canvas.rotate(angle.as_degrees(), None);
    }
    fn scale(&self, sx: f32, sy: f32) {
        self.canvas.scale((sx, sy));
    }

    fn draw_image(
        &self,
        image_source: &ImageSource,
        src_rect: Rect<Px>,
        dest_rect: Rect<Px>,
        paint: &Option<Paint>,
    ) {
        let Some(image) = NativeImage::get(image_source) else {
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

fn namui_matrix_to_skia_matrix(matrix: Matrix3x3) -> skia_safe::M44 {
    skia_safe::M44::new(
        matrix.index_0_0(),
        matrix.index_0_1(),
        matrix.index_0_2(),
        0.0,
        matrix.index_1_0(),
        matrix.index_1_1(),
        matrix.index_1_2(),
        0.0,
        matrix.index_2_0(),
        matrix.index_2_1(),
        matrix.index_2_2(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
