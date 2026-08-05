use crate::{
    card::{Card, Suit},
    image_filter_utils::dilated_color_filter,
};
use namui::*;

pub const STICKER_THUMBNAIL_STROKE: Px = px(6.0);
const STICKER_SHADOW_ALPHA: u8 = 192;
const STICKER_SHADOW_BLUR: Px = px(2.5);
const STICKER_SHADOW_OFFSET_Y: Px = px(2.0);

#[derive(Clone, Copy)]
pub struct ThumbnailRenderOptions {
    pub stroke_px: Px,
    pub shadow: bool,
}

impl ThumbnailRenderOptions {
    pub const fn new(stroke_px: Px, shadow: bool) -> Self {
        Self { stroke_px, shadow }
    }
}

pub enum ThumbnailContent<'a> {
    Sticker(Image),
    Card(&'a Card),
}

pub fn render_thumbnail(
    content: ThumbnailContent<'_>,
    width_height: Wh<Px>,
    options: ThumbnailRenderOptions,
) -> RenderingTree {
    match content {
        ThumbnailContent::Sticker(image) => {
            render_sticker_thumbnail_impl(image, width_height, options)
        }
        ThumbnailContent::Card(card) => render_card_thumbnail_impl(card, width_height, options),
    }
}

pub fn with_opacity(rendering_tree: RenderingTree, opacity: f32) -> RenderingTree {
    let opacity = opacity.clamp(0.0, 1.0);
    match rendering_tree {
        RenderingTree::Empty => RenderingTree::Empty,
        RenderingTree::Children(children) => namui::render(
            children
                .iter()
                .copied()
                .map(|child| with_opacity(child, opacity)),
        ),
        RenderingTree::Node(DrawCommand::Path { command }) => {
            RenderingTree::Node(DrawCommand::Path {
                command: arena_alloc(PathDrawCommand {
                    path: command.path.clone(),
                    paint: paint_with_opacity(command.paint.clone(), opacity),
                }),
            })
        }
        RenderingTree::Node(DrawCommand::Image { command }) => {
            RenderingTree::Node(DrawCommand::Image {
                command: arena_alloc(ImageDrawCommand {
                    image: command.image,
                    sprites: command.sprites.clone(),
                    paint: command
                        .paint
                        .clone()
                        .map(|paint| paint_with_opacity(paint, opacity)),
                    sprite_colors_blend_mode: command.sprite_colors_blend_mode,
                }),
            })
        }
        RenderingTree::Node(DrawCommand::Text { command }) => {
            RenderingTree::Node(DrawCommand::Text {
                command: arena_alloc(TextDrawCommand {
                    text: command.text.clone(),
                    font: command.font.clone(),
                    x: command.x,
                    y: command.y,
                    paint: paint_with_opacity(command.paint.clone(), opacity),
                    align: command.align,
                    baseline: command.baseline,
                    max_width: command.max_width,
                    line_height_percent: command.line_height_percent,
                    underline: command
                        .underline
                        .clone()
                        .map(|paint| Box::new(paint_with_opacity(*paint, opacity))),
                }),
            })
        }
        RenderingTree::Special(_) => rendering_tree,
    }
}

fn paint_with_opacity(mut paint: Paint, opacity: f32) -> Paint {
    paint.color = paint
        .color
        .with_alpha((f32::from(paint.color.a) * opacity).round() as u8);
    paint
}

pub fn render_sticker_image_with_shadow(
    image: Image,
    width_height: Wh<Px>,
    stroke_px: Px,
    shadow: bool,
) -> RenderingTree {
    render_thumbnail(
        ThumbnailContent::Sticker(image),
        width_height,
        ThumbnailRenderOptions::new(stroke_px, shadow),
    )
}

fn render_sticker_thumbnail_impl(
    image: Image,
    width_height: Wh<Px>,
    options: ThumbnailRenderOptions,
) -> RenderingTree {
    let image_tree = render_sticker_image_tree(image, width_height, options.stroke_px);
    if !options.shadow {
        return image_tree;
    }

    let shadow_tree = render_sticker_shadow(image, width_height, options.stroke_px);
    namui::render(vec![image_tree, shadow_tree])
}

const OVERLAY_OVERLAP: Px = px(8.0);
pub fn render_right_bottom_overlay(
    width_height: Wh<Px>,
    text: &str,
    text_color: Color,
) -> RenderingTree {
    let overlay = crate::theme::typography::TypographyBuilder::new()
        .headline()
        .size(crate::theme::typography::FontSize::Medium)
        .color(text_color)
        .stroke(2.px(), crate::theme::palette::DARK_CHARCOAL)
        .static_text(text)
        .render_right_bottom(width_height);

    namui::translate(
        overlay.offset.x + OVERLAY_OVERLAP,
        overlay.offset.y + OVERLAY_OVERLAP,
        overlay.tree,
    )
}

pub fn render_right_top_overlay(width: Px, text: &str, text_color: Color) -> RenderingTree {
    let overlay = crate::theme::typography::TypographyBuilder::new()
        .headline()
        .size(crate::theme::typography::FontSize::Medium)
        .color(text_color)
        .stroke(2.px(), crate::theme::palette::DARK_CHARCOAL)
        .static_text(text)
        .render_right_top(width);

    namui::translate(
        overlay.offset.x + OVERLAY_OVERLAP,
        overlay.offset.y - OVERLAY_OVERLAP,
        overlay.tree,
    )
}

const CARD_ASPECT_RATIO: f32 = 0.72;
const CARD_ROUND_RATIO: f32 = 0.12;
const CARD_SUIT_SIZE_RATIO: f32 = 0.5;
const CARD_SUIT_CENTER_RATIO: (f32, f32) = (0.4, 0.31);
const CARD_RANK_SIZE_RATIO: f32 = 0.52;
const CARD_RANK_CENTER_RATIO: (f32, f32) = (0.6, 0.67);

pub fn render_card_thumbnail(
    card: &Card,
    width_height: Wh<Px>,
    stroke_px: Px,
    shadow: bool,
) -> RenderingTree {
    render_thumbnail(
        ThumbnailContent::Card(card),
        width_height,
        ThumbnailRenderOptions::new(stroke_px, shadow),
    )
}

fn render_card_thumbnail_impl(
    card: &Card,
    width_height: Wh<Px>,
    options: ThumbnailRenderOptions,
) -> RenderingTree {
    let margin = options.stroke_px;
    let avail_width = width_height.width - margin * 2.0;
    let avail_height = width_height.height - margin * 2.0;

    let mut card_wh = Wh::new(avail_height * CARD_ASPECT_RATIO, avail_height);
    if card_wh.width > avail_width {
        card_wh = Wh::new(avail_width, avail_width / CARD_ASPECT_RATIO);
    }
    let card_xy = Xy::new(
        (width_height.width - card_wh.width) * 0.5,
        (width_height.height - card_wh.height) * 0.5,
    );
    let card_rect = Rect::from_xy_wh(card_xy, card_wh);
    let round = card_wh.width * CARD_ROUND_RATIO;

    let tint = card_suit_tint(card.suit);

    let suit_tree = render_tinted_image(
        card.suit.hand_drawn_image(),
        card_xy
            + Xy::new(
                card_wh.width * CARD_SUIT_CENTER_RATIO.0,
                card_wh.height * CARD_SUIT_CENTER_RATIO.1,
            ),
        card_wh * CARD_SUIT_SIZE_RATIO,
        tint,
    );

    let rank_tree = render_tinted_image(
        card.rank.hand_drawn_image(),
        card_xy
            + Xy::new(
                card_wh.width * CARD_RANK_CENTER_RATIO.0,
                card_wh.height * CARD_RANK_CENTER_RATIO.1,
            ),
        card_wh * CARD_RANK_SIZE_RATIO,
        tint,
    );

    let mut trees = vec![rank_tree, suit_tree];
    trees.extend(render_card_base(
        card_rect,
        round,
        options.stroke_px,
        options.shadow,
    ));
    namui::render(trees)
}

fn render_card_base(
    card_rect: Rect<Px>,
    round: Px,
    stroke_px: Px,
    shadow: bool,
) -> Vec<RenderingTree> {
    let path = Path::new().add_rrect(card_rect, round, round);

    let mut trees = vec![
        namui::path(
            path.clone(),
            Paint::new(crate::theme::palette::ON_SURFACE)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(stroke_px)
                .set_stroke_position(StrokePosition::Inside)
                .set_stroke_join(StrokeJoin::Round),
        ),
        namui::path(
            path.clone(),
            Paint::new(Color::WHITE).set_style(PaintStyle::Fill),
        ),
        namui::path(
            path.clone(),
            Paint::new(Color::WHITE)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(stroke_px)
                .set_stroke_position(StrokePosition::Outside)
                .set_stroke_join(StrokeJoin::Round),
        ),
    ];

    if shadow {
        let shadow_path = path.translate(0.px(), STICKER_SHADOW_OFFSET_Y);
        trees.push(namui::path(
            shadow_path,
            Paint::new(Color::BLACK.with_alpha(STICKER_SHADOW_ALPHA))
                .set_style(PaintStyle::Fill)
                .set_mask_filter(MaskFilter::Blur {
                    blur_style: BlurStyle::Normal,
                    sigma: STICKER_SHADOW_BLUR.as_f32(),
                }),
        ));
    }

    trees
}

fn card_suit_tint(suit: Suit) -> Color {
    match suit {
        Suit::Hearts | Suit::Diamonds => crate::theme::palette::RED,
        Suit::Spades | Suit::Clubs => crate::theme::palette::ON_SURFACE,
    }
}

fn render_tinted_image(image: Image, center: Xy<Px>, wh: Wh<Px>, color: Color) -> RenderingTree {
    let paint = Paint::new(Color::WHITE).set_color_filter(ColorFilter::Blend {
        color,
        blend_mode: BlendMode::SrcIn,
    });

    namui::image(ImageParam {
        rect: Rect::from_xy_wh(center - Xy::new(wh.width * 0.5, wh.height * 0.5), wh),
        image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: Some(paint),
        },
    })
}

fn render_sticker_image_tree(image: Image, width_height: Wh<Px>, stroke_px: Px) -> RenderingTree {
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

fn render_sticker_shadow(image: Image, width_height: Wh<Px>, stroke_px: Px) -> RenderingTree {
    let shadow_color = Color::BLACK.with_alpha(STICKER_SHADOW_ALPHA);

    let shadow_filter = ImageFilter::Blur {
        sigma_xy: Xy::new(
            OrderedFloat::new(STICKER_SHADOW_BLUR.as_f32()),
            OrderedFloat::new(STICKER_SHADOW_BLUR.as_f32()),
        ),
        tile_mode: None,
        input: Some(Box::new(
            sticker_image_filter(image, width_height, stroke_px).color_filter(ColorFilter::Blend {
                color: shadow_color,
                blend_mode: BlendMode::SrcIn,
            }),
        )),
        crop_rect: None,
    }
    .offset(Xy::new(0.px(), STICKER_SHADOW_OFFSET_Y));

    let paint = Paint::new(Color::WHITE).set_image_filter(shadow_filter);

    namui::image(ImageParam {
        rect: width_height.to_rect(),
        image,
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: Some(paint),
        },
    })
}

fn sticker_destination_rect(image_wh: Wh<Px>, width_height: Wh<Px>) -> Option<Rect<Px>> {
    let image_width = image_wh.width.as_f32();
    let image_height = image_wh.height.as_f32();
    let target_width = width_height.width.as_f32();
    let target_height = width_height.height.as_f32();

    if !image_width.is_finite()
        || !image_height.is_finite()
        || !target_width.is_finite()
        || !target_height.is_finite()
        || image_width <= 0.0
        || image_height <= 0.0
        || target_width <= 0.0
        || target_height <= 0.0
    {
        return None;
    }

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

    Some(dest_rect)
}

fn sticker_outline_radii(stroke_px: Px) -> (Px, Px) {
    (stroke_px, stroke_px * 0.4)
}

fn sticker_image_filter(image: Image, width_height: Wh<Px>, stroke_px: Px) -> ImageFilter {
    let source = ImageFilter::Image { src: image };

    let image_wh = image.info().wh();
    let Some(dest_rect) = sticker_destination_rect(image_wh, width_height) else {
        return source;
    };

    let scale_x = dest_rect.width().as_f32() / image_wh.width.as_f32();
    let scale_y = dest_rect.height().as_f32() / image_wh.height.as_f32();

    if !scale_x.is_finite() || !scale_y.is_finite() || scale_x <= 0.0 || scale_y <= 0.0 {
        return source;
    }

    let source = source.with_local_matrix(
        TransformMatrix::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
            * TransformMatrix::from_scale(scale_x, scale_y),
    );

    let (total_stroke_px, inner_stroke_px) = sticker_outline_radii(stroke_px);
    let total_radius = Xy::new(
        OrderedFloat::new(total_stroke_px.as_f32()),
        OrderedFloat::new(total_stroke_px.as_f32()),
    );
    let inner_radius = Xy::new(
        OrderedFloat::new(inner_stroke_px.as_f32()),
        OrderedFloat::new(inner_stroke_px.as_f32()),
    );

    let dilated_inner = dilated_color_filter(source.clone(), inner_radius, Color::BLACK);
    let dilated_total = dilated_color_filter(source.clone(), total_radius, Color::WHITE);

    let black_ring = ImageFilter::blend(BlendMode::DstOut, dilated_inner.clone(), source.clone());

    let white_ring = ImageFilter::blend(
        BlendMode::DstOut,
        dilated_total,
        dilated_color_filter(source.clone(), inner_radius, Color::WHITE),
    );

    let black_and_source = ImageFilter::blend(BlendMode::SrcOver, black_ring, source.clone());
    ImageFilter::blend(BlendMode::SrcOver, white_ring, black_and_source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sticker_fit_is_independent_of_source_resolution() {
        let target = Wh::single(px(96.0));
        let small = sticker_destination_rect(Wh::single(px(64.0)), target).unwrap();
        let large = sticker_destination_rect(Wh::single(px(1024.0)), target).unwrap();

        assert_eq!(small.width(), target.width);
        assert_eq!(small.height(), target.height);
        assert_eq!(small, large);
    }

    #[test]
    fn sticker_fit_contains_non_square_source() {
        let target = Wh::single(px(128.0));
        let dest = sticker_destination_rect(Wh::new(px(512.0), px(256.0)), target).unwrap();

        assert_eq!(dest.width().as_f32(), 128.0);
        assert_eq!(dest.height().as_f32(), 64.0);
        assert_eq!(dest.y().as_f32(), 32.0);
    }

    #[test]
    fn sticker_stroke_radii_are_in_target_pixels() {
        let (total, inner) = sticker_outline_radii(px(6.0));

        assert_eq!(total.as_f32(), 6.0);
        assert_eq!(inner.as_f32(), 2.4);
    }

    #[test]
    fn sticker_fit_rejects_invalid_dimensions() {
        assert!(
            sticker_destination_rect(Wh::new(px(0.0), px(256.0)), Wh::single(px(96.0))).is_none()
        );
        assert!(
            sticker_destination_rect(Wh::single(px(256.0)), Wh::new(px(96.0), px(0.0))).is_none()
        );
    }
}
