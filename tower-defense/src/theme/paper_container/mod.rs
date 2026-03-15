use crate::palette;
use namui::*;

mod smooth_path;
mod zigzag_path;

const SHADOW_OFFSET_Y: Px = px(2.0);
const SHADOW_ALPHA: u8 = 192;
const TORN_BORDER_OUTER_BRIGHTER_VALUE: f32 = 0.275;

use smooth_path::{dual_layer_torn_paper_paths, single_layer_reduced_paper_path};
use zigzag_path::{TearSide, torn_paper_path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub enum PaperTexture {
    Rough,
    Crumpled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub enum PaperVariant {
    Tape,
    Sticky,
    Paper,
    Card,
    PaperSingleLayer,
    Pill,
}

impl PaperTexture {
    fn image(self) -> Image {
        match self {
            PaperTexture::Rough => crate::asset::image::ui::paper::PAPER_00,
            PaperTexture::Crumpled => crate::asset::image::ui::paper::PAPER_01,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub struct PaperArrow {
    pub side: ArrowSide,
    pub width: Px,
    pub height: Px,
    pub offset: Px,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub enum ArrowSide {
    Left,
    Right,
    Top,
    Bottom,
}

pub struct PaperContainerBackground {
    pub width: Px,
    pub height: Px,
    pub texture: PaperTexture,
    pub variant: PaperVariant,
    pub color: Color,
    pub shadow: bool,
    pub arrow: Option<PaperArrow>,
}

impl Component for PaperContainerBackground {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            width,
            height,
            texture,
            variant,
            color,
            shadow,
            arrow,
        } = self;

        match variant {
            PaperVariant::Tape | PaperVariant::Sticky => {
                render_tape_or_sticky(ctx, width, height, variant, texture, color, shadow, arrow);
            }
            PaperVariant::Paper => {
                render_paper(ctx, width, height, texture, color, shadow, arrow);
            }
            PaperVariant::Card => {
                render_card(ctx, width, height, texture, color, shadow, arrow);
            }
            PaperVariant::Pill => {
                render_pill(ctx, width, height, texture, color, shadow, arrow);
            }
            PaperVariant::PaperSingleLayer => {
                render_single_layer_paper(ctx, width, height, texture, color, shadow, arrow);
            }
        }
    }
}

fn render_pill(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
    arrow: Option<PaperArrow>,
) {
    let tracked = ctx.track_eq(&(width, height, arrow));
    let radius = height / 2.0;
    let path = ctx
        .memo(|| {
            let r = Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: tracked.0,
                height: tracked.1,
            };
            let base = Path::new().add_rrect(r, radius, radius);
            if let Some(a) = arrow {
                with_arrow(base, width, height, Some(a))
            } else {
                base
            }
        })
        .as_ref()
        .clone();

    ctx.add(namui::path(path.clone(), textured_paint(texture, color)));
    if shadow {
        add_shadow(ctx, path);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_tape_or_sticky(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    variant: PaperVariant,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
    arrow: Option<PaperArrow>,
) {
    let tear_side = match variant {
        PaperVariant::Tape => TearSide::Torn,
        _ => TearSide::Subtle,
    };
    let tracked = ctx.track_eq(&(width, height, tear_side, arrow));
    let path = ctx
        .memo(|| torn_paper_path(tracked.0, tracked.1, tracked.2, tracked.3))
        .as_ref()
        .clone();

    ctx.add(namui::path(path.clone(), textured_paint(texture, color)));
    if shadow {
        add_shadow(ctx, path);
    }
}

fn render_paper(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
    arrow: Option<PaperArrow>,
) {
    let tracked = ctx.track_eq(&(width, height, arrow));
    let (inner_path, outer_path) = ctx
        .memo(|| {
            let (i, o) = dual_layer_torn_paper_paths(tracked.0, tracked.1);
            if let Some(a) = arrow {
                (
                    with_arrow(i, width, height, Some(a)),
                    with_arrow(o, width, height, Some(a)),
                )
            } else {
                (i, o)
            }
        })
        .as_ref()
        .clone();

    ctx.add(namui::path(
        inner_path.clone(),
        textured_paint(texture, color),
    ));
    ctx.add(namui::path(
        outer_path.clone(),
        textured_paint(texture, color.brighter(TORN_BORDER_OUTER_BRIGHTER_VALUE)),
    ));

    if shadow {
        add_shadow(ctx, outer_path);
    }
}

fn render_card(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
    arrow: Option<PaperArrow>,
) {
    let tracked = ctx.track_eq(&(width, height, arrow));
    let path = ctx
        .memo(|| {
            let r = Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: tracked.0,
                height: tracked.1,
            };
            let base = Path::new().add_rrect(r, palette::ROUND, palette::ROUND);
            if let Some(a) = arrow {
                with_arrow(base, width, height, Some(a))
            } else {
                base
            }
        })
        .as_ref()
        .clone();

    ctx.add(namui::path(path.clone(), textured_paint(texture, color)));

    if shadow {
        add_shadow(ctx, path);
    }
}

fn render_single_layer_paper(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
    arrow: Option<PaperArrow>,
) {
    let tracked = ctx.track_eq(&(width, height, arrow));
    let path = ctx
        .memo(|| {
            let base = single_layer_reduced_paper_path(tracked.0, tracked.1);
            if let Some(a) = arrow {
                with_arrow(base, width, height, Some(a))
            } else {
                base
            }
        })
        .as_ref()
        .clone();

    ctx.add(namui::path(path.clone(), textured_paint(texture, color)));

    if shadow {
        add_shadow(ctx, path);
    }
}

fn textured_paint(texture: PaperTexture, color: Color) -> Paint {
    Paint::new(Color::WHITE)
        .set_style(PaintStyle::Fill)
        .set_shader(Shader::Image {
            src: texture.image(),
            tile_mode: Xy::single(TileMode::Repeat),
        })
        .set_color_filter(ColorFilter::Blend {
            color,
            blend_mode: BlendMode::Modulate,
        })
}

fn add_shadow(ctx: &RenderCtx, path: Path) {
    let shadow_path = path.translate(px(0.0), SHADOW_OFFSET_Y);
    let shadow_paint = Paint::new(Color::BLACK.with_alpha(SHADOW_ALPHA))
        .set_style(PaintStyle::Fill)
        .set_mask_filter(MaskFilter::Blur {
            blur_style: BlurStyle::Normal,
            sigma: 2.5,
        });
    ctx.add(namui::path(shadow_path, shadow_paint));
}

fn with_arrow(path: Path, width: Px, height: Px, arrow: Option<PaperArrow>) -> Path {
    let Some(arrow) = arrow else {
        return path;
    };

    match arrow.side {
        ArrowSide::Right => {
            let half = arrow.height / 2.0;
            path.move_to(width, arrow.offset - half)
                .line_to(width + arrow.width, arrow.offset)
                .line_to(width, arrow.offset + half)
                .close()
        }
        ArrowSide::Left => {
            let half = arrow.height / 2.0;
            path.move_to(0.px(), arrow.offset - half)
                .line_to(-arrow.width, arrow.offset)
                .line_to(0.px(), arrow.offset + half)
                .close()
        }
        ArrowSide::Top => {
            let half = arrow.width / 2.0;
            path.move_to(arrow.offset - half, 0.px())
                .line_to(arrow.offset, -arrow.height)
                .line_to(arrow.offset + half, 0.px())
                .close()
        }
        ArrowSide::Bottom => {
            let half = arrow.width / 2.0;
            path.move_to(arrow.offset - half, height)
                .line_to(arrow.offset, height + arrow.height)
                .line_to(arrow.offset + half, height)
                .close()
        }
    }
}
