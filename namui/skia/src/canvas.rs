use crate::*;

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
        image: &Image,
        src_rect: Rect<Px>,
        dest_rect: Rect<Px>,
        paint: &Option<Paint>,
    ) {
        let mut paint = paint.clone().unwrap_or(Paint::new(Color::WHITE));
        let image_shader = image.get_default_shader();

        let next_shader = match &paint.shader {
            Some(super_shader) => super_shader.blend(BlendMode::Plus, &image_shader),
            _ => image_shader,
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

    fn draw_atlas(
        &self,
        atlas: &Image,
        xforms: &[RSXform],
        tex_rects: &[Rect<Px>],
        paint: &Option<Paint>,
    ) {
        if xforms.is_empty() || xforms.len() != tex_rects.len() {
            return;
        }

        // Convert RSXform to skia_safe::RSXform
        let skia_xforms: Vec<skia_safe::RSXform> = xforms
            .iter()
            .map(|xform| {
                skia_safe::RSXform::new(
                    xform.scos,
                    xform.ssin,
                    (xform.tx.as_f32(), xform.ty.as_f32()),
                )
            })
            .collect();

        // Convert Rect<Px> to skia_safe::Rect
        let skia_tex_rects: Vec<skia_safe::Rect> = tex_rects
            .iter()
            .map(|rect| {
                skia_safe::Rect::new(
                    rect.left().as_f32(),
                    rect.top().as_f32(),
                    rect.right().as_f32(),
                    rect.bottom().as_f32(),
                )
            })
            .collect();

        // Get the native skia image
        let skia_image = atlas.skia_image();

        // Prepare paint if provided
        let skia_paint = paint.as_ref().map(|p| NativePaint::get(p).skia().clone());

        self.draw_atlas(
            &skia_image,
            &skia_xforms,
            &skia_tex_rects,
            None, // colors
            skia_safe::BlendMode::SrcOver,
            skia_safe::SamplingOptions::default(),
            None, // cull_rect
            skia_paint.as_ref(),
        );
    }
}

fn namui_matrix_to_skia_matrix(matrix: TransformMatrix) -> skia_safe::M44 {
    skia_safe::Matrix::new_all(
        matrix[0][0].as_f32(),
        matrix[0][1].as_f32(),
        matrix[0][2].as_f32(),
        matrix[1][0].as_f32(),
        matrix[1][1].as_f32(),
        matrix[1][2].as_f32(),
        0.0,
        0.0,
        1.0,
    )
    .into()
}
