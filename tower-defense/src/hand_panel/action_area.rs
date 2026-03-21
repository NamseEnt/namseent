use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::{
    card::Card,
    game_state::{flow::GameFlow, mutate_game_state, use_game_state},
    icon::{Icon, IconKind, IconSize},
    sound,
    theme::{
        button::{Button, ButtonColor, ButtonVariant},
        typography::memoized_text,
    },
};
use namui::*;
use namui_prebuilt::table;

use super::constants::INNER_PADDING;

pub(super) struct HandActionArea {
    pub wh: Wh<Px>,
    pub selected_slot_ids: Vec<crate::hand::HandSlotId>,
    pub tower_template: Option<crate::game_state::tower::TowerTemplate>,
}

impl Component for HandActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            selected_slot_ids: _,
            tower_template: _,
        } = self;
        let game_state = use_game_state(ctx);
        let action_padding = INNER_PADDING * 2.0;

        match &game_state.flow {
            GameFlow::SelectingTower(_) => {
                let selected_slot_ids = self.selected_slot_ids.clone();
                let some_selected = !selected_slot_ids.is_empty();

                let tower_template = self.tower_template.clone();

                let reroll_selected = || {
                    if game_state.left_dice == 0 || selected_slot_ids.is_empty() {
                        return;
                    }
                    let selected_slot_ids = selected_slot_ids.clone();
                    mutate_game_state(move |game_state| {
                        if game_state.left_dice == 0 || selected_slot_ids.is_empty() {
                            return;
                        }
                        let health_cost = game_state.stage_modifiers.get_reroll_health_cost();
                        if (game_state.hp - health_cost as f32) < 1.0 {
                            return;
                        }

                        let select_count = selected_slot_ids.len();
                        game_state.hand.delete_slots(&selected_slot_ids);
                        (0..select_count).for_each(|_| {
                            game_state
                                .hand
                                .push(crate::hand::HandItem::Card(Card::new_random()));
                        });
                        sound::play_card_draw_sounds(select_count);

                        game_state.left_dice -= 1;
                        game_state.rerolled_count += 1;
                        game_state.take_damage(health_cost as f32);
                    });
                };

                let use_tower = || {
                    if let Some(template) = tower_template.clone() {
                        mutate_game_state(move |state| {
                            state.goto_placing_tower(template);
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
                                ctx.add(
                                    Button::new(wh, &reroll_selected, &|wh, color, ctx| {
                                        let max_dice = game_state.max_dice_chance();
                                        let used_dice =
                                            max_dice.saturating_sub(game_state.left_dice);
                                        ctx.add(memoized_text(
                                            (&color, &used_dice, &max_dice, &health_cost),
                                            |mut builder| {
                                                let mut builder = builder
                                                    .headline()
                                                    .size(crate::theme::typography::FontSize::Large)
                                                    .icon(IconKind::Refresh);

                                                if health_cost > 0 {
                                                    builder =
                                                        builder.space().icon(IconKind::Health);
                                                }

                                                builder.color(color).render_center(wh)
                                            },
                                        ));
                                    })
                                    .disabled(
                                        !some_selected || game_state.left_dice == 0 || {
                                            (game_state.hp - health_cost as f32) < 1.0
                                        },
                                    ),
                                );
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
                                    .disabled(tower_template.is_none()),
                                );
                            }),
                        ]),
                    )(wh, ctx);
                });
            }
            GameFlow::PlacingTower => {
                let start_defense = || {
                    mutate_game_state(|game_state| {
                        game_state.goto_defense();
                    });
                };

                ctx.compose(|ctx| {
                    table::padding_no_clip(
                        action_padding,
                        table::vertical([
                            table::ratio_no_clip(1, |_, _| {}),
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(
                                    Button::new(wh, &start_defense, &|wh, text_color, ctx| {
                                        ctx.add(memoized_text(&text_color, |mut builder| {
                                            builder
                                                .headline()
                                                .color(text_color)
                                                .text("START")
                                                .render_center(wh)
                                        }));
                                    })
                                    .long_press_time(1.sec())
                                    .variant(ButtonVariant::Contained)
                                    .color(ButtonColor::Primary),
                                );
                            }),
                        ]),
                    )(wh, ctx);
                });
            }
            _ => {}
        }

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: crate::theme::palette::SURFACE_CONTAINER_LOW,
            shadow: true,
            arrow: None,
        });
    }
}
