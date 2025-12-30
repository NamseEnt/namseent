use crate::card::{REVERSED_RANKS, Rank, SUITS, Suit};
use crate::game_state::tower::{TowerKind, TowerTemplate};
use crate::game_state::{flow::GameFlow, mutate_game_state};
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::{TextAlign, headline, paragraph};
use namui::*;
use namui_prebuilt::table;

const BUTTON_HEIGHT: Px = px(36.);
const GAP: Px = px(8.);

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

const RANKS: [Rank; 8] = REVERSED_RANKS;

pub struct AddTowerCardTool {
    pub width: Px,
}

impl Component for AddTowerCardTool {
    fn render(self, ctx: &RenderCtx) {
        let (selected_kind, set_selected_kind) = ctx.state(|| TowerKind::High);
        let (selected_suit, set_selected_suit) = ctx.state(|| Suit::Spades);
        let (selected_rank, set_selected_rank) = ctx.state(|| Rank::Ace);

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
                    let text = format!("족보: {:?}", *selected_kind);
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &|| {
                                set_selected_kind
                                    .set(next_value(&TOWER_KIND_ORDER, *selected_kind));
                            },
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    paragraph(text.clone())
                                        .color(text_color)
                                        .align(TextAlign::Center { wh })
                                        .build(),
                                );
                            },
                        )
                        .variant(ButtonVariant::Outlined),
                    );
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    let text = format!("문양: {:?}", *selected_suit);
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &|| {
                                set_selected_suit.set(next_value(&SUITS, *selected_suit));
                            },
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    paragraph(text.clone())
                                        .color(text_color)
                                        .align(TextAlign::Center { wh })
                                        .build(),
                                );
                            },
                        )
                        .variant(ButtonVariant::Outlined),
                    );
                }),
                table::fixed(GAP, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    let text = format!("랭크: {:?}", *selected_rank);
                    ctx.add(
                        Button::new(
                            Wh::new(self.width, BUTTON_HEIGHT),
                            &|| {
                                set_selected_rank.set(next_value(&RANKS, *selected_rank));
                            },
                            &|wh, text_color, ctx| {
                                ctx.add(
                                    paragraph(text.clone())
                                        .color(text_color)
                                        .align(TextAlign::Center { wh })
                                        .build(),
                                );
                            },
                        )
                        .variant(ButtonVariant::Outlined),
                    );
                }),
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

fn next_value<T: PartialEq + Copy>(values: &[T], current: T) -> T {
    let current_index = values
        .iter()
        .position(|value| *value == current)
        .unwrap_or(0);
    let next_index = (current_index + 1) % values.len();
    values[next_index]
}

impl AddTowerCardTool {}
