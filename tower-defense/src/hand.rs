use crate::{card::Card, palette};
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
                            .chain(cards.iter().map(|card| {
                                table::fixed(
                                    CARD_WIDTH,
                                    table::padding(PADDING, |wh, ctx| {
                                        ctx.add(RenderCard { card: *card, wh });
                                    }),
                                )
                            }))
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
