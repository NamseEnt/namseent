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

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let title = match game_state.locale.language {
            crate::l10n::locale::Language::English => "Select a card",
            crate::l10n::locale::Language::Korean => "카드를 선택하세요",
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
            crate::l10n::locale::Language::English => builder
                .static_text("Select one card, change it to the Club suit, and deal +200% damage."),
            crate::l10n::locale::Language::Korean => builder
                .static_text("카드 1장을 선택해 클럽으로 변경하고 데미지 +200%를 부여합니다."),
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        // ClubSword: remove low rank cards (low impact on poker hands).
        let deck = &game_state.deck;
        let mut cards = deck.all_cards().to_vec();
        cards.sort_by_key(|c| c.rank as u8); // lowest first
        cards.iter().take(3).map(|c| vec![c.id]).collect()
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_club_sword_card_service,
        || crate::Rarity::Common,
    );

fn generate_club_sword_card_service() -> CardService {
    ClubSwordCardService::new(2.0).into_card_service()
}
