use super::*;
use crate::{
    card::{CardId, Suit},
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ClubSwordCardService {
    pub damage_bonus_pct: f32,
}

impl ClubSwordCardService {
    pub fn new(damage_bonus_pct: f32) -> Self {
        Self { damage_bonus_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::ClubSword(self)
    }
}

impl CardServiceBehavior for ClubSwordCardService {
    fn key(&self) -> &'static str {
        "club_sword"
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
            game_state.action(crate::game_state::GameStateAction::ModifyDeck(
                DeckEdit::Enhance {
                    enhances: card_ids
                        .into_iter()
                        .map(|card_id| DeckEnhance {
                            card_id,
                            changes: vec![
                                DeckEditChange::SetSuit(Suit::Clubs),
                                DeckEditChange::AddDamageBonusPct(self.damage_bonus_pct),
                            ],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, _stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::CLUB_SWORD,
            wh,
            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Club",
            crate::l10n::locale::Language::Korean => "클럽",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Give a card the Club suit and damage +100%.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드에 클럽 효과를 부여하고 데미지 +100%.")
            }
        };
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_club_sword_card_service,
        || crate::Rarity::Common,
    );

fn generate_club_sword_card_service() -> CardService {
    ClubSwordCardService::new(1.0).into_card_service()
}
