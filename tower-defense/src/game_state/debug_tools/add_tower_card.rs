use crate::card::{REVERSED_RANKS, Rank, SUITS, Suit};
use crate::game_state::tower::{TowerKind, TowerTemplate};
use crate::game_state::{flow::GameFlow, mutate_game_state, use_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::palette;
use crate::theme::typography::{TextAlign, headline, paragraph};
use namui::*;
use namui_prebuilt::table;

const BUTTON_HEIGHT: Px = px(36.);
const GAP: Px = px(8.);
const DROPDOWN_ICON_SIZE: Px = px(16.);
const DROPDOWN_ITEM_HEIGHT: Px = px(32.);

// Dropdown type: 0 = none, 1 = Kind, 2 = Suit, 3 = Rank

const TOWER_KIND_ORDER: [TowerKind; 11] = [
    TowerKind::Barricade,
    TowerKind::High,
    TowerKind::OnePair,
    TowerKind::TwoPair,
    TowerKind::ThreeOfAKind,
    TowerKind::Straight,
    TowerKind::Flush,
    TowerKind::FullHouse,
    TowerKind::FourOfAKind,
    TowerKind::StraightFlush,
    TowerKind::RoyalFlush,
];

const EXPECTED_TOWERS_BY_STAGE: [TowerKind; 50] = [
    TowerKind::High,
    TowerKind::OnePair,
    TowerKind::OnePair,
    TowerKind::TwoPair,
    TowerKind::ThreeOfAKind,
    TowerKind::OnePair,
    TowerKind::OnePair,
    TowerKind::ThreeOfAKind,
    TowerKind::FullHouse,
    TowerKind::Straight,
    TowerKind::OnePair,
    TowerKind::TwoPair,
    TowerKind::TwoPair,
    TowerKind::ThreeOfAKind,
    TowerKind::FourOfAKind,
    TowerKind::TwoPair,
    TowerKind::Flush,
    TowerKind::ThreeOfAKind,
    TowerKind::TwoPair,
    TowerKind::FullHouse,
    TowerKind::StraightFlush,
    TowerKind::OnePair,
    TowerKind::TwoPair,
    TowerKind::FourOfAKind,
    TowerKind::TwoPair,
    TowerKind::FullHouse,
    TowerKind::OnePair,
    TowerKind::Flush,
    TowerKind::ThreeOfAKind,
    TowerKind::FullHouse,
    TowerKind::High,
    TowerKind::FullHouse,
    TowerKind::FullHouse,
    TowerKind::FourOfAKind,
    TowerKind::OnePair,
    TowerKind::FullHouse,
    TowerKind::Straight,
    TowerKind::StraightFlush,
    TowerKind::Straight,
    TowerKind::TwoPair,
    TowerKind::FullHouse,
    TowerKind::Straight,
    TowerKind::TwoPair,
    TowerKind::FullHouse,
    TowerKind::StraightFlush,
    TowerKind::OnePair,
    TowerKind::FullHouse,
    TowerKind::High,
    TowerKind::OnePair,
    TowerKind::FullHouse,
];

fn get_expected_tower_for_stage(stage: usize) -> TowerKind {
    if stage == 0 || stage > 50 {
        TowerKind::High
    } else {
        EXPECTED_TOWERS_BY_STAGE[stage - 1]
    }
}

pub struct AddTowerCardTool {
    pub width: Px,
}

impl Component for AddTowerCardTool {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let expected_tower = get_expected_tower_for_stage(game_state.stage);

        let (selected_kind, set_selected_kind) = ctx.state(|| expected_tower);
        let (selected_suit, set_selected_suit) = ctx.state(|| Suit::Spades);
        let (selected_rank, set_selected_rank) = ctx.state(|| Rank::Ace);
        let (is_suit_random, set_is_suit_random) = ctx.state(|| true);
        let (is_rank_random, set_is_rank_random) = ctx.state(|| true);
        // 0 = none, 1 = Kind, 2 = Suit, 3 = Rank
        let (dropdown, set_dropdown) = ctx.state(|| 0u8);

        let add_card = || {
            let kind = *selected_kind;
            let suit = if *is_suit_random {
                use rand::seq::SliceRandom;
                *SUITS.choose(&mut rand::thread_rng()).unwrap()
            } else {
                *selected_suit
            };
            let rank = if *is_rank_random {
                use rand::seq::SliceRandom;
                *REVERSED_RANKS.choose(&mut rand::thread_rng()).unwrap()
            } else {
                *selected_rank
            };
            mutate_game_state(move |gs| {
                if let GameFlow::PlacingTower { hand } = &mut gs.flow {
                    hand.push(TowerTemplate::new(kind, suit, rank));
                } else {
                    gs.stage_modifiers
                        .enqueue_extra_tower_card(kind, suit, rank);
                }
            });
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(headline("Add tower card").build());
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    let info_text = format!(
                        "Stage {}: Expected Tower - {:?}",
                        game_state.stage, expected_tower
                    );
                    ctx.add(
                        paragraph(&info_text)
                            .color(palette::ON_SURFACE_VARIANT)
                            .build(),
                    );
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    table::horizontal([
                        table::ratio(1, |wh, ctx| {
                            let text = format!("{:?}", *selected_kind);
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        if *dropdown == 1 {
                                            set_dropdown.set(0);
                                        } else {
                                            set_dropdown.set(1);
                                        }
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.compose(|ctx| {
                                            table::horizontal([
                                                table::ratio(1, |wh, ctx| {
                                                    ctx.add(
                                                        paragraph(text.clone())
                                                            .color(text_color)
                                                            .align(TextAlign::LeftCenter {
                                                                height: wh.height,
                                                            })
                                                            .build(),
                                                    );
                                                }),
                                                table::fixed(DROPDOWN_ICON_SIZE, |wh, ctx| {
                                                    ctx.add(
                                                        Icon::new(IconKind::Down)
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                    );
                                                }),
                                            ])(wh, ctx);
                                        });
                                    },
                                )
                                .variant(ButtonVariant::Outlined),
                            );
                        }),
                        table::fixed(GAP, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            let text = if *is_suit_random {
                                "Random".to_string()
                            } else {
                                format!("{:?}", *selected_suit)
                            };
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        if *dropdown == 2 {
                                            set_dropdown.set(0);
                                        } else {
                                            set_dropdown.set(2);
                                        }
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.compose(|ctx| {
                                            table::horizontal([
                                                table::ratio(1, |wh, ctx| {
                                                    ctx.add(
                                                        paragraph(text.clone())
                                                            .color(text_color)
                                                            .align(TextAlign::LeftCenter {
                                                                height: wh.height,
                                                            })
                                                            .build(),
                                                    );
                                                }),
                                                table::fixed(DROPDOWN_ICON_SIZE, |wh, ctx| {
                                                    ctx.add(
                                                        Icon::new(IconKind::Down)
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                    );
                                                }),
                                            ])(wh, ctx);
                                        });
                                    },
                                )
                                .variant(ButtonVariant::Outlined),
                            );
                        }),
                        table::fixed(GAP, |_, _| {}),
                        table::ratio(1, |wh, ctx| {
                            let text = if *is_rank_random {
                                "Random".to_string()
                            } else {
                                format!("{:?}", *selected_rank)
                            };
                            ctx.add(
                                Button::new(
                                    wh,
                                    &|| {
                                        if *dropdown == 3 {
                                            set_dropdown.set(0);
                                        } else {
                                            set_dropdown.set(3);
                                        }
                                    },
                                    &|wh, text_color, ctx| {
                                        ctx.compose(|ctx| {
                                            table::horizontal([
                                                table::ratio(1, |wh, ctx| {
                                                    ctx.add(
                                                        paragraph(text.clone())
                                                            .color(text_color)
                                                            .align(TextAlign::LeftCenter {
                                                                height: wh.height,
                                                            })
                                                            .build(),
                                                    );
                                                }),
                                                table::fixed(DROPDOWN_ICON_SIZE, |wh, ctx| {
                                                    ctx.add(
                                                        Icon::new(IconKind::Down)
                                                            .size(IconSize::Custom {
                                                                size: wh.width,
                                                            })
                                                            .wh(wh),
                                                    );
                                                }),
                                            ])(wh, ctx);
                                        });
                                    },
                                )
                                .variant(ButtonVariant::Outlined),
                            );
                        }),
                    ])(Wh::new(self.width, BUTTON_HEIGHT), ctx);
                }),
                table::fixed(GAP, |_, _| {}),
                // Render dropdown menu if open
                match *dropdown {
                    1 => table::fit(table::FitAlign::LeftTop, |ctx| {
                        let selector_width = (self.width - GAP * 2.) / 3.;
                        table::vertical(
                            TOWER_KIND_ORDER
                                .iter()
                                .map(|kind| {
                                    let kind = *kind;

                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        let text = format!("{:?}", kind);
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_kind.set(kind);
                                                    set_dropdown.set(0);
                                                },
                                                &|wh, text_color, ctx| {
                                                    ctx.add(
                                                        paragraph(text.clone())
                                                            .color(text_color)
                                                            .align(TextAlign::LeftCenter {
                                                                height: wh.height,
                                                            })
                                                            .build(),
                                                    );
                                                },
                                            )
                                            .variant(
                                                if *selected_kind == kind {
                                                    ButtonVariant::Contained
                                                } else {
                                                    ButtonVariant::Outlined
                                                },
                                            ),
                                        );
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )(Wh::new(selector_width, f32::MAX.px()), ctx);
                    }),
                    2 => table::fit(table::FitAlign::LeftTop, |ctx| {
                        let selector_width = (self.width - GAP * 2.) / 3.;
                        let mut items: Vec<Option<Suit>> = vec![];

                        // Random button
                        items.push(None);

                        // Suit options
                        for suit in SUITS.iter() {
                            items.push(Some(*suit));
                        }

                        table::vertical(
                            items
                                .into_iter()
                                .map(|suit_opt| {
                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        if let Some(suit) = suit_opt {
                                            let text = format!("{:?}", suit);
                                            ctx.add(
                                                Button::new(
                                                    Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                    &move || {
                                                        set_selected_suit.set(suit);
                                                        set_is_suit_random.set(false);
                                                        set_dropdown.set(0);
                                                    },
                                                    &|wh, text_color, ctx| {
                                                        ctx.add(
                                                            paragraph(text.clone())
                                                                .color(text_color)
                                                                .align(TextAlign::LeftCenter {
                                                                    height: wh.height,
                                                                })
                                                                .build(),
                                                        );
                                                    },
                                                )
                                                .variant(
                                                    if !*is_suit_random && *selected_suit == suit {
                                                        ButtonVariant::Contained
                                                    } else {
                                                        ButtonVariant::Outlined
                                                    },
                                                ),
                                            );
                                        } else {
                                            // Random button
                                            let text = "Random".to_string();
                                            ctx.add(
                                                Button::new(
                                                    Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                    &|| {
                                                        set_is_suit_random.set(true);
                                                        set_dropdown.set(0);
                                                    },
                                                    &|wh, text_color, ctx| {
                                                        ctx.add(
                                                            paragraph(text.clone())
                                                                .color(text_color)
                                                                .align(TextAlign::LeftCenter {
                                                                    height: wh.height,
                                                                })
                                                                .build(),
                                                        );
                                                    },
                                                )
                                                .variant(if *is_suit_random {
                                                    ButtonVariant::Contained
                                                } else {
                                                    ButtonVariant::Outlined
                                                }),
                                            );
                                        }
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )(Wh::new(selector_width, f32::MAX.px()), ctx);
                    }),
                    3 => table::fit(table::FitAlign::LeftTop, |ctx| {
                        let selector_width = (self.width - GAP * 2.) / 3.;
                        let mut items: Vec<Option<Rank>> = vec![];

                        // Random button
                        items.push(None);

                        // Rank options
                        for rank in REVERSED_RANKS.iter() {
                            items.push(Some(*rank));
                        }

                        table::vertical(
                            items
                                .into_iter()
                                .map(|rank_opt| {
                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        if let Some(rank) = rank_opt {
                                            let text = format!("{:?}", rank);
                                            ctx.add(
                                                Button::new(
                                                    Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                    &move || {
                                                        set_selected_rank.set(rank);
                                                        set_is_rank_random.set(false);
                                                        set_dropdown.set(0);
                                                    },
                                                    &|wh, text_color, ctx| {
                                                        ctx.add(
                                                            paragraph(text.clone())
                                                                .color(text_color)
                                                                .align(TextAlign::LeftCenter {
                                                                    height: wh.height,
                                                                })
                                                                .build(),
                                                        );
                                                    },
                                                )
                                                .variant(
                                                    if !*is_rank_random && *selected_rank == rank {
                                                        ButtonVariant::Contained
                                                    } else {
                                                        ButtonVariant::Outlined
                                                    },
                                                ),
                                            );
                                        } else {
                                            // Random button
                                            let text = "Random".to_string();
                                            ctx.add(
                                                Button::new(
                                                    Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                    &|| {
                                                        set_is_rank_random.set(true);
                                                        set_dropdown.set(0);
                                                    },
                                                    &|wh, text_color, ctx| {
                                                        ctx.add(
                                                            paragraph(text.clone())
                                                                .color(text_color)
                                                                .align(TextAlign::LeftCenter {
                                                                    height: wh.height,
                                                                })
                                                                .build(),
                                                        );
                                                    },
                                                )
                                                .variant(if *is_rank_random {
                                                    ButtonVariant::Contained
                                                } else {
                                                    ButtonVariant::Outlined
                                                }),
                                            );
                                        }
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )(Wh::new(selector_width, f32::MAX.px()), ctx);
                    }),
                    _ => table::fixed(0.px(), |_, _| {}),
                },
                table::fixed(GAP * 2.0, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &add_card,
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    paragraph("덱에 추가")
                                        .color(text_color)
                                        .align(TextAlign::Center { wh })
                                        .build(),
                                );
                            },
                        )
                        .variant(ButtonVariant::Contained),
                    );
                }),
            ])(Wh::new(self.width, f32::MAX.px()), ctx);
        });
    }
}

impl AddTowerCardTool {}
