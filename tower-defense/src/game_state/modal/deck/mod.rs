mod cards;

use crate::game_state::{set_modal, use_game_state};
use crate::icon::Icon;
use crate::{game_state::modal::deck::cards::Cards, theme::button::Button};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect};

const PADDING: Px = px(24.0);
const SCROLL_BAR_WIDTH: Px = px(8.0);
const CARD_VIEW_WIDTH: Px = px(540.0);
const VERTICAL_MARGIN: Px = px(128.0);
const FAB_SIZE: Px = px(48.0);

#[derive(Debug, Clone, State)]
pub enum DeckKind {
    Deck,
    Draw,
    Discard,
}

pub struct DeckModal {
    pub deck_kind: DeckKind,
}

impl Component for DeckModal {
    fn render(self, ctx: &RenderCtx) {
        let Self { deck_kind } = self;

        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();

        let deck = ctx.track_eq(&game_state.deck);
        let cards = ctx.memo(|| {
            println!("DeckModal: cards memo called");
            deck.record_as_used();
            let cards = match deck_kind {
                DeckKind::Deck => deck.all_cards(),
                DeckKind::Draw => deck.draw_pile(),
                DeckKind::Discard => deck.discard_pile(),
            };
            cards.to_vec()
        });

        ctx.translate((screen_wh.width - PADDING - FAB_SIZE, PADDING))
            .add(
                Button::new(
                    Wh::single(FAB_SIZE),
                    &|| set_modal(None),
                    &|wh, _color, ctx| {
                        ctx.add(Icon {
                            kind: crate::icon::IconKind::Reject,
                            size: crate::icon::IconSize::Custom { size: FAB_SIZE },
                            attributes: vec![],
                            wh,
                            opacity: 1.0,
                        });
                    },
                )
                .variant(crate::theme::button::ButtonVariant::Text),
            );

        ctx.add(AutoScrollViewWithCtx {
            wh: screen_wh,
            scroll_bar_width: SCROLL_BAR_WIDTH,
            content: move |ctx| {
                let card_view_x = (screen_wh.width - CARD_VIEW_WIDTH) * 0.5;
                let card_view = ctx.ghost_add(
                    "cards".to_string(),
                    Cards {
                        width: CARD_VIEW_WIDTH,
                        cards: &cards,
                    },
                );
                let bounding_box = card_view.bounding_box().unwrap_or(Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: CARD_VIEW_WIDTH,
                    height: 0.px(),
                });

                ctx.translate((card_view_x, VERTICAL_MARGIN)).add(card_view);
                ctx.add(simple_rect(
                    Wh::new(
                        screen_wh.width,
                        bounding_box.height() + VERTICAL_MARGIN * 2.0,
                    ),
                    Color::TRANSPARENT,
                    0.px(),
                    Color::TRANSPARENT,
                ));
            },
        })
        .attach_event(|event| match event {
            Event::MouseDown { event } | Event::MouseMove { event } | Event::MouseUp { event }
                if event.is_local_xy_in() =>
            {
                event.stop_propagation();
            }
            Event::Wheel { event } if event.is_local_xy_in() => {
                event.stop_propagation();
            }
            _ => {}
        });

        ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Default))
            .add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::BLACK.with_alpha(180),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        event.stop_propagation();
                    }
                    _ => {}
                }),
            );
    }
}
