use crate::{card::Card, palette, tower::get_highest_tower};
use namui::*;
use namui_prebuilt::{table, typography};
use std::iter::once;

const HAND_HEIGHT: Px = px(160.);
const CARD_WIDTH: Px = px(120.);
const PADDING: Px = px(4.);

pub struct Hand {
    pub screen_wh: Wh<Px>,
}
impl Component for Hand {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        let (cards, _set_cards) =
            ctx.state(|| (0..5).map(|_| Card::new_random()).collect::<Vec<_>>());

        ctx.compose(|ctx| {
            table::vertical([
                table::ratio(1, |_, _| {}),
                table::fixed(
                    HAND_HEIGHT,
                    table::horizontal(
                        once(table::ratio(1, |_, _| {}))
                            .chain(once(table::fixed(
                                HAND_HEIGHT,
                                table::padding(PADDING, |wh, ctx| {
                                    ctx.add(TowerPreview {
                                        wh,
                                        cards: cards.clone(),
                                    });
                                }),
                            )))
                            .chain(cards.iter().map(|card| {
                                table::fixed(
                                    CARD_WIDTH,
                                    table::padding(PADDING, |wh, ctx| {
                                        ctx.add(RenderCard { card: *card, wh });
                                    }),
                                )
                            }))
                            .chain(once(table::fixed(
                                HAND_HEIGHT,
                                table::padding(PADDING, |wh, ctx| {
                                    ctx.add(InteractionArea { wh });
                                }),
                            )))
                            .chain(once(table::ratio(1, |_, _| {}))),
                    ),
                ),
            ])(screen_wh, ctx);
        });
    }
}

struct RenderCard {
    card: Card,
    wh: Wh<Px>,
}
impl Component for RenderCard {
    fn render(self, ctx: &RenderCtx) {
        let Self { card, wh } = self;

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

        ctx.add(rect(RectParam {
            rect: wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER_HIGH,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}

struct TowerPreview<'a> {
    wh: Wh<Px>,
    cards: Sig<'a, Vec<Card>>,
}
impl Component for TowerPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, cards } = self;

        let tower_blueprint = ctx.memo(|| get_highest_tower(&cards));

        ctx.compose(|ctx| {
            table::padding(
                PADDING,
                table::vertical([
                    table::fixed(typography::title::FONT_SIZE.into_px(), |wh, ctx| {
                        ctx.add(typography::title::left(
                            wh.height,
                            format!("{:?}", tower_blueprint.kind),
                            palette::ON_SURFACE,
                        ));
                    }),
                    table::fixed(typography::body::FONT_SIZE.into_px(), |wh, ctx| {
                        let Some(rank) = tower_blueprint.rank else {
                            return;
                        };
                        ctx.add(typography::body::left(
                            wh.height,
                            format!("{}", rank),
                            palette::ON_SURFACE_VARIANT,
                        ));
                    }),
                    table::fixed(typography::body::FONT_SIZE.into_px(), |wh, ctx| {
                        let Some(suit) = tower_blueprint.suit else {
                            return;
                        };
                        ctx.add(typography::body::left(
                            wh.height,
                            format!("{}", suit),
                            palette::ON_SURFACE_VARIANT,
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

struct InteractionArea {
    wh: Wh<Px>,
}
impl Component for InteractionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

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
