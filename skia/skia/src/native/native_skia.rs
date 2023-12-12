use super::*;
use crate::*;

pub(crate) struct NativeSkia {
    context: skia_safe::gpu::DirectContext,
    surfaces: Vec<skia_safe::surface::Surface>,
}
unsafe impl Send for NativeSkia {}
unsafe impl Sync for NativeSkia {}

impl NativeSkia {
    pub(crate) fn new(
        context: skia_safe::gpu::DirectContext,
        surfaces: Vec<skia_safe::surface::Surface>,
    ) -> NativeSkia {
        Self { context, surfaces }
    }
}

impl SkSkia for NativeSkia {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        todo!()
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        todo!()
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
        todo!()
    }

    fn image(&self, image_source: &ImageSource) -> Option<Image> {
        todo!()
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        todo!()
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        todo!()
    }

    fn encode_loaded_image_to_png(
        &self,
        image: &Image,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<u8>>>> {
        todo!()
    }
    // fn group_glyph(&self, font: &Font, paint: &Paint) -> std::sync::Arc<dyn GroupGlyph> {
    //     CkGroupGlyph::get(font, paint)
    // }

    // fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
    //     CkFont::get(font).map(|x| x.metrics)
    // }

    // fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) {
    //     CkTypeface::load(typeface_name, bytes)
    // }

    // fn image(&self, image_source: &ImageSource) -> Option<Image> {
    //     CkImage::get(image_source).map(|x| x.image())
    // }

    // fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
    //     CkPath::get(path).contains(paint, xy)
    // }

    // fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
    //     CkPath::get(path).bounding_box(paint)
    // }

    // fn encode_loaded_image_to_png(
    //     &self,
    //     image: &Image,
    // ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<u8>>>> {
    //     let ck_image = CkImage::get(&image.src).unwrap();
    //     Box::pin(async move { ck_image.encode_to_png().await })
    // }
}
