use namui::*;

mod smooth_path;
mod zigzag_path;

const SHADOW_OFFSET_Y: Px = px(2.0);
const SHADOW_ALPHA: u8 = 192;
const TORN_BORDER_OUTER_BRIGHTER_VALUE: f32 = 0.275;

use smooth_path::dual_layer_torn_paper_paths;
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
}

impl PaperTexture {
    fn image(self) -> Image {
        match self {
            PaperTexture::Rough => crate::asset::image::ui::paper::PAPER_00,
            PaperTexture::Crumpled => crate::asset::image::ui::paper::PAPER_01,
        }
    }
}

pub struct PaperContainerBackground {
    pub width: Px,
    pub height: Px,
    pub texture: PaperTexture,
    pub variant: PaperVariant,
    pub color: Color,
    pub shadow: bool,
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
        } = self;

        match variant {
            PaperVariant::Tape | PaperVariant::Sticky => {
                render_tape_or_sticky(ctx, width, height, variant, texture, color, shadow);
            }
            PaperVariant::Paper => {
                render_paper(ctx, width, height, texture, color, shadow);
            }
        }
    }
}

fn render_tape_or_sticky(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    variant: PaperVariant,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
) {
    let tear_side = match variant {
        PaperVariant::Tape => TearSide::Torn,
        _ => TearSide::Subtle,
    };
    let tracked = ctx.track_eq(&(width, height, tear_side));
    let path = ctx.memo(|| torn_paper_path(tracked.0, tracked.1, tracked.2));

    ctx.add(namui::path(
        path.as_ref().clone(),
        textured_paint(texture, color),
    ));

    if shadow {
        add_shadow(ctx, path.as_ref().clone());
    }
}

fn render_paper(
    ctx: &RenderCtx,
    width: Px,
    height: Px,
    texture: PaperTexture,
    color: Color,
    shadow: bool,
) {
    let tracked = ctx.track_eq(&(width, height));
    let paths = ctx.memo(|| dual_layer_torn_paper_paths(tracked.0, tracked.1));
    let (inner_path, outer_path) = paths.as_ref();

    ctx.add(namui::path(
        inner_path.clone(),
        textured_paint(texture, color),
    ));
    ctx.add(namui::path(
        outer_path.clone(),
        textured_paint(texture, color.brighter(TORN_BORDER_OUTER_BRIGHTER_VALUE)),
    ));

    if shadow {
        add_shadow(ctx, outer_path.clone());
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
