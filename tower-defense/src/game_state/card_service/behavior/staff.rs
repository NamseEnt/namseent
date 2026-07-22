use super::*;
use crate::{
    card::{CardId, Suit},
    game_state::{GameState, set_modal},
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct StaffCardService {
    pub damage_bonus_pct: f32,
}

impl StaffCardService {
    pub fn new(damage_bonus_pct: f32) -> Self {
        Self { damage_bonus_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Staff(self)
    }
}

impl CardServiceBehavior for StaffCardService {
    fn key(&self) -> &'static str {
        "staff"
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
                    card.suit = Suit::Diamonds;
                    card.add_damage_bonus_pct(self.damage_bonus_pct);
                });
            }
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, _stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::STAFF,
            wh,
            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Staff",
            crate::l10n::locale::Language::Korean => "지팡이",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Give a card the Diamond suit and damage +100%.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드에 다이아 효과를 부여하고 데미지 +100%.")
            }
        };
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_staff_card_service,
        || crate::Rarity::Common,
    );

fn generate_staff_card_service() -> CardService {
    StaffCardService::new(1.0).into_card_service()
}
