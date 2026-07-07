use crate::card::{Card, RenderCard};
use namui::*;

const CARD_WIDTH: Px = px(120.0);
const CARD_HEIGHT: Px = px(162.0);
const COLUMN_COUNT: usize = 4;

pub(super) struct Cards<'a> {
    pub width: Px,
    pub cards: &'a [Card],
}

impl Component for Cards<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { width, cards } = self;
        let card_gap = (width - (CARD_WIDTH * COLUMN_COUNT as f32)) / (COLUMN_COUNT as f32 - 1.0);

        ctx.compose(|mut ctx| {
            for row in cards.chunks(COLUMN_COUNT) {
                ctx.compose(|mut ctx| {
                    for card in row {
                        ctx.add(RenderCard {
                            wh: Wh::new(CARD_WIDTH, CARD_HEIGHT),
                            card,
                        });
                        ctx = ctx.translate((CARD_WIDTH + card_gap, 0.px()));
                    }
                });
                ctx = ctx.translate((0.px(), card_gap + CARD_HEIGHT));
            }
        });
    }
}
