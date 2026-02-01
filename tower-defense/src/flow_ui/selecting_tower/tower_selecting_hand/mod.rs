mod get_highest_tower;
mod tower_preview;

use crate::card::Card;
use crate::game_state::flow::GameFlow;
use crate::game_state::mutate_game_state;
use crate::hand::{HAND_WH, Hand, HandComponent, HandSlotId};
use crate::icon::{Icon, IconKind, IconSize};
use crate::palette;
use crate::theme::{button::Button, typography::{memoized_text}};
use get_highest_tower::get_highest_tower_template;
use namui::*;
use namui_prebuilt::table;
pub use tower_preview::{TowerPreview, TowerPreviewContent};

const PADDING: Px = px(4.);

pub struct TowerSelectingHand<'a> {
    pub hand: &'a Hand<Card>,
}

impl<'a> Component for TowerSelectingHand<'a> {
    fn render(self, ctx: &RenderCtx) {
        let Self { hand } = self;
        let screen_wh = screen::size().into_type::<Px>();

        let game_state = crate::game_state::use_game_state(ctx);
        let selected_hand_slot_ids = ctx.track_eq(&hand.selected_slot_ids());
        let some_selected = ctx.memo(|| !selected_hand_slot_ids.is_empty());
        let using_cards = ctx.memo(|| {
            let slot_ids = {
                if !selected_hand_slot_ids.is_empty() {
                    selected_hand_slot_ids.clone_inner()
                } else {
                    hand.active_slot_ids()
                }
            };
            hand.get_items(&slot_ids).cloned().collect::<Vec<Card>>()
        });
        let tower_template = ctx.memo({
            let upgrade_state = &game_state.upgrade_state;
            let rerolled_count = game_state.rerolled_count;
            move || get_highest_tower_template(&using_cards, upgrade_state, rerolled_count)
        });

        let reroll_selected = || {
            if game_state.left_reroll_chance == 0 || selected_hand_slot_ids.is_empty() {
                return;
            }
            let selected_hand_slot_ids = selected_hand_slot_ids.clone_inner();
            mutate_game_state(move |game_state| {
                if game_state.left_reroll_chance == 0 || selected_hand_slot_ids.is_empty() {
                    return;
                }
                let health_cost = game_state
                    .stage_modifiers
                    .get_card_selection_hand_reroll_health_cost();
                if (game_state.hp - health_cost as f32) < 1.0 {
                    return; // 체력이 부족하면 리롤하지 않음
                }
                {
                    let GameFlow::SelectingTower(flow) = &mut game_state.flow else {
                        unreachable!()
                    };
                    let select_count = selected_hand_slot_ids.len();
                    flow.hand.delete_slots(&selected_hand_slot_ids);
                    (0..select_count).for_each(|_| {
                        flow.hand.push(Card::new_random());
                    });
                }

                game_state.left_reroll_chance -= 1;
                game_state.rerolled_count += 1;
                game_state.take_damage(health_cost as f32);
            });
        };

        let on_card_click = |id: HandSlotId| {
            mutate_game_state(move |game_state| {
                let GameFlow::SelectingTower(flow) = &mut game_state.flow else {
                    unreachable!()
                };
                if flow.hand.selected_slot_ids().contains(&id) {
                    flow.hand.deselect_slot(id);
                } else {
                    flow.hand.select_slot(id);
                }
            });
        };

        let use_tower = || {
            let tower_template = tower_template.clone_inner();
            mutate_game_state(move |state| {
                state.goto_placing_tower(tower_template);
            });
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio_no_clip(1, |_, _| {}),
                table::fixed_no_clip(
                    HAND_WH.height,
                    table::horizontal([
                        table::ratio_no_clip(1, |_, _| {}),
                        table::fixed_no_clip(
                            HAND_WH.height,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(TowerPreview {
                                    wh,
                                    tower_template: &tower_template,
                                });
                            }),
                        ),
                        table::fixed_no_clip(HAND_WH.width, |_wh, ctx| {
                            ctx.add(HandComponent {
                                hand,
                                on_click: &on_card_click,
                            });
                        }),
                        table::fixed(
                            HAND_WH.height,
                            table::padding(PADDING, |wh, ctx| {
                                ctx.add(InteractionArea {
                                    wh,
                                    some_selected: *some_selected,
                                    reroll_selected: &reroll_selected,
                                    use_tower: &use_tower,
                                });
                            }),
                        ),
                        table::ratio_no_clip(1, |_, _| {}),
                    ]),
                ),
            ])(screen_wh, ctx);
        });
    }
}

struct InteractionArea<'a> {
    wh: Wh<Px>,
    some_selected: bool,
    reroll_selected: &'a dyn Fn(),
    use_tower: &'a dyn Fn(),
}
impl Component for InteractionArea<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            some_selected,
            reroll_selected,
            use_tower,
        } = self;
        let game_state = crate::game_state::use_game_state(ctx);
        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::fixed(48.px(), |wh, ctx| {
                        ctx.add(
                            Button::new(
                                wh,
                                &|| {
                                    reroll_selected();
                                },
                                &|wh, color, ctx| {
                                    let health_cost = game_state
                                        .stage_modifiers
                                        .get_card_selection_hand_reroll_health_cost();
                                    let reroll_count = (
                                        game_state.rerolled_count,
                                        game_state.left_reroll_chance,
                                    );

                                    ctx.add(memoized_text(
                                        (&color, &reroll_count.0, &reroll_count.1, &health_cost),
                                        |mut builder| {
                                            let reroll_text = format!(
                                                "{}/{}",
                                                reroll_count.0,
                                                reroll_count.0 + reroll_count.1,
                                            );

                                            let mut builder = builder
                                                .headline()
                                                .icon::<()>(IconKind::Refresh)
                                                .space()
                                                .text(reroll_text);

                                            if health_cost > 0 {
                                                builder = builder.space().icon::<()>(IconKind::Health);
                                            }

                                            builder.color(color).render_center(wh)
                                        },
                                    ));
                                },
                            )
                            .disabled(
                                !some_selected || game_state.left_reroll_chance == 0 || {
                                    let health_cost = game_state
                                        .stage_modifiers
                                        .get_card_selection_hand_reroll_health_cost();
                                    (game_state.hp - health_cost as f32) < 1.0
                                },
                            ),
                        );
                    }),
                    table::ratio(1, |_, _| {}),
                    table::fixed(48.px(), |wh, ctx| {
                        ctx.add(Button::new(
                            wh,
                            &|| {
                                use_tower();
                            },
                            &|wh, _text_color, ctx| {
                                ctx.add(Icon::new(IconKind::Accept).size(IconSize::Large).wh(wh));
                            },
                        ));
                    }),
                ]),
            )(wh, ctx);
        });
        ctx.add(rect(RectParam {
            rect: wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}
