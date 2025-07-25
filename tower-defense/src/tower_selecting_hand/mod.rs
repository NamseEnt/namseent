mod get_highest_tower;
mod tower_preview;

use crate::{
    card::Card,
    game_state::{
        flow::GameFlow,
        mutate_game_state,
        quest::{QuestTriggerEvent, on_quest_trigger_event},
    },
    l10n::ui::TopBarText,
    palette,
};
use get_highest_tower::get_highest_tower_template;
use namui::*;
use namui_prebuilt::{button, table, typography};
use std::iter::once;
use tower_preview::TowerPreview;

const HAND_HEIGHT: Px = px(160.);
const CARD_WIDTH: Px = px(120.);
const PADDING: Px = px(4.);

pub struct TowerSelectingHand<'a> {
    pub screen_wh: Wh<Px>,
    pub cards: &'a [Card],
}
impl Component for TowerSelectingHand<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh, cards } = self;

        let game_state = crate::game_state::use_game_state(ctx);
        let (selected, set_selected) = ctx.state(|| [false, false, false, false, false]);
        let some_selected = ctx.memo(|| selected.iter().any(|selected| *selected));
        let using_cards = ctx.memo(|| {
            let selected_cards = cards
                .iter()
                .zip(selected.iter())
                .filter_map(|(card, selected)| {
                    if *selected {
                        return Some(*card);
                    }
                    None
                })
                .collect::<Vec<_>>();
            if !selected_cards.is_empty() {
                return selected_cards;
            }
            cards.to_vec()
        });
        let tower_template = ctx.memo(|| {
            get_highest_tower_template(
                &using_cards,
                &game_state.upgrade_state,
                game_state.rerolled_count,
            )
        });

        let reroll_selected = || {
            if game_state.left_reroll_chance == 0 || selected.is_empty() {
                return;
            }
            let selected = selected.clone_inner();
            mutate_game_state(move |game_state| {
                if game_state.left_reroll_chance == 0 {
                    return;
                }
                let GameFlow::SelectingTower { cards } = &mut game_state.flow else {
                    return;
                };
                for (index, selected) in selected.iter().enumerate() {
                    if !selected {
                        continue;
                    }
                    cards[index] = Card::new_random();
                }
                game_state.left_reroll_chance -= 1;
                game_state.rerolled_count += 1;

                on_quest_trigger_event(game_state, QuestTriggerEvent::Reroll);
            });
            set_selected.mutate(move |set_selected| {
                set_selected
                    .iter_mut()
                    .for_each(|selected| *selected = false);
            });
        };

        let toggle_selected = |index: usize| {
            set_selected.mutate(move |selected| {
                selected[index] = !selected[index];
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
                    HAND_HEIGHT,
                    table::horizontal(
                        once(table::ratio_no_clip(1, |_, _| {}))
                            .chain(once(table::fixed_no_clip(
                                HAND_HEIGHT,
                                table::padding_no_clip(PADDING, |wh, ctx| {
                                    ctx.add(TowerPreview {
                                        wh,
                                        tower_template: &tower_template,
                                    });
                                }),
                            )))
                            .chain(cards.iter().zip(selected.iter()).enumerate().map(
                                |(index, (card, selected))| {
                                    table::fixed(
                                        CARD_WIDTH,
                                        table::padding(PADDING, move |wh, ctx| {
                                            ctx.add(RenderCard {
                                                card: *card,
                                                wh,
                                                selected: *selected,
                                                on_click: &|| {
                                                    toggle_selected(index);
                                                },
                                            });
                                        }),
                                    )
                                },
                            ))
                            .chain(once(table::fixed(
                                HAND_HEIGHT,
                                table::padding(PADDING, |wh, ctx| {
                                    ctx.add(InteractionArea {
                                        wh,
                                        some_selected: *some_selected,
                                        reroll_selected: &reroll_selected,
                                        use_tower: &use_tower,
                                    });
                                }),
                            )))
                            .chain(once(table::ratio(1, |_, _| {}))),
                    ),
                ),
            ])(screen_wh, ctx);
        });
    }
}

struct RenderCard<'a> {
    card: Card,
    wh: Wh<Px>,
    selected: bool,
    on_click: &'a dyn Fn(),
}
impl Component for RenderCard<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            card,
            wh,
            selected,
            on_click,
        } = self;

        ctx.compose(|ctx| {
            ctx.translate(Xy::single(PADDING * 3.))
                .add(typography::title::left_top(
                    format!("{} {}", card.suit, card.rank,),
                    palette::ON_SURFACE,
                ));
        });

        ctx.add(rect(RectParam {
            rect: Rect::from_xy_wh(Xy::single(PADDING * 2.), wh - Wh::single(PADDING * 4.0)),
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));

        ctx.add(
            rect(RectParam {
                rect: wh.to_rect(),
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: palette::OUTLINE,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: Some(RectFill {
                        color: match selected {
                            true => palette::PRIMARY,
                            false => palette::SURFACE_CONTAINER_HIGH,
                        },
                    }),
                    round: Some(RectRound {
                        radius: palette::ROUND,
                    }),
                },
            })
            .attach_event(|event| {
                let Event::MouseDown { event } = event else {
                    return;
                };
                if !event.is_local_xy_in() {
                    return;
                }
                on_click();
            }),
        );
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
                    table::fixed(32.px(), |wh, ctx| {
                        ctx.add(button::TextButton {
                            rect: wh.to_rect(),
                            text: game_state.text().ui(TopBarText::Refresh),
                            text_color: match some_selected {
                                true => palette::ON_PRIMARY,
                                false => palette::ON_SURFACE,
                            },
                            stroke_color: palette::OUTLINE,
                            stroke_width: 1.px(),
                            fill_color: match some_selected {
                                true => palette::PRIMARY,
                                false => palette::SURFACE_CONTAINER_HIGH,
                            },
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: |_| {
                                reroll_selected();
                            },
                        });
                    }),
                    table::ratio(1, |_, _| {}),
                    table::fixed(32.px(), |wh, ctx| {
                        ctx.add(button::TextButton {
                            rect: wh.to_rect(),
                            text: game_state.text().ui(TopBarText::UseTower),
                            text_color: palette::ON_PRIMARY,
                            stroke_color: palette::OUTLINE,
                            stroke_width: 1.px(),
                            fill_color: palette::PRIMARY,
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: |_| {
                                use_tower();
                            },
                        });
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
