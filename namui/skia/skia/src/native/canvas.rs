use super::*;
use crate::*;
use namui_type::*;

impl SkCanvas for skia_safe::Canvas {
    fn clear(&self, color: Color) {
        self.clear(color);
    }
    fn draw_text_blob(&self, glyph_ids: GlyphIds, xy: Xy<Px>, font: &Font, paint: &Paint) {
        let Some(text_blob) = NativeTextBlob::from_glyph_ids(glyph_ids, font) else {
            println!("text_blob not found");
            return;
        };
        self.draw_text_blob(text_blob.skia(), xy, NativePaint::get(paint).skia());
    }
    fn draw_path(&self, path: &Path, paint: &Paint) {
        self.draw_path(NativePath::get(path).skia(), NativePaint::get(paint).skia());
    }
    fn draw_line(&self, from: Xy<Px>, to: Xy<Px>, paint: &Paint) {
        self.draw_line(from, to, NativePaint::get(paint).skia());
    }
    fn draw_image(
        &self,
        image_source: &ImageSource,
        src_rect: Rect<Px>,
        dest_rect: Rect<Px>,
        paint: &Option<Paint>,
    ) {
        let Some(image) = NativeImage::get(image_source) else {
            println!("image not loaded");
            return;
        };

        let mut paint = paint.clone().unwrap_or(Paint::new(Color::WHITE));
        let image_shader = image.get_default_shader();

        let next_shader = if let Some(super_shader) = &paint.shader {
            super_shader.blend(BlendMode::Plus, &image_shader)
        } else {
            image_shader
        };

        paint = paint.set_shader(next_shader);

        self.save();
        self.transform(
            TransformMatrix::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
                * TransformMatrix::from_scale(
                    dest_rect.width() / src_rect.width(),
                    dest_rect.height() / src_rect.height(),
                )
                * TransformMatrix::from_translate(-src_rect.x().as_f32(), -src_rect.y().as_f32()),
        );

        SkCanvas::draw_path(self, &Path::new().add_rect(src_rect), &paint);

        self.restore();
    }
    fn translate(&self, dx: Px, dy: Px) {
        self.translate(skia_safe::Point::new(dx.as_f32(), dy.as_f32()));
    }
    fn save(&self) {
        self.save();
    }
    fn clip_path(&self, path: &Path, clip_op: ClipOp, do_anti_alias: bool) {
        self.clip_path(
            NativePath::get(path).skia(),
            Some(clip_op.into()),
            do_anti_alias,
        );
    }
    fn restore(&self) {
        self.restore();
    }
    #[allow(dead_code)]
    fn get_matrix(&self) -> TransformMatrix {
        let total_matrix = self.local_to_device_as_3x3();
        TransformMatrix::from_slice([
            [total_matrix[0], total_matrix[1], total_matrix[2]],
            [total_matrix[3], total_matrix[4], total_matrix[5]],
        ])
    }
    fn set_matrix(&self, matrix: TransformMatrix) {
        self.set_matrix(&namui_matrix_to_skia_matrix(matrix));
    }
    fn transform(&self, matrix: TransformMatrix) {
        self.concat_44(&namui_matrix_to_skia_matrix(matrix));
    }

    fn rotate(&self, angle: Angle) {
        self.rotate(angle.as_degrees(), None);
    }

    fn scale(&self, sx: f32, sy: f32) {
        self.scale((sx, sy));
    }
}

fn namui_matrix_to_skia_matrix(matrix: TransformMatrix) -> skia_safe::M44 {
    skia_safe::Matrix::new_all(
        matrix[0][0].into(),
        matrix[0][1].into(),
        matrix[0][2].into(),
        matrix[1][0].into(),
        matrix[1][1].into(),
        matrix[1][2].into(),
        0.0,
        0.0,
        1.0,
    )
    .into()
}
