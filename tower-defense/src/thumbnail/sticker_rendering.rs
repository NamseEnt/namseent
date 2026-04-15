use crate::{card::Card, icon::IconKind};
use namui::*;

pub const STICKER_THUMBNAIL_STROKE: Px = px(8.0);

pub fn render_sticker_image(image: Image, width_height: Wh<Px>, stroke_px: Px) -> RenderingTree {
    let paint = Paint::new(Color::WHITE).set_image_filter(sticker_image_filter(
        image,
        width_height,
        stroke_px,
    ));

    namui::image(ImageParam {
        rect: width_height.to_rect(),
        image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: Some(paint),
        },
    })
}

pub fn render_placeholder_thumbnail(width_height: Wh<Px>, stroke_px: Px) -> RenderingTree {
    render_sticker_image(
        crate::asset::image::ui::PLACEHOLDER,
        width_height,
        stroke_px,
    )
}

pub fn render_card_thumbnail(card: &Card, width_height: Wh<Px>, stroke_px: Px) -> RenderingTree {
    let rank_overlay = crate::thumbnail::overlay_rendering::render_text_overlay(
        width_height,
        &card.rank.to_string(),
        crate::thumbnail::overlay_rendering::OverlayPosition::TopLeft,
        0.25,
        0.75,
    );

    let suit_overlay = crate::thumbnail::overlay_rendering::render_icon_overlay(
        width_height,
        IconKind::Suit { suit: card.suit },
        crate::thumbnail::overlay_rendering::OverlayPosition::TopRight,
        0.2,
    );

    namui::render(vec![
        rank_overlay,
        suit_overlay,
        render_sticker_image(IconKind::Card.image(), width_height, stroke_px),
    ])
}

fn sticker_image_filter(image: Image, width_height: Wh<Px>, stroke_px: Px) -> ImageFilter {
    let source = ImageFilter::Image { src: image };

    let image_wh = image.info().wh();
    let image_width = image_wh.width.as_f32();
    let image_height = image_wh.height.as_f32();
    let target_width = width_height.width.as_f32();
    let target_height = width_height.height.as_f32();
    let target_ratio = target_width / target_height;
    let image_ratio = image_width / image_height;

    let dest_rect = if image_ratio == target_ratio {
        Rect::from_xy_wh(Xy::zero(), width_height)
    } else if image_ratio > target_ratio {
        let scale = target_width / image_width;
        let height = px(image_height * scale);
        let y = (width_height.height - height) / 2.0;
        Rect::from_xy_wh(Xy::new(0.px(), y), Wh::new(width_height.width, height))
    } else {
        let scale = target_height / image_height;
        let width = px(image_width * scale);
        let x = (width_height.width - width) / 2.0;
        Rect::from_xy_wh(Xy::new(x, 0.px()), Wh::new(width, width_height.height))
    };

    let scale_x = dest_rect.width().as_f32() / image_width;
    let scale_y = dest_rect.height().as_f32() / image_height;
    let total_radius = Xy::new(
        OrderedFloat::new(stroke_px.as_f32() / scale_x),
        OrderedFloat::new(stroke_px.as_f32() / scale_y),
    );
    let inner_radius = Xy::new(
        OrderedFloat::new((stroke_px * 0.4).as_f32() / scale_x),
        OrderedFloat::new((stroke_px * 0.4).as_f32() / scale_y),
    );

    let dilated_total = source.clone().dilate(total_radius, None);
    let dilated_inner = source.clone().dilate(inner_radius, None);

    let black_ring = ImageFilter::blend(
        BlendMode::DstOut,
        dilated_inner.clone().color_filter(ColorFilter::Blend {
            color: Color::BLACK,
            blend_mode: BlendMode::SrcIn,
        }),
        source.clone(),
    );

    let white_ring = ImageFilter::blend(
        BlendMode::DstOut,
        dilated_total.color_filter(ColorFilter::Blend {
            color: Color::WHITE,
            blend_mode: BlendMode::SrcIn,
        }),
        dilated_inner.clone().color_filter(ColorFilter::Blend {
            color: Color::WHITE,
            blend_mode: BlendMode::SrcIn,
        }),
    );

    let black_and_source = ImageFilter::blend(BlendMode::SrcOver, black_ring, source.clone());
    let filter = ImageFilter::blend(BlendMode::SrcOver, white_ring, black_and_source);

    filter.with_local_matrix(
        TransformMatrix::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
            * TransformMatrix::from_scale(scale_x, scale_y),
    )
}
