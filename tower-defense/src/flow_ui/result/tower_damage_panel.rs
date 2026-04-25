use crate::game_state::TowerDamageStats;
use crate::game_state::tower::render::{AnimationKind, TowerImage, TowerSpriteWithOverlay};
use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::theme::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
    typography::{self, memoized_text},
};
use namui::*;
use namui_prebuilt::{list_view::ListViewWithCtx, table};

pub(super) struct TowerDamagePanel<'a> {
    pub(super) wh: Wh<Px>,
    pub(super) tower_stats: &'a [TowerDamageStats],
    pub(super) empty_text: &'static str,
}

impl Component for TowerDamagePanel<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            tower_stats,
            empty_text,
        } = self;
        let mut towers: Vec<_> = tower_stats.to_vec();
        towers.sort_by(|a, b| {
            b.total_damage
                .partial_cmp(&a.total_damage)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let (scroll_y, set_scroll_y) = ctx.state(|| 0.px());

        ctx.compose(|ctx| {
            if towers.is_empty() {
                ctx.add(memoized_text((), |mut builder| {
                    builder
                        .headline()
                        .size(typography::FontSize::Medium)
                        .text(empty_text)
                        .render_center(wh)
                }));
                return;
            }

            let max_damage = towers
                .first()
                .map(|entry| entry.total_damage)
                .unwrap_or(1.0)
                .max(1.0);
            let scroll_bar_width = px(4.0);
            let item_wh = Wh {
                width: wh.width - scroll_bar_width,
                height: super::TOWER_DAMAGE_ITEM_HEIGHT,
            };

            ctx.add(ListViewWithCtx {
                height: wh.height,
                scroll_bar_width,
                item_wh,
                items: towers.into_iter().enumerate(),
                scroll_y: *scroll_y,
                set_scroll_y,
                item_render: move |stat, ctx| {
                    ctx.add(TowerDamageRow {
                        wh: item_wh,
                        stat,
                        max_damage,
                    });
                },
            });

            ctx.add(PaperContainerBackground {
                width: wh.width,
                height: wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::PaperSingleLayer,
                color: palette::SURFACE_CONTAINER_LOWEST,
                shadow: false,
                arrow: None,
            });
        });
    }
}

struct TowerDamageRow {
    wh: Wh<Px>,
    stat: TowerDamageStats,
    max_damage: f32,
}

impl Component for TowerDamageRow {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            stat,
            max_damage,
        } = self;
        let bar_ratio = (stat.total_damage / max_damage).clamp(0.0, 1.0);
        ctx.compose(|ctx| {
            table::horizontal([
                table::fixed_no_clip(wh.height, |wh, ctx| {
                    ctx.add(TowerSpriteWithOverlay {
                        image: (stat.tower_kind, AnimationKind::Idle1).image(),
                        wh,
                        suit: Some(stat.suit),
                        rank: Some(stat.rank),
                        alpha: 1.0,
                    });
                }),
                table::ratio_no_clip(
                    1,
                    table::padding_no_clip(
                        8.px(),
                        table::vertical([
                            table::fixed_no_clip(24.px(), |wh, ctx| {
                                let damage_key = format!("tower_damage_{}", stat.tower_id);
                                ctx.add(memoized_text((&damage_key, &wh.width), |mut builder| {
                                    builder
                                        .paragraph()
                                        .size(typography::FontSize::Medium)
                                        .bold()
                                        .with_icon_bold(
                                            IconKind::Damage,
                                            format!("{:.0}", stat.total_damage),
                                        )
                                        .render_right_top(wh.width)
                                }));
                            }),
                            table::ratio_no_clip(1, |wh, ctx| {
                                const PROGRESS_BAR_BG_COLOR: Color =
                                    palette::SURFACE_CONTAINER_HIGHEST;
                                const PROGRESS_BAR_FILL_COLOR: Color = palette::PRIMARY;
                                const PROGRESS_BAR_BORDER_COLOR: Color = palette::OUTLINE;
                                let bar_width = wh.width * bar_ratio;

                                if bar_width > px(0.0) {
                                    ctx.add(rect(RectParam {
                                        rect: Rect::Xywh {
                                            x: px(0.0),
                                            y: px(0.0),
                                            width: bar_width,
                                            height: wh.height,
                                        },
                                        style: RectStyle {
                                            fill: Some(RectFill {
                                                color: PROGRESS_BAR_FILL_COLOR,
                                            }),
                                            stroke: None,
                                            round: Some(RectRound { radius: px(4.0) }),
                                        },
                                    }));
                                }

                                ctx.add(rect(RectParam {
                                    rect: Rect::Xywh {
                                        x: px(0.0),
                                        y: px(0.0),
                                        width: wh.width,
                                        height: wh.height,
                                    },
                                    style: RectStyle {
                                        fill: Some(RectFill {
                                            color: PROGRESS_BAR_BG_COLOR,
                                        }),
                                        stroke: Some(RectStroke {
                                            color: PROGRESS_BAR_BORDER_COLOR,
                                            width: px(1.0),
                                            border_position: BorderPosition::Inside,
                                        }),
                                        round: Some(RectRound { radius: px(4.0) }),
                                    },
                                }));
                            }),
                        ]),
                    ),
                ),
            ])(wh, ctx);
        });
    }
}
