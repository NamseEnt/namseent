use crate::{
    card::{CardId, RenderCard},
    game_state::presentation_deck::PresentationDeckCard,
};
use namui::*;
use namui_prebuilt::simple_rect;
use std::sync::Arc;

const CARD_WIDTH: Px = px(120.0);
const CARD_HEIGHT: Px = px(162.0);
const COLUMN_COUNT: usize = 4;

pub(super) struct Cards<'a> {
    pub width: Px,
    pub cards: &'a [PresentationDeckCard],
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

        ctx.compose(|ctx| {
            for (row_index, row) in cards.chunks(COLUMN_COUNT).enumerate() {
                ctx.compose(|ctx| {
                    for (column_index, entry) in row.iter().enumerate() {
                        let card_id = entry.card.id;
                        let selected = selected_card_ids.contains(&card_id);
                        let card_wh = Wh::new(CARD_WIDTH, CARD_HEIGHT);
                        let on_card_click = on_card_click.clone();
                        let has_click_handler = on_card_click.is_some() && !entry.is_exiting();
                        ctx.add_with_key(
                            card_id,
                            DeckCardView {
                                card: &entry.card,
                                card_wh,
                                selected,
                                exiting: entry.is_exiting(),
                                target_xy: Xy::new(
                                    (CARD_WIDTH + card_gap) * column_index as f32,
                                    (CARD_HEIGHT + card_gap) * row_index as f32,
                                ),
                                on_card_click,
                                interactive: has_click_handler,
                            },
                        );
                    }
                });
            }
        });
    }
}

struct DeckCardView<'a> {
    card: &'a crate::card::Card,
    card_wh: Wh<Px>,
    selected: bool,
    exiting: bool,
    target_xy: Xy<Px>,
    on_card_click: Option<Arc<dyn Fn(CardId) + Send + Sync>>,
    interactive: bool,
}

impl Component for DeckCardView<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            card,
            card_wh,
            selected,
            exiting,
            target_xy,
            on_card_click,
            interactive,
        } = self;
        let animation_target = if exiting {
            Xy::new(target_xy.x, target_xy.y + CARD_HEIGHT)
        } else {
            target_xy
        };
        let animated_xy = crate::hand::xy_with_spring(
            ctx,
            animation_target,
            Xy::new(target_xy.x, target_xy.y + CARD_HEIGHT),
        );
        let animated_scale = crate::hand::xy_with_spring(
            ctx,
            if exiting {
                Xy::single(0.0)
            } else {
                Xy::single(1.0)
            },
            Xy::single(0.0),
        );
        let half_card = card_wh.to_xy() * 0.5;
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_card)
            .scale(animated_scale)
            .translate(-half_card);

        if interactive {
            let card_id = card.id;
            let on_card_click = on_card_click.expect("interactive card has a handler");
            ctx.add(
                simple_rect(card_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(
                    move |event| match event {
                        Event::MouseDown { event } if event.is_local_xy_in() => {
                            event.stop_propagation();
                            on_card_click(card_id);
                        }
                        _ => {}
                    },
                ),
            );
        }
        ctx.mouse_cursor(MouseCursor::Standard(match interactive {
            true => StandardCursor::Pointer,
            false => StandardCursor::Default,
        }))
        .add(RenderCard {
            wh: card_wh,
            card,
            selected,
            opacity: 1.0,
        });
    }
}
