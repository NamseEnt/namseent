use crate::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum ImageFilter {
    Blur {
        sigma_xy: Xy<OrderedFloat<f32>>,
        tile_mode: Option<TileMode>,
        input: Option<Box<ImageFilter>>,
        /// crop_rect is not supported in wasm
        crop_rect: Option<Rect<Px>>,
    },
    Image {
        src: Image,
    },
    Blend {
        mode: BlendMode,
        background: Box<ImageFilter>,
        foreground: Box<ImageFilter>,
    },
    Offset {
        offset: Xy<Px>,
        input: Box<ImageFilter>,
    },
    ColorFilter {
        color_filter: ColorFilter,
        input: Box<ImageFilter>,
    },
    Empty,
}

impl ImageFilter {
    pub fn offset(self, offset: Xy<Px>) -> Self {
        ImageFilter::Offset {
            offset,
            input: Box::new(self),
        }
    }

    pub fn blend(mode: BlendMode, background: ImageFilter, foreground: ImageFilter) -> Self {
        ImageFilter::Blend {
            mode,
            background: Box::new(background),
            foreground: Box::new(foreground),
        }
    }

    pub fn color_filter(self, color_filter: ColorFilter) -> Self {
        ImageFilter::ColorFilter {
            color_filter,
            input: Box::new(self),
        }
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
            ImageFilter::Image { src } => skia_safe::image_filters::image(
                skia_safe::Image::clone(&src.skia_image),
                None,
                None,
                None,
            )
            .unwrap(),
            ImageFilter::Blend {
                mode,
                background,
                foreground,
            } => skia_safe::image_filters::blend(
                skia_safe::BlendMode::from(*mode),
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

impl Default for ImageFilter {
    fn default() -> Self {
        ImageFilter::Empty
    }
}
