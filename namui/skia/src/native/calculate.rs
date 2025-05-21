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

    fn load_typeface(
        &self,
        typeface_name: String,
        bytes: Vec<u8>,
    ) -> tokio::task::JoinHandle<Result<()>> {
        tokio::task::spawn_blocking(move || {
            NativeTypeface::load(&typeface_name, &bytes)?;
            Ok(())
        })
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        NativePath::get(path).contains(paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        NativePath::get(path).bounding_box(paint)
    }

    fn load_image_from_encoded(&self, bytes: &[u8]) -> tokio::task::JoinHandle<Image> {
        let data = skia_safe::Data::new_copy(bytes);

        tokio::task::spawn_blocking(move || {
            let image = skia_safe::Image::from_encoded(data).unwrap();
            Image::new(image.image_info().into(), image)
        })
    }
    fn load_image_from_raw(
        &self,
        image_info: ImageInfo,
        bytes: &[u8],
    ) -> tokio::task::JoinHandle<Image> {
        let data = skia_safe::Data::new_copy(bytes);

        tokio::task::spawn_blocking(move || {
            let row_bytes = image_info.width.as_f32() as usize * image_info.color_type.word();
            let image =
                skia_safe::images::raster_from_data(&image_info.into(), data, row_bytes).unwrap();
            Image::new(image_info, image)
        })
    }
}
