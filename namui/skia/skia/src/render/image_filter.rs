use crate::*;

#[type_derives()]
pub enum ImageFilter {
    Blur {
        sigma_xy: Xy<f32>,
        tile_mode: Option<TileMode>,
        input: Option<Box<ImageFilter>>,
        /// crop_rect is not supported in wasm
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

#[cfg(target_family = "wasm")]
impl From<&ImageFilter> for canvas_kit_wasm_bindgen::CanvasKitImageFilter {
    fn from(image_filter: &ImageFilter) -> Self {
        match image_filter {
            &ImageFilter::Blur {
                sigma_xy,
                tile_mode,
                ref input,
                crop_rect: _,
            } => canvas_kit_wasm_bindgen::canvas_kit()
                .ImageFilter()
                .MakeBlur(
                    sigma_xy.x,
                    sigma_xy.y,
                    tile_mode
                        .map(|tile_mode| tile_mode.into())
                        .unwrap_or(canvas_kit_wasm_bindgen::tile_mode_clamp()),
                    input.as_ref().map(|input| input.as_ref().into()),
                ),
        }
    }
}
