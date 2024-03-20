use namui_type::*;
use std::sync::Arc;

#[cfg(feature = "wasm")]
use web_sys::ImageBitmap;

pub trait SkSkia: SkCalculate {
    fn move_to_next_frame(&mut self);
    fn surface(&mut self) -> &mut dyn SkSurface;
    fn on_resize(&mut self, wh: Wh<IntPx>);
}

pub trait SkCalculate {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph>;
    fn font_metrics(&self, font: &Font) -> Option<FontMetrics>;
    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]);
    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool;
    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>>;
    fn image(&self, image_source: &ImageSource) -> Option<Image>;
    #[cfg(feature = "wasm")]
    async fn encode_loaded_image_to_png(&self, image: &Image) -> Vec<u8>;
    #[cfg(feature = "wasm")]
    fn load_image(&self, image_source: ImageSource, image_bitmap: web_sys::ImageBitmap);
    #[cfg(not(feature = "wasm"))]
    fn load_image(&self, image_source: &ImageSource, encoded_image: &[u8]) -> ImageInfo;
    #[cfg(not(feature = "wasm"))]
    fn load_image_from_raw(&self, image_info: ImageInfo, bitmap: &[u8]) -> ImageHandle;
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
        image_source: &ImageSource,
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

pub trait ImageLoader<Image> {
    #[cfg(feature = "wasm")]
    fn get_or_start_load_image(
        &self,
        image_source: &ImageSource,
        on_loaded: Box<dyn FnOnce(ImageBitmap) -> Image>,
    ) -> Option<Image>;
}
