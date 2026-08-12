mod cards;

use crate::card::{Card, CardId, Rank};
use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::{UserModal, mutate_game_state, set_modal, use_game_state};
use crate::icon::IconKind;
use crate::{
    game_state::modal::deck::cards::Cards,
    theme::{
        fab::{FabPosition, FabSide, FabVerticalPosition, FloatingActionButton},
        typography::{FontSize, memoized_text},
    },
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect};
use std::sync::Arc;

type ActionButtonCtx = Option<(IconKind, bool, Arc<dyn Fn() + Send + Sync>)>;

const PADDING: Px = px(36.0);
const SCROLL_BAR_WIDTH: Px = px(8.0);
const CARD_VIEW_WIDTH: Px = px(540.0);
const VERTICAL_MARGIN: Px = px(128.0);

#[derive(Debug, Clone, State)]
pub enum DeckKind {
    Deck,
    Draw,
    Discard,
}

#[derive(Debug, Clone, State, PartialEq)]
pub enum CardSelectionFilter {
    Any,
    Face,
    Number,
    Rank(Rank),
    Engraved,
    NotEngraved,
    And(Vec<CardSelectionFilter>),
    Or(Vec<CardSelectionFilter>),
}

impl CardSelectionFilter {
    pub fn matches(&self, card: &Card) -> bool {
        match self {
            CardSelectionFilter::Any => true,
            CardSelectionFilter::Face => card.rank.is_face(),
            CardSelectionFilter::Number => card.rank.is_number_card(),
            CardSelectionFilter::Rank(rank) => card.rank == *rank,
            CardSelectionFilter::Engraved => card.engraving().is_some(),
            CardSelectionFilter::NotEngraved => card.engraving().is_none(),
            CardSelectionFilter::And(filters) => filters.iter().all(|filter| filter.matches(card)),
            CardSelectionFilter::Or(filters) => filters.iter().any(|filter| filter.matches(card)),
        }
    }
}

#[derive(Debug, Clone, State)]
pub struct CardSelectionStep {
    pub title: String,
    pub count: usize,
    pub filter: CardSelectionFilter,
}

#[derive(Debug, Clone, State)]
pub struct CardSelectionState {
    pub steps: Vec<CardSelectionStep>,
    pub current_step: usize,
    pub selected_card_ids: Vec<Vec<CardId>>,
    pub card_service: crate::game_state::card_service::CardService,
}

impl CardSelectionState {
    pub fn new(
        steps: Vec<CardSelectionStep>,
        card_service: crate::game_state::card_service::CardService,
    ) -> Self {
        let selected_card_ids = steps.iter().map(|_| Vec::new()).collect();
        Self {
            steps,
            current_step: 0,
            selected_card_ids,
            card_service,
        }
    }

    pub fn current_step(&self) -> &CardSelectionStep {
        &self.steps[self.current_step]
    }

    pub fn current_selected_count(&self) -> usize {
        self.selected_card_ids[self.current_step].len()
    }

    pub fn required_count(&self) -> usize {
        self.current_step().count
    }

    pub fn is_card_selected(&self, card_id: CardId) -> bool {
        self.selected_card_ids[self.current_step].contains(&card_id)
    }

    pub fn toggle_card(&mut self, card_id: CardId) {
        let current_step = self.current_step;
        let required_count = self.steps[current_step].count;
        let selected = &mut self.selected_card_ids[current_step];
        if let Some(pos) = selected.iter().position(|&i| i == card_id) {
            selected.remove(pos);
        } else {
            if selected.len() >= required_count {
                selected.remove(0);
            }
            selected.push(card_id);
        }
    }

    pub fn is_step_complete(&self) -> bool {
        self.current_selected_count() == self.required_count()
    }

    pub fn selected_card_ids_by_step(&self) -> Vec<Vec<CardId>> {
        self.selected_card_ids.clone()
    }
}

#[derive(Debug, Clone, State)]
pub struct DeckModal {
    pub deck_kind: DeckKind,
    pub selection: Option<CardSelectionState>,
}

impl Component for DeckModal {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            deck_kind,
            selection,
        } = self;

        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();

        let deck = ctx.track_eq(&game_state.deck);
        let selected_indices = if let Some(selection) = &selection {
            selection.selected_card_ids[selection.current_step].clone()
        } else {
            Vec::new()
        };

        let cards = ctx.memo(|| {
            deck.record_as_used();
            let mut cards = match deck_kind {
                DeckKind::Deck => deck.all_cards().to_vec(),
                DeckKind::Draw => deck.draw_pile().to_vec(),
                DeckKind::Discard => deck.discard_pile().to_vec(),
            };
            if let Some(selection) = &selection {
                let filter = &selection.current_step().filter;
                if *filter != CardSelectionFilter::Any {
                    cards.retain(|card| filter.matches(card));
                }
            }
            cards
        });

        let on_card_click = if selection.is_some() {
            Some(Arc::new(move |card_id: CardId| {
                mutate_game_state(move |gs| {
                    if let Some(UserModal::Deck(deck_modal)) = &mut gs.opened_modals.user
                        && let Some(selection) = &mut deck_modal.selection
                    {
                        selection.toggle_card(card_id);
                    }
                });
            }) as Arc<dyn Fn(CardId) + Send + Sync>)
        } else {
            None
        };

        let mut action_button: ActionButtonCtx = None;
        if let Some(selection) = &selection {
            let step = selection.current_step();

            ctx.translate((PADDING, PADDING)).add(memoized_text(
                (
                    &step.title,
                    &selection.current_selected_count(),
                    &step.count,
                ),
                |mut builder| {
                    let progress_text = format!(
                        "{} ({}/{})",
                        step.title,
                        selection.current_selected_count(),
                        step.count
                    );
                    builder
                        .headline()
                        .bold()
                        .color(Color::WHITE)
                        .stroke(2.px(), Color::BLACK)
                        .size(FontSize::Large)
                        .text(progress_text)
                        .render_left_top()
                },
            ));

            let icon = if selection.current_step + 1 >= selection.steps.len() {
                IconKind::Accept
            } else {
                IconKind::Play
            };
            let card_service = selection.card_service.clone();
            action_button = Some((
                icon,
                !selection.is_step_complete(),
                Arc::new(move || {
                    let card_service = card_service.clone();
                    mutate_game_state(move |gs| {
                        if let Some(UserModal::Deck(deck_modal)) = &mut gs.opened_modals.user
                            && let Some(selection) = &mut deck_modal.selection
                        {
                            if !selection.is_step_complete() {
                                return;
                            }

                            if selection.current_step + 1 >= selection.steps.len() {
                                let selected_card_ids = selection.selected_card_ids_by_step();
                                deck_modal.selection = None;
                                gs.opened_modals.user = None;
                                card_service.select_cards(gs, selected_card_ids);
                            } else {
                                selection.current_step += 1;
                            }
                        }
                    });
                }) as Arc<dyn Fn() + Send + Sync>,
            ));
        }

        let close = || set_modal(None);
        ctx.compose(|ctx| {
            if selection.is_none() {
                ctx.add(FloatingActionButton {
                    screen_wh,
                    position: FabPosition::new(FabSide::Right, FabVerticalPosition::Top),
                    visible: true,
                    icon: IconKind::Reject,
                    disabled: false,
                    on_click: &close,
                    tooltip_content: None,
                });
            }
        });

        ctx.compose(|ctx| {
            if let Some((icon, disable, action)) = action_button {
                ctx.add(FloatingActionButton {
                    screen_wh,
                    position: FabPosition::new(FabSide::Right, FabVerticalPosition::Center),
                    visible: true,
                    icon,
                    disabled: disable,
                    on_click: &move || action(),
                    tooltip_content: None,
                });
            }
        });

        ctx.add(AutoScrollViewWithCtx {
            wh: screen_wh,
            scroll_bar_width: SCROLL_BAR_WIDTH,
            content: move |ctx| {
                let card_view_x = (screen_wh.width - CARD_VIEW_WIDTH) * 0.5;
                let card_view = ctx.translate((card_view_x, VERTICAL_MARGIN)).ghost_add(
                    "cards".to_string(),
                    Cards {
                        width: CARD_VIEW_WIDTH,
                        cards: &cards,
                        selected_card_ids: &selected_indices,
                        on_card_click: on_card_click.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Engraving, Suit};

    #[test]
    fn engraved_filters_split_the_deck_by_engraving() {
        let blank = Card::new(Rank::Ace, Suit::Spades);
        let mut engraved = Card::new(Rank::Ace, Suit::Hearts);
        engraved.effects.engraving = Some(Engraving::Magnet);

        assert!(CardSelectionFilter::NotEngraved.matches(&blank));
        assert!(!CardSelectionFilter::NotEngraved.matches(&engraved));

        assert!(!CardSelectionFilter::Engraved.matches(&blank));
        assert!(CardSelectionFilter::Engraved.matches(&engraved));
    }

    #[test]
    fn not_engraved_composes_with_other_filters() {
        let mut engraved_face = Card::new(Rank::King, Suit::Spades);
        engraved_face.effects.engraving = Some(Engraving::Magnet);
        let blank_face = Card::new(Rank::King, Suit::Hearts);
        let blank_number = Card::new(Rank::Two, Suit::Hearts);

        let filter = CardSelectionFilter::And(vec![
            CardSelectionFilter::Face,
            CardSelectionFilter::NotEngraved,
        ]);

        assert!(filter.matches(&blank_face));
        assert!(!filter.matches(&engraved_face));
        assert!(!filter.matches(&blank_number));
    }
}
