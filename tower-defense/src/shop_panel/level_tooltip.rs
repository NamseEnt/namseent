use crate::game_state::level_rarity_weights;
use crate::icon::{Icon, IconKind, IconSize};
use crate::rarity::Rarity;
use crate::theme::palette;
use crate::theme::paper_container::{
    PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
};
use crate::theme::typography::{FontSize, TypographyBuilder, memoized_text};
use namui::{ComposeCtx, *};
use namui_prebuilt::table;
use std::num::NonZeroUsize;

pub const PADDING: Px = px(8.0);
pub const MAX_WIDTH: Px = px(260.0);
pub const HEIGHT: Px = px(168.0);
pub const TOOLTIP_WH: Wh<Px> = Wh::new(MAX_WIDTH, HEIGHT);
const ARROW_WIDTH: Px = px(48.0);

pub const BACKGROUND_ARROW_WIDTH: Px = px(8.0);
pub const BACKGROUND_ARROW_HEIGHT: Px = px(16.0);
pub const BACKGROUND_ARROW_OFFSET_X: Px = px(8.0);

const ROW_HEIGHT: Px = px(20.0);

const RARITIES: [Rarity; 4] = [
    Rarity::Common,
    Rarity::Rare,
    Rarity::Epic,
    Rarity::Legendary,
];

pub(crate) struct LevelUpTooltip {
    pub current_level: usize,
    pub next_level: usize,
}

impl Component for LevelUpTooltip {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            current_level,
            next_level,
        } = self;

        let current_weights =
            level_rarity_weights(NonZeroUsize::new(current_level.max(1)).unwrap());
        let next_weights = level_rarity_weights(NonZeroUsize::new(next_level.max(1)).unwrap());

        let container_wh = TOOLTIP_WH;

        ctx.translate((PADDING, PADDING)).compose(|ctx| {
            table::horizontal([
                table::ratio_no_clip(1, |wh, ctx| {
                    render_level_column(ctx, wh, current_level, &current_weights);
                }),
                table::fixed_no_clip(ARROW_WIDTH, |wh, ctx| {
                    table::padding_no_clip(
                        12.px(),
                        table::horizontal([
                            table::ratio_no_clip(1, |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Play).size(IconSize::Medium).wh(wh));
                            }),
                            table::ratio_no_clip(1, |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Play).size(IconSize::Medium).wh(wh));
                            }),
                            table::ratio_no_clip(1, |wh, ctx| {
                                ctx.add(Icon::new(IconKind::Play).size(IconSize::Medium).wh(wh));
                            }),
                        ]),
                    )(wh, ctx);
                }),
                table::ratio_no_clip(1, |wh, ctx| {
                    render_level_column(ctx, wh, next_level, &next_weights);
                }),
            ])(
                Wh::new(
                    TOOLTIP_WH.width - PADDING * 2.0,
                    TOOLTIP_WH.height - PADDING * 2.0,
                ),
                ctx,
            );
        });

        Self::render_background(ctx, container_wh);
    }
}

fn render_level_column(ctx: ComposeCtx<'_, '_>, wh: Wh<Px>, level: usize, weights: &[usize; 4]) {
    let total = weights.iter().copied().sum::<usize>() as f32;

    table::vertical([
        table::fixed_no_clip(ROW_HEIGHT, |wh, ctx| {
            ctx.add(memoized_text(&level, |mut builder| {
                builder
                    .headline()
                    .size(FontSize::Medium)
                    .color(palette::ON_SURFACE_VARIANT)
                    .text(format!("Lv {level}"))
                    .render_center(wh)
            }));
        }),
        table::ratio_no_clip(1, |wh, ctx| render_rarity_rows(ctx, wh, total, weights)),
    ])(wh, ctx);
}

fn render_rarity_rows(ctx: ComposeCtx<'_, '_>, wh: Wh<Px>, total: f32, weights: &[usize; 4]) {
    table::padding_no_clip(
        PADDING,
        table::vertical([
            table::ratio_no_clip(1, |wh, mut ctx| {
                render_rarity_row(&mut ctx, wh, RARITIES[0], weights[0], total);
            }),
            table::ratio_no_clip(1, |wh, mut ctx| {
                render_rarity_row(&mut ctx, wh, RARITIES[1], weights[1], total);
            }),
            table::ratio_no_clip(1, |wh, mut ctx| {
                render_rarity_row(&mut ctx, wh, RARITIES[2], weights[2], total);
            }),
            table::ratio_no_clip(1, |wh, mut ctx| {
                render_rarity_row(&mut ctx, wh, RARITIES[3], weights[3], total);
            }),
        ]),
    )(wh, ctx)
}

fn render_rarity_row(
    ctx: &mut ComposeCtx<'_, '_>,
    wh: Wh<Px>,
    rarity: Rarity,
    weight: usize,
    total: f32,
) {
    let pct = if total > 0.0 {
        (weight as f32 / total) * 100.0
    } else {
        0.0
    };

    let icon_wh = Wh::new(wh.height, wh.height);
    ctx.add(
        Icon::new(IconKind::Rarity { rarity })
            .size(IconSize::Medium)
            .wh(icon_wh),
    );

    ctx.add(
        TypographyBuilder::new()
            .headline()
            .size(FontSize::Small)
            .color(rarity.color())
            .text(format!("{pct:.1}%"))
            .render_right_center(wh),
    );
}

impl LevelUpTooltip {
    fn render_background(ctx: &RenderCtx, wh: Wh<Px>) {
        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE,
            shadow: true,
            arrow: Some(PaperArrow {
                side: crate::theme::paper_container::ArrowSide::Right,
                width: BACKGROUND_ARROW_WIDTH,
                height: BACKGROUND_ARROW_HEIGHT,
                offset: wh.height / 2.0,
            }),
        });
    }
}
