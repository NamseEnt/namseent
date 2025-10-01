use crate::*;
pub use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub type JoinHandle<T> = tokio::task::JoinHandle<T>;

pub trait SkSkia: SkCalculate {
    fn move_to_next_frame(&mut self);
    fn surface(&mut self) -> &mut dyn SkSurface;
    fn on_resize(&mut self, wh: Wh<IntPx>);
}

pub trait SkSurface {
    fn flush(&mut self);
    fn canvas(&mut self) -> &dyn SkCanvas;
}

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

pub trait SkImage {
    fn info(&self) -> ImageInfo;
}
