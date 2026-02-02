use crate::*;

pub trait SkCanvas {
    fn clear(&self, color: Color);
    fn draw_text_blob(&self, glyph_ids: GlyphIds, xy: Xy<Px>, font: &Font, paint: &Paint);
    fn draw_path(&self, path: &Path, paint: &Paint);
    fn draw_line(&self, from: Xy<Px>, to: Xy<Px>, paint: &Paint);
    fn draw_image(
        &self,
        image: &Image,
        src_rect: Rect<Px>,
        dest_rect: Rect<Px>,
        paint: &Option<Paint>,
    );
    /// Draws multiple sprites from an atlas image in a single draw call.
    ///
    /// * `atlas` - The atlas image containing all sprites
    /// * `xforms` - RSXform transformations for each sprite
    /// * `tex_rects` - Source rectangles within the atlas for each sprite
    /// * `paint` - Optional paint to apply to all sprites
    fn draw_atlas(
        &self,
        atlas: &Image,
        xforms: &[RSXform],
        tex_rects: &[Rect<Px>],
        paint: &Option<Paint>,
    );
    fn translate(&self, dx: Px, dy: Px);
    fn save(&self);
    fn clip_path(&self, path: &Path, clip_op: ClipOp, do_anti_alias: bool);
    fn restore(&self);
    fn get_matrix(&self) -> TransformMatrix;
    fn set_matrix(&self, matrix: TransformMatrix);
    fn transform(&self, matrix: TransformMatrix);
    fn rotate(&self, angle: Angle);
    fn scale(&self, sx: f32, sy: f32);
}
