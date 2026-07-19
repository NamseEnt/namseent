use crate::card::{Card, CardId, RenderCard};
use namui::*;
use namui_prebuilt::simple_rect;
use std::sync::Arc;

const CARD_WIDTH: Px = px(120.0);
const CARD_HEIGHT: Px = px(162.0);
const COLUMN_COUNT: usize = 4;

pub(super) struct Cards<'a> {
    pub width: Px,
    pub cards: &'a [Card],
    pub selected_card_ids: &'a [CardId],
    pub on_card_click: Option<Arc<dyn Fn(CardId) + Send + Sync>>,
}

impl Component for Cards<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            width,
            cards,
            selected_card_ids,
            on_card_click,
        } = self;
        let card_gap = (width - (CARD_WIDTH * COLUMN_COUNT as f32)) / (COLUMN_COUNT as f32 - 1.0);

        ctx.compose(|mut ctx| {
            for row in cards.chunks(COLUMN_COUNT) {
                ctx.compose(|mut ctx| {
                    for card in row.iter() {
                        let selected = selected_card_ids.contains(&card.id);
                        let card_wh = Wh::new(CARD_WIDTH, CARD_HEIGHT);
                        let on_card_click = on_card_click.clone();
                        ctx.add(
                            simple_rect(card_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                                .attach_event(move |event| match event {
                                    Event::MouseDown { event } if event.is_local_xy_in() => {
                                        event.stop_propagation();
                                        let Some(on_card_click) = on_card_click else {
                                            return;
                                        };
                                        on_card_click(card.id);
                                    }
                                    _ => {}
                                }),
                        );

                        ctx.add(RenderCard {
                            wh: card_wh,
                            card,
                            selected,
                        });
                        ctx = ctx.translate((CARD_WIDTH + card_gap, 0.px()));
                    }
                });
                ctx = ctx.translate((0.px(), card_gap + CARD_HEIGHT));
            }
        });
    }
}
