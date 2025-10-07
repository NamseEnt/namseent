mod blender;
mod color_filter;
mod font;
mod group_glyph;
mod paint;
mod path;
mod shader;
mod text_blob;
mod typeface;

use crate::*;
use color_filter::*;
use font::*;
use group_glyph::*;
use paint::*;
use path::*;
use shader::*;
use std::sync::Arc;
use typeface::*;

pub struct SkCalculate {}

impl SkCalculate {
    pub fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        NativeGroupGlyph::get(font, paint)
    }

    pub fn font_metrics(font: &Font) -> Option<FontMetrics> {
        NativeFont::get(font).map(|x| x.metrics)
    }

    pub fn load_typeface(
        typeface_name: String,
        bytes: Vec<u8>,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>> {
        tokio::task::spawn_blocking(move || {
            NativeTypeface::load(&typeface_name, &bytes)?;
            Ok(())
        })
    }

    pub fn path_contains_xy(path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        NativePath::get(path).contains(paint, xy)
    }

    pub fn path_bounding_box(path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        NativePath::get(path).bounding_box(paint)
    }

    pub fn load_image_from_encoded(bytes: &[u8]) -> tokio::task::JoinHandle<Image> {
        let data = skia_safe::Data::new_copy(bytes);

        tokio::task::spawn_blocking(move || {
            let image = skia_safe::Image::from_encoded(data).unwrap();
            Image::new(image.image_info().into(), image)
        })
    }
    pub fn load_image_from_raw(
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
