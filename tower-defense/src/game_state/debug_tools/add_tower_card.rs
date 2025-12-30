use crate::card::{REVERSED_RANKS, Rank, SUITS, Suit};
use crate::game_state::tower::{TowerKind, TowerTemplate};
use crate::game_state::{flow::GameFlow, mutate_game_state};
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::button::{Button, ButtonVariant};
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

pub struct AddTowerCardTool {
    pub width: Px,
}

impl Component for AddTowerCardTool {
    fn render(self, ctx: &RenderCtx) {
        let (selected_kind, set_selected_kind) = ctx.state(|| TowerKind::High);
        let (selected_suit, set_selected_suit) = ctx.state(|| Suit::Spades);
        let (selected_rank, set_selected_rank) = ctx.state(|| Rank::Ace);
        // 0 = none, 1 = Kind, 2 = Suit, 3 = Rank
        let (dropdown, set_dropdown) = ctx.state(|| 0u8);

        let add_card = || {
            let kind = *selected_kind;
            let suit = *selected_suit;
            let rank = *selected_rank;
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
                            let text = format!("{:?}", *selected_suit);
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
                            let text = format!("{:?}", *selected_rank);
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
                        table::vertical(
                            SUITS
                                .iter()
                                .map(|suit| {
                                    let suit = *suit;

                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        let text = format!("{:?}", suit);
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_suit.set(suit);
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
                                                if *selected_suit == suit {
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
                    3 => table::fit(table::FitAlign::LeftTop, |ctx| {
                        let selector_width = (self.width - GAP * 2.) / 3.;
                        table::vertical(
                            REVERSED_RANKS
                                .iter()
                                .map(|rank| {
                                    let rank = *rank;

                                    table::fit(table::FitAlign::LeftTop, move |ctx| {
                                        let text = format!("{:?}", rank);
                                        ctx.add(
                                            Button::new(
                                                Wh::new(selector_width, DROPDOWN_ITEM_HEIGHT),
                                                &move || {
                                                    set_selected_rank.set(rank);
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
                                                if *selected_rank == rank {
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
