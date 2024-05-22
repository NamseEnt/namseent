use crate::*;
use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub trait SkSkia: SkCalculate {
    fn move_to_next_frame(&mut self);
    fn surface(&mut self) -> &mut dyn SkSurface;
    fn on_resize(&mut self, wh: Wh<IntPx>);
    // #[cfg(target_family = "wasm")]
    // async fn encode_loaded_image_to_png(&self, image: &Image) -> Vec<u8>;
    #[cfg(target_family = "wasm")]
    fn load_image_from_web_image_bitmap(&self, image_bitmap: web_sys::ImageBitmap) -> ImageLoaded;
    #[cfg(target_family = "wasm")]
    fn unload_image(&self, image_id: u32);
}

pub trait SkCalculate {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph>;
    fn font_metrics(&self, font: &Font) -> Option<FontMetrics>;
    fn load_typeface(
        &self,
        typeface_name: String,
        bytes: Vec<u8>,
    ) -> tokio::task::JoinHandle<Result<()>>;
    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool;
    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>>;
    #[cfg(not(target_family = "wasm"))]
    fn load_image_from_encoded(&self, bytes: &[u8]) -> tokio::task::JoinHandle<Image>;
    #[cfg(not(target_family = "wasm"))]
    fn load_image_from_raw(
        &self,
        image_info: ImageInfo,
        bytes: &[u8],
    ) -> tokio::task::JoinHandle<Image>;
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
