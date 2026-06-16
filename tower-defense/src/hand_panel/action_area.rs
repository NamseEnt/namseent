use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::{
    game_state::{flow::GameFlow, mutate_game_state, use_game_state},
    icon::{Icon, IconKind, IconSize},
    theme::{
        button::{Button, ButtonColor, ButtonVariant},
        typography::memoized_text,
    },
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

use super::constants::INNER_PADDING;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HandActionFlow {
    SelectingTower,
    PlacingTower,
}

impl HandActionFlow {
    pub(super) fn from_game_flow(flow: &GameFlow) -> Option<Self> {
        match flow {
            GameFlow::SelectingTower(_) => Some(Self::SelectingTower),
            GameFlow::PlacingTower => Some(Self::PlacingTower),
            _ => None,
        }
    }
}

pub(super) struct HandActionArea {
    pub wh: Wh<Px>,
    pub flow: HandActionFlow,
    pub active_flow: Option<HandActionFlow>,
    pub tower_template: Option<crate::game_state::tower::TowerTemplate>,
}

struct HandRerollButton<'a> {
    wh: Wh<Px>,
    used_dice: usize,
    max_dice: usize,
    health_cost: usize,
    disabled: bool,
    on_click: &'a dyn Fn(),
}

impl Component for HandRerollButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            used_dice,
            max_dice,
            health_cost,
            disabled,
            on_click,
        } = self;

        let (hovering, set_hovering) = ctx.state(|| false);
        let (tooltip_id, _) = ctx.state(crate::tooltip::TooltipId::new);

        ctx.add(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                move |event| {
                    let Event::MouseMove { event } = event else {
                        return;
                    };
                    if event.is_local_xy_in() && health_cost > 0 {
                        if !*hovering {
                            set_hovering.set(true);
                            let origin = event.global_xy - event.local_xy();
                            crate::tooltip::show_tooltip(
                                *tooltip_id,
                                Rect::from_xy_wh(origin, wh),
                                crate::tooltip::TooltipPlacement::RightOf,
                                crate::tooltip::TooltipContent::Reroll { health_cost },
                            );
                        }
                    } else if *hovering {
                        set_hovering.set(false);
                        crate::tooltip::hide_tooltip(*tooltip_id);
                    }
                },
            ),
        );

        ctx.add(
            Button::new(wh, on_click, &|wh, color, ctx| {
                ctx.add(memoized_text(
                    (&color, &used_dice, &max_dice, &health_cost),
                    |mut builder| {
                        let mut builder = builder
                            .headline()
                            .size(crate::theme::typography::FontSize::Large)
                            .icon(IconKind::Refresh);

                        if health_cost > 0 {
                            builder = builder.space().icon(IconKind::Health);
                        }

                        builder.color(color).render_center(wh)
                    },
                ));
            })
            .disabled(disabled),
        );
    }
}

impl Component for HandActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            flow,
            active_flow,
            tower_template,
        } = self;
        let game_state = use_game_state(ctx);
        let action_padding = INNER_PADDING * 2.0;
        let is_active_flow = active_flow == Some(flow);

        match flow {
            HandActionFlow::SelectingTower => {
                let reroll_selected = || {
                    if !is_active_flow {
                        return;
                    }
                    mutate_game_state(move |game_state| {
                        game_state.action(crate::game_state::GameStateAction::CardReroll);
                    });
                };

                let use_tower = || {
                    if !is_active_flow {
                        return;
                    }
                    if let Some(template) = tower_template.clone() {
                        mutate_game_state(move |state| {
                            state.action(crate::game_state::GameStateAction::StartPlacingTower(
                                template,
                            ));
                        });
                    }
                };

                ctx.compose(|ctx| {
                    table::padding_no_clip(
                        action_padding,
                        table::vertical([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                let health_cost =
                                    game_state.stage_modifiers.get_reroll_health_cost();
                                let max_dice = game_state.max_dice_chance();
                                let used_dice = max_dice.saturating_sub(game_state.left_dice);
                                let disabled = !is_active_flow
                                    || game_state.left_dice == 0
                                    || (game_state.hp - health_cost as f32) < 1.0;

                                ctx.add(HandRerollButton {
                                    wh,
                                    used_dice,
                                    max_dice,
                                    health_cost,
                                    disabled,
                                    on_click: &reroll_selected,
                                });
                            }),
                            table::ratio_no_clip(1, |_, _| {}),
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(wh, &use_tower, &|wh, _text_color, ctx| {
                                        ctx.add(
                                            Icon::new(IconKind::Accept)
                                                .size(IconSize::Large)
                                                .wh(wh),
                                        );
                                    })
                                    .disabled(!is_active_flow || tower_template.is_none()),
                                );
                            }),
                        ]),
                    )(wh, ctx);
                });
            }
            HandActionFlow::PlacingTower => {
                let start_defense = || {
                    if !is_active_flow {
                        return;
                    }
                    mutate_game_state(|state| {
                        state.action(crate::game_state::GameStateAction::StartDefense);
                    });
                };

                ctx.compose(|ctx| {
                    table::padding_no_clip(
                        action_padding,
                        table::vertical([table::fixed_no_clip(48.px(), |wh, ctx| {
                            ctx.add(
                                Button::new(wh, &start_defense, &|wh, text_color, ctx| {
                                    ctx.add(memoized_text(&text_color, |mut builder| {
                                        builder
                                            .headline()
                                            .bold()
                                            .color(palette::WHITE)
                                            .stroke(2.px(), crate::theme::palette::DARK_CHARCOAL)
                                            .text("START")
                                            .render_center(wh)
                                    }));
                                })
                                .variant(ButtonVariant::Contained)
                                .color(ButtonColor::Primary)
                                .disabled(!is_active_flow),
                            );
                        })]),
                    )(wh, ctx);
                });
            }
        }

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: crate::theme::palette::SURFACE_CONTAINER_LOW,
            outline_color: None,
            shadow: true,
            arrow: None,
        });
    }
}
