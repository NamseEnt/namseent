use crate::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Default)]
pub enum ImageFilter {
    #[default]
    Empty,
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
        blender: Blender,
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
}

impl ImageFilter {
    pub fn offset(self, offset: Xy<Px>) -> Self {
        ImageFilter::Offset {
            offset,
            input: Box::new(self),
        }
    }

    pub fn blend(
        blender: impl Into<Blender>,
        background: ImageFilter,
        foreground: ImageFilter,
    ) -> Self {
        ImageFilter::Blend {
            blender: blender.into(),
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
