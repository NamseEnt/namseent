use crate::*;
use namui_type::*;
use std::sync::Arc;

pub(crate) struct NativeColorFilter {
    pub(crate) skia_color_filter: skia_safe::ColorFilter,
}

impl NativeColorFilter {
    pub(crate) fn get(color_filter: ColorFilter) -> Arc<NativeColorFilter> {
        static CACHE: LruCache<ColorFilter, NativeColorFilter> = LruCache::new();

        CACHE.get_or_create(&color_filter, |color_filter| color_filter.into())
    }

    pub(crate) fn skia(&self) -> &skia_safe::ColorFilter {
        &self.skia_color_filter
    }
}

impl From<&ImageFilter> for skia_safe::ImageFilter {
    fn from(image_filter: &ImageFilter) -> Self {
        match image_filter {
            &ImageFilter::Blur {
                sigma_xy,
                tile_mode,
                ref input,
                crop_rect,
            } => skia_safe::image_filters::blur(
                sigma_xy.into(),
                tile_mode.map(|tile_mode| tile_mode.into()),
                input.as_ref().map(|input| input.as_ref().into()),
                crop_rect.map(|x| skia_safe::Rect::from(x).into()),
            )
            .unwrap(),
            ImageFilter::Image { src } => {
                todo!();
                //     skia_safe::image_filters::image(
                //     skia_safe::Image::clone(&src.skia_image),
                //     None,
                //     None,
                //     None,
                // )
                // .unwrap()
            }
            ImageFilter::Blend {
                blender,
                background,
                foreground,
            } => skia_safe::image_filters::blend(
                skia_safe::Blender::from(blender),
                skia_safe::ImageFilter::from(background.as_ref()),
                skia_safe::ImageFilter::from(foreground.as_ref()),
                None,
            )
            .unwrap(),
            ImageFilter::Offset { offset, input } => skia_safe::image_filters::offset(
                (offset.x.as_f32(), offset.y.as_f32()),
                skia_safe::ImageFilter::from(input.as_ref()),
                None,
            )
            .unwrap(),
            ImageFilter::ColorFilter {
                color_filter,
                input,
            } => skia_safe::image_filters::color_filter(
                NativeColorFilter::from(color_filter).skia(),
                skia_safe::ImageFilter::from(input.as_ref()),
                None,
            )
            .unwrap(),
            ImageFilter::Empty => skia_safe::image_filters::empty(),
        }
    }
}

impl From<&ColorFilter> for NativeColorFilter {
    fn from(value: &ColorFilter) -> Self {
        match *value {
            ColorFilter::Blend { color, blend_mode } => NativeColorFilter {
                skia_color_filter: skia_safe::color_filters::blend(color, blend_mode.into())
                    .unwrap(),
            },
            ColorFilter::ScaleMatrix { r, g, b, a } => {
                let mut color_matrix = skia_safe::ColorMatrix::default();
                color_matrix.set_scale(r.into(), b.into(), g.into(), Some(a.into()));
                let skia_color_filter = skia_safe::color_filters::matrix(&color_matrix, None);
                NativeColorFilter { skia_color_filter }
            }
        }
    }
}
