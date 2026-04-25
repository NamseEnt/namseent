mod clear_progress;
mod stat_row;
mod tower_damage_panel;

use crate::flow_ui::result::clear_progress::ClearProgress;
use crate::flow_ui::result::stat_row::StatRow;
use crate::flow_ui::result::tower_damage_panel::TowerDamagePanel;
use crate::game_state::flow::GameFlow;
use crate::game_state::use_game_state;
use crate::game_state::{restart_game, set_modal};
use crate::icon::IconKind;
use crate::l10n::ui::ResultModalText;
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::{
    palette,
    typography::{self, memoized_text},
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TITLE_HEIGHT: Px = px(36.);
const PADDING: Px = px(16.);
const PROGRESS_BAR_HEIGHT: Px = px(24.);
const TOWER_DAMAGE_ITEM_HEIGHT: Px = px(64.);

pub struct ResultModal;

impl Component for ResultModal {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let clear_rate = match &game_state.flow {
            GameFlow::Result { clear_rate } => *clear_rate,
            _ => 0.0,
        };

        let screen_wh = screen::size().into_type::<Px>();
        let height = (screen_wh.height * 0.6).clamp(px(640.0), px(720.0));
        let width = (screen_wh.width * 0.8).clamp(px(480.0), px(720.0));
        let result_modal_wh = Wh { width, height };
        let modal_xy = ((screen_wh - result_modal_wh) * 0.5).to_xy();

        ctx.compose(|ctx| {
            let ctx = ctx.translate(modal_xy);

            ctx.compose(|ctx| {
                table::padding_no_clip(
                    PADDING,
                    table::vertical([
                        table::fixed_no_clip(TITLE_HEIGHT, |wh, ctx| {
                            ctx.add(memoized_text((), |mut builder| {
                                builder
                                    .headline()
                                    .size(typography::FontSize::Large)
                                    .text(game_state.text().result_modal(ResultModalText::Title))
                                    .render_center(wh)
                            }));
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::fixed_no_clip(PROGRESS_BAR_HEIGHT, |wh, ctx| {
                            ctx.add(ClearProgress { wh, clear_rate });
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::fixed_no_clip(24.px(), |wh, ctx| {
                            ctx.add(StatRow {
                                wh,
                                label: game_state
                                    .text()
                                    .result_modal(ResultModalText::MaxPerfectClearLabel),
                                value: format!(
                                    "{}회",
                                    game_state.metrics.max_consecutive_perfect_clears
                                ),
                                icon_kind: None,
                            });
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::fixed_no_clip(24.px(), |wh, ctx| {
                            ctx.add(StatRow {
                                wh,
                                label: game_state
                                    .text()
                                    .result_modal(ResultModalText::TotalGoldLabel),
                                value: format!("{}", game_state.metrics.total_gold_earned),
                                icon_kind: Some(IconKind::Gold),
                            });
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::fixed_no_clip(24.px(), |wh, ctx| {
                            let total_damage = game_state
                                .metrics
                                .tower_damage_stats
                                .iter()
                                .map(|stat| stat.total_damage)
                                .sum::<f32>();
                            ctx.add(StatRow {
                                wh,
                                label: game_state
                                    .text()
                                    .result_modal(ResultModalText::TotalDamageLabel),
                                value: format!("{:.0}", total_damage),
                                icon_kind: Some(IconKind::Damage),
                            });
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::fixed_no_clip(24.px(), |wh, ctx| {
                            ctx.add(StatRow {
                                wh,
                                label: game_state
                                    .text()
                                    .result_modal(ResultModalText::RerollCountLabel),
                                value: format!("{}회", game_state.rerolled_count),
                                icon_kind: None,
                            });
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::ratio_no_clip(1, |wh, ctx| {
                            ctx.add(TowerDamagePanel {
                                wh,
                                tower_stats: &game_state.metrics.tower_damage_stats,
                                empty_text: game_state
                                    .text()
                                    .result_modal(ResultModalText::NoTowerDamage),
                            });
                        }),
                        table::fixed_no_clip(PADDING, |_wh, _ctx| {}),
                        table::fixed_no_clip(48.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        restart_game();
                                        set_modal(None);
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.add(memoized_text(&text_color, |mut builder| {
                                            builder
                                                .headline()
                                                .size(typography::FontSize::Medium)
                                                .color(text_color)
                                                .text(
                                                    game_state.text().result_modal(
                                                        ResultModalText::RestartButton,
                                                    ),
                                                )
                                                .render_center(wh)
                                        }));
                                    },
                                )
                                .color(ButtonColor::Primary)
                                .variant(ButtonVariant::Contained),
                            );
                        }),
                    ]),
                )(result_modal_wh, ctx);
            });

            ctx.add(PaperContainerBackground {
                width: result_modal_wh.width,
                height: result_modal_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Paper,
                color: palette::SURFACE_CONTAINER,
                shadow: true,
                arrow: None,
            });
        });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK.with_alpha(225),
        ))
        .attach_event(|event| {
            match event {
                Event::MouseDown { event }
                | Event::MouseUp { event }
                | Event::MouseMove { event } => {
                    event.stop_propagation();
                }
                _ => {}
            };
        });
    }
}
