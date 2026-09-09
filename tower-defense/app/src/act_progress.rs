use crate::game_state::use_game_state;
use crate::icon::{Icon, IconKind, IconSize};
use crate::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::{FontSize, memoized_text};
use crate::tooltip::{TooltipContent, TooltipPlacement, WithHoverArea};
use namui::*;
use namui_prebuilt::table;

const PROGRESS_HEIGHT: Px = px(44.);
const LABEL_WIDTH: Px = px(64.);
const NODE_WIDTH: Px = px(32.);
const NODE_ICON_SIZE: Px = px(28.);
const INDICATOR_WIDTH: Px = px(384.);
const BG_OVERSIZE_H: Px = px(4.);
const BG_OVERSIZE_V: Px = px(4.);

pub struct ActProgressStrip {
    pub wh: Wh<Px>,
}

impl Component for ActProgressStrip {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);
        let raw = game_state.raw_core_state();
        let stage = raw.progress().stage;
        let act = td_core::act_for_stage(stage).unwrap_or(1);
        let local_stage = td_core::stage_in_act(stage).unwrap_or(1);

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(INDICATOR_WIDTH, |wh, ctx| {
                    table::horizontal([
                        table::fixed_no_clip(LABEL_WIDTH, |label_wh, ctx| {
                            ctx.add(memoized_text(&act, |mut builder| {
                                builder
                                    .headline()
                                    .size(FontSize::Small)
                                    .text(format!("Act {act}"))
                                    .render_center(label_wh)
                            }));
                        }),
                        table::ratio_no_clip(1, |nodes_wh, ctx| {
                            let nodes = (1..=td_core::STAGES_PER_ACT)
                                .map(|node_stage| {
                                    table::fixed_no_clip(NODE_WIDTH, move |node_wh, ctx| {
                                        let reached = node_stage <= local_stage;
                                        let (icon, tooltip_kind) = match td_core::stage_kind(
                                            (act - 1) * td_core::STAGES_PER_ACT + node_stage,
                                        ) {
                                            Some(td_core::StageKind::Treasure) => (
                                                IconKind::Treasure,
                                                crate::tooltip::StageIndicatorKind::Treasure,
                                            ),
                                            Some(td_core::StageKind::Boss) => (
                                                IconKind::EnemyBoss,
                                                crate::tooltip::StageIndicatorKind::StrongEnemy,
                                            ),
                                            Some(td_core::StageKind::Normal) | None => (
                                                IconKind::EnemyNormal,
                                                crate::tooltip::StageIndicatorKind::Combat,
                                            ),
                                        };
                                        let opacity = if reached { 1.0 } else { 0.35 };
                                        let component_key =
                                            format!("act_progress_stage_{act}_{node_stage}");
                                        ctx.add(WithHoverArea {
                                            component_key,
                                            component: Icon::new(icon)
                                                .size(IconSize::Custom {
                                                    size: NODE_ICON_SIZE,
                                                })
                                                .opacity(opacity)
                                                .wh(node_wh),
                                            placement: TooltipPlacement::Above,
                                            on_enter: move || {
                                                Some(TooltipContent::Stage(tooltip_kind))
                                            },
                                            on_exit: || {},
                                        });
                                    })
                                })
                                .collect::<Vec<_>>();
                            table::horizontal(nodes)(nodes_wh, ctx);
                        }),
                    ])(wh, ctx);
                }),
                table::ratio_no_clip(1, |_, _| {}),
            ])(Wh::new(wh.width, PROGRESS_HEIGHT), ctx);
        });

        ctx.translate((
            (wh.width - INDICATOR_WIDTH) / 2.0 - BG_OVERSIZE_H,
            -BG_OVERSIZE_V,
        ))
        .add(PaperContainerBackground {
            width: INDICATOR_WIDTH + BG_OVERSIZE_H * 2.0,
            height: PROGRESS_HEIGHT + BG_OVERSIZE_V * 2.0,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER_HIGHEST,
            outline_color: None,
            shadow: true,
            arrow: None,
        });
    }
}
