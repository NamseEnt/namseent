use super::*;
use crate::{
    card::{CardId, Suit},
    game_state::{GameState, set_modal},
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MaceCardService {
    pub damage_bonus_pct: f32,
}

impl MaceCardService {
    pub fn new(damage_bonus_pct: f32) -> Self {
        Self { damage_bonus_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Mace(self)
    }
}

impl CardServiceBehavior for MaceCardService {
    fn key(&self) -> &'static str {
        "mace"
    }

    fn acquire(self, _game_state: &mut GameState)
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
            for card_id in card_ids {
                game_state.deck.apply_to_card(card_id, |card| {
                    card.suit = Suit::Hearts;
                    card.add_damage_bonus_pct(self.damage_bonus_pct);
                });
            }
        }
    }

    fn thumbnail(&self, _wh: Wh<Px>, _stroke_px: Px, _shadow: bool) -> RenderingTree {
        RenderingTree::Empty
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Mace",
            crate::l10n::locale::Language::Korean => "메이스",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Give a card the Heart suit and damage +100%.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드에 하트 효과를 부여하고 데미지 +100%.")
            }
        };
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_mace_card_service,
        || crate::Rarity::Common,
    );

fn generate_mace_card_service() -> CardService {
    MaceCardService::new(1.0).into_card_service()
}
