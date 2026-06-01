use namui::{BlendMode, Color, ColorFilter, ImageFilter, OrderedFloat, Ratio, Xy};

/// Create an antialiased dilated color mask from an image filter source.
///
/// This is used for image-based strokes and borders where we want a soft
/// dilation edge instead of a hard mask.
pub(crate) fn dilated_color_filter(
    source: ImageFilter,
    radius_xy: Xy<OrderedFloat>,
    color: Color,
) -> ImageFilter {
    let radius_x = radius_xy.x.as_f32();
    let radius_y = radius_xy.y.as_f32();

    if !radius_x.is_finite() || !radius_y.is_finite() || radius_x <= 0.0 || radius_y <= 0.0 {
        return source;
    }

    let dilated = source.dilate(radius_xy, None);
    let colored = dilated.color_filter(ColorFilter::Blend {
        color,
        blend_mode: BlendMode::SrcIn,
    });

    ImageFilter::Blur {
        sigma_xy: Xy::new(OrderedFloat::new(0.5), OrderedFloat::new(0.5)),
        tile_mode: None,
        input: Some(Box::new(colored)),
        crop_rect: None,
    }
}
