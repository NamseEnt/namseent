use crate::{
    animation::with_spring,
    game_state::{UpgradeInfo, UpgradeInfoDescription, get_upgrade_infos, use_game_state},
    l10n::Locale,
    palette,
    theme::{
        paper_container::{
            ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
        },
        typography::{FontSize, memoized_text},
    },
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const PADDING: Px = px(8.);
const ITEM_SIZE: Px = px(64.);
const ITEM_GAP: Px = px(12.);
const ITEM_MARGIN: Px = px(6.);

mod tooltip {
    use namui::*;
    pub const PADDING: Px = px(8.0);
    pub const MAX_WIDTH: Px = px(240.0);
    pub const ARROW_WIDTH: Px = px(8.0);
    pub const ARROW_HEIGHT: Px = px(16.0);
    pub const OFFSET_X: Px = px(2.0);
}

pub struct Upgrades {
    pub wh: Wh<Px>,
}

impl Component for Upgrades {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let game_state = use_game_state(ctx);
        let locale = game_state.text().locale();
        let upgrade_infos = get_upgrade_infos(&game_state.upgrade_state, &game_state.text());

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            ctx.add(AutoScrollViewWithCtx {
                wh,
                scroll_bar_width: PADDING,
                content: |mut ctx| {
                    for upgrade_info in upgrade_infos.iter().cloned() {
                        ctx.add(UpgradeThumbnailItem {
                            wh: Wh::new(ITEM_SIZE, ITEM_SIZE),
                            upgrade_info,
                            locale,
                        });
                        ctx = ctx.translate(Xy::new(0.px(), ITEM_SIZE + ITEM_GAP));
                    }
                },
            });
        };

        ctx.compose(|ctx| {
            table::horizontal([table::fixed_no_clip(
                wh.width,
                table::padding_no_clip(PADDING, scroll_view),
            )])(wh, ctx);
        });
    }
}

struct UpgradeThumbnailItem {
    wh: Wh<Px>,
    upgrade_info: UpgradeInfo,
    locale: Locale,
}

impl Component for UpgradeThumbnailItem {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade_info,
            locale,
        } = self;

        let (hovering, set_hovering) = ctx.state(|| false);
        let (hover_start, set_hover_start) = ctx.state(|| None::<Instant>);
        let tooltip_scale = with_spring(
            ctx,
            if *hovering { 1.0 } else { 0.0 },
            0.0,
            |v| v * v,
            || 0.0,
        );

        if *hovering && (*hover_start).is_none() {
            set_hover_start.set(Some(Instant::now()));
        }
        if !*hovering {
            set_hover_start.set(None);
        }

        let hover_rotation = if let Some(start) = *hover_start {
            ((Instant::now() - start).as_secs_f32() * 25.0).sin() * 3.0
        } else {
            0.0
        };

        ctx.compose(|ctx| {
            if tooltip_scale > 0.01 {
                let tooltip = ctx.ghost_add(
                    "upgrade-tooltip",
                    UpgradeTooltip {
                        description: upgrade_info.description.clone(),
                        locale,
                    },
                );
                if let Some(tooltip_wh) = tooltip.bounding_box().map(|rect| rect.wh()) {
                    let y = (wh.height - tooltip_wh.height) / 2.0;
                    let pivot = Xy::new(0.px(), tooltip_wh.height / 2.0);
                    let base = Xy::new(
                        wh.width
                            + tooltip::OFFSET_X
                            + tooltip::ARROW_WIDTH
                            + ITEM_MARGIN * 2.0
                            + PADDING,
                        y,
                    );
                    ctx.translate(base + pivot)
                        .scale(Xy::new(tooltip_scale, tooltip_scale))
                        .translate(Xy::new(-pivot.x, -pivot.y))
                        .on_top()
                        .add(tooltip);
                }
            }
        });

        let ctx = ctx.translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN));
        let thumbnail_wh = Wh::new(ITEM_SIZE - PADDING * 2.0, ITEM_SIZE - PADDING * 2.0);

        ctx.translate(Xy::single(PADDING)).compose(|ctx| {
            let pivot = Xy::new(thumbnail_wh.width * 0.5, thumbnail_wh.height * 0.5);
            ctx.translate(pivot)
                .rotate(hover_rotation.deg())
                .translate(Xy::new(-pivot.x, -pivot.y))
                .add(upgrade_info.upgrade_kind.thumbnail(thumbnail_wh));
        });

        ctx.add(
            simple_rect(
                Wh::new(ITEM_SIZE, ITEM_SIZE),
                Color::TRANSPARENT,
                0.px(),
                Color::TRANSPARENT,
            )
            .attach_event(move |event| {
                let Event::MouseMove { event } = event else {
                    return;
                };
                if event.is_local_xy_in() {
                    set_hovering.set(true);
                } else {
                    set_hovering.set(false);
                }
            }),
        );
    }
}

struct UpgradeTooltip {
    description: UpgradeInfoDescription,
    locale: Locale,
}

impl Component for UpgradeTooltip {
    fn render(self, ctx: &RenderCtx) {
        let UpgradeTooltip {
            description,
            locale,
        } = self;

        let max_width = tooltip::MAX_WIDTH;
        let text_max = max_width - (tooltip::PADDING * 2.0);

        let content = ctx.ghost_compose("tooltip-content", |ctx| {
            table::vertical([table::fit(table::FitAlign::LeftTop, |compose_ctx| {
                let description_key = description.key();
                compose_ctx.add(memoized_text(
                    (&description_key, &text_max, &locale.language),
                    |mut builder| {
                        let builder = builder
                            .paragraph()
                            .size(FontSize::Medium)
                            .max_width(text_max);
                        let builder = match &description {
                            UpgradeInfoDescription::Single(text) => {
                                builder.l10n(text.clone(), &locale)
                            }
                            UpgradeInfoDescription::PrefixSuffix { prefix, suffix } => builder
                                .l10n(prefix.clone(), &locale)
                                .space()
                                .l10n(suffix.clone(), &locale),
                        };
                        builder.render_left_top()
                    },
                ));
            })])(Wh::new(text_max, f32::MAX.px()), ctx);
        });

        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };

        let container_wh = content_wh + Wh::single(tooltip::PADDING * 2.0);

        ctx.translate((tooltip::PADDING, tooltip::PADDING))
            .add(content);
        Self::render_background(ctx, container_wh);
    }
}

impl UpgradeTooltip {
    fn render_background(ctx: &RenderCtx, wh: Wh<Px>) {
        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER,
            shadow: true,
            arrow: Some(PaperArrow {
                side: ArrowSide::Left,
                width: tooltip::ARROW_WIDTH,
                height: tooltip::ARROW_HEIGHT,
                offset: wh.height / 2.0,
            }),
        });
    }
}
