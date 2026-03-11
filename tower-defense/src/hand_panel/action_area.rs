use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::{
    card::Card,
    flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template,
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
}

impl Component for HandActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let game_state = use_game_state(ctx);
        let action_padding = INNER_PADDING * 2.0;

        match &game_state.flow {
            GameFlow::SelectingTower(_) => {
                let selected_slot_ids = ctx.track_eq(&game_state.hand.selected_slot_ids());
                let some_selected = ctx.memo(|| !selected_slot_ids.is_empty());

                let using_cards = ctx.memo(|| {
                    let slot_ids = if !selected_slot_ids.is_empty() {
                        selected_slot_ids.clone_inner()
                    } else {
                        game_state.hand.active_slot_ids()
                    };

                    game_state
                        .hand
                        .get_items(&slot_ids)
                        .filter_map(|item| item.as_card().copied())
                        .collect::<Vec<Card>>()
                });

                let tower_template = ctx.memo({
                    let upgrade_state = &game_state.upgrade_state;
                    let rerolled_count = game_state.rerolled_count;
                    move || get_highest_tower_template(&using_cards, upgrade_state, rerolled_count)
                });

                let reroll_selected = || {
                    if game_state.left_reroll_chance == 0 || selected_slot_ids.is_empty() {
                        return;
                    }
                    let selected_slot_ids = selected_slot_ids.clone_inner();
                    mutate_game_state(move |game_state| {
                        if game_state.left_reroll_chance == 0 || selected_slot_ids.is_empty() {
                            return;
                        }
                        let health_cost = game_state
                            .stage_modifiers
                            .get_card_selection_hand_reroll_health_cost();
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

                        game_state.left_reroll_chance -= 1;
                        game_state.rerolled_count += 1;
                        game_state.take_damage(health_cost as f32);
                    });
                };

                let use_tower = || {
                    let tower_template = tower_template.clone_inner();
                    mutate_game_state(move |state| {
                        state.goto_placing_tower(tower_template);
                    });
                };

                ctx.compose(|ctx| {
                    table::padding_no_clip(
                        action_padding,
                        table::vertical([
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                let health_cost = game_state
                                    .stage_modifiers
                                    .get_card_selection_hand_reroll_health_cost();
                                ctx.add(
                                    Button::new(wh, &reroll_selected, &|wh, color, ctx| {
                                        let reroll_count = (
                                            game_state.rerolled_count,
                                            game_state.left_reroll_chance,
                                        );
                                        ctx.add(memoized_text(
                                            (
                                                &color,
                                                &reroll_count.0,
                                                &reroll_count.1,
                                                &health_cost,
                                            ),
                                            |mut builder| {
                                                let reroll_text = format!(
                                                    "{}/{}",
                                                    reroll_count.0,
                                                    reroll_count.0 + reroll_count.1,
                                                );
                                                let mut builder = builder
                                                    .headline()
                                                    .icon(IconKind::Refresh)
                                                    .space()
                                                    .text(reroll_text);

                                                if health_cost > 0 {
                                                    builder =
                                                        builder.space().icon(IconKind::Health);
                                                }

                                                builder.color(color).render_center(wh)
                                            },
                                        ));
                                    })
                                    .disabled(
                                        !*some_selected || game_state.left_reroll_chance == 0 || {
                                            (game_state.hp - health_cost as f32) < 1.0
                                        },
                                    ),
                                );
                            }),
                            table::ratio_no_clip(1, |_, _| {}),
                            table::fixed_no_clip(48.px(), |wh, ctx| {
                                ctx.add(Button::new(wh, &use_tower, &|wh, _text_color, ctx| {
                                    ctx.add(
                                        Icon::new(IconKind::Accept).size(IconSize::Large).wh(wh),
                                    );
                                }));
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
