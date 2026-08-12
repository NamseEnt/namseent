use super::*;
use crate::{
    card::CardId,
    game_state::{GameState, action::DeckEdit, set_modal},
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserCardService;

impl EraserCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Eraser(self)
    }
}

impl CardServiceBehavior for EraserCardService {
    fn key(&self) -> &'static str {
        "eraser"
    }

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let title = match game_state.locale.language {
            crate::l10n::locale::Language::English => "Select a card to remove",
            crate::l10n::locale::Language::Korean => "제거할 카드를 선택하세요",
        }
        .to_string();

        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![crate::game_state::modal::deck::CardSelectionStep {
                title,
                count: 1,
                filter: crate::game_state::modal::deck::CardSelectionFilter::Any,
            }],
            self.into_card_service(),
        );

        set_modal(Some(crate::game_state::modal::UserModal::Deck(
            crate::game_state::modal::deck::DeckModal {
                deck_kind: crate::game_state::modal::deck::DeckKind::Deck,
                selection: Some(selection),
            },
        )));
    }

    fn select_cards(self, game_state: &mut GameState, selected_card_ids: Vec<Vec<CardId>>)
    where
        Self: Sized + Into<CardService>,
    {
        for card_ids in selected_card_ids {
            game_state.action(crate::game_state::GameStateAction::ModifyDeck(
                DeckEdit::Remove { card_ids },
            ));
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::ERASER)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Eraser",
            crate::l10n::locale::Language::Korean => "지우개",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select 1 card and remove it from the deck.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드를 1장 선택해 덱에서 제거합니다.")
            }
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        // Eraser: remove lowest rank card (least valuable).
        let deck = &game_state.deck;
        let mut cards = deck.all_cards().to_vec();
        cards.sort_by_key(|c| c.rank as u8); // lowest first
        cards.iter().take(1).map(|c| vec![c.id]).collect()
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_eraser_card_service,
        || crate::Rarity::Rare,
    );

fn generate_eraser_card_service() -> CardService {
    EraserCardService::new().into_card_service()
}
