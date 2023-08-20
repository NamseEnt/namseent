use namui_type::*;
use std::sync::Arc;
use web_sys::ImageBitmap;

pub trait SkSkia {
    fn surface(&self) -> &dyn SkSurface;
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph>;
    fn font_metrics(&self, font: &Font) -> Option<FontMetrics>;
    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]);
    fn load_image(&self, image_source: &ImageSource, image_bitmap: &web_sys::ImageBitmap);
    fn image(&self, image_source: &ImageSource) -> Option<Image>;
    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool;
    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>>;
    fn encode_loaded_image_to_png(&self, image: &Image) -> Vec<u8>;
}

pub trait SkSurface {
    fn flush(&self);
    fn canvas(&self) -> &dyn SkCanvas;
}

pub trait SkCanvas {
    fn clear(&self, color: Color);
    fn draw_text_blob(&self, glyph_ids: Vec<usize>, xy: Xy<Px>, font: &Font, paint: &Paint);
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
    fn get_matrix(&self) -> Matrix3x3;
    fn set_matrix(&self, matrix: Matrix3x3);
    fn transform(&self, matrix: Matrix3x3);
    fn rotate(&self, angle: Angle);
    fn scale(&self, sx: f32, sy: f32);
}

pub trait SkImage {
    fn info(&self) -> ImageInfo;
}

pub trait ImageLoader<Image> {
    fn get_or_start_load_image(
        &self,
        image_source: &ImageSource,
        on_loaded: Box<dyn FnOnce(ImageBitmap) -> Image>,
    ) -> Option<Image>;
}

/*

Main: 이미지의 크기를 알고 싶어.
이미지를 가리키는 오브젝트를 얻고 싶어.
픽셀 정보는 몰라도 돼.
메인에서 아는 이미지는 Drawer에 로딩되어있어야해.

Drawer: 이미지의 모든 것을 알아야 한다.

            메인                    Drawer
로딩         Drawer에 전달           풀로딩
로딩여부     알아야함                알수밖에
이미지 크기  알아야함                알수밖에


사용자는 이미지를 src로 쓰거나, 이미지 바이트를 직접 들고 있거나, 이미지를 로딩해달라고 요청해서 사이즈 정도 알고 잇는 정도.

*/
