use super::*;
use crate::{
    Rarity,
    game_state::{
        DeckKind, DeckModal, UserModal, card_service::definition::CardServiceDefinition, set_modal,
    },
    theme::typography::TypographyBuilder,
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CardServiceNoop;

impl CardServiceNoop {
    pub fn standard() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Noop(self)
    }
}

impl CardServiceBehavior for CardServiceNoop {
    fn key(&self) -> &'static str {
        "card_service_noop"
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.text("No Op");
        let _ = locale;
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.text("No effect placeholder.");
        let _ = locale;
    }

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![crate::game_state::modal::deck::CardSelectionStep {
                title: "Select a card".to_string(),
                count: 1,
            }],
            self.into_card_service(),
        );

        set_modal(Some(UserModal::Deck(DeckModal {
            deck_kind: DeckKind::Deck,
            selection: Some(selection),
        })));
    }

    fn thumbnail(&self, _wh: Wh<Px>, _stroke_px: Px, _shadow: bool) -> RenderingTree {
        RenderingTree::Empty
    }

    fn select_cards(
        self,
        game_state: &mut GameState,
        selected_card_ids: Vec<Vec<crate::card::CardId>>,
    ) where
        Self: Sized + Into<CardService>,
    {
        todo!()
    }
}

pub(super) const DEFINITION: CardServiceDefinition =
    CardServiceDefinition::new(|| CardService::Noop(CardServiceNoop), || Rarity::Common);
