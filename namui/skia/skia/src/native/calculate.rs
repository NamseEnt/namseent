use crate::*;
use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct NativeCalculate;

impl NativeCalculate {
    pub(crate) fn new() -> Self {
        NativeCalculate
    }
}

impl SkCalculate for NativeCalculate {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        NativeGroupGlyph::get(font, paint)
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        NativeFont::get(font).map(|x| x.metrics)
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) -> Result<()> {
        NativeTypeface::load(typeface_name, bytes)
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        NativePath::get(path).contains(paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        NativePath::get(path).bounding_box(paint)
    }
    fn image(&self, image_source: &ImageSource) -> Option<Image> {
        NativeImage::get(image_source).map(|x| x.image())
    }

    fn load_image(&self, image_source: &ImageSource, encoded_image: &[u8]) -> ImageInfo {
        NativeImage::load(image_source, encoded_image)
    }

    /// TODO: Make this from mut_ref to ref, using second context.
    fn load_image_from_raw(&self, image_info: ImageInfo, bitmap: &[u8]) -> ImageHandle {
        let row_bytes = image_info.width.as_f32() as usize * image_info.color_type.word();
        let image = skia_safe::images::raster_from_data(
            &image_info.into(),
            skia_safe::Data::new_copy(bitmap),
            row_bytes,
        )
        .unwrap();

        ImageHandle::new(image_info, image)
    }
}
