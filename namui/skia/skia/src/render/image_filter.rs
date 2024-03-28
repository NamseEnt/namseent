use crate::*;

#[type_derives()]
pub enum ImageFilter {
    Blur {
        sigma_xy: Xy<f32>,
        tile_mode: Option<TileMode>,
        input: Option<Box<ImageFilter>>,
        crop_rect: Option<Rect<Px>>,
    },
}

#[cfg(feature = "skia")]
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
        }
    }
}
