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
pub struct BrushCardService {
    pub damage_bonus_pct: f32,
}

impl BrushCardService {
    pub fn new(damage_bonus_pct: f32) -> Self {
        Self { damage_bonus_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Brush(self)
    }
}

impl CardServiceBehavior for BrushCardService {
    fn key(&self) -> &'static str {
        "brush"
    }

    fn acquire(self, _game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![crate::game_state::modal::deck::CardSelectionStep {
                title: "Select cards".to_string(),
                count: 3,
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
                                DeckEditChange::SetSuit(Suit::Hearts),
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
            crate::asset::image::thumbnail::BRUSH,
            wh,
            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Brush",
            crate::l10n::locale::Language::Korean => "붓",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Give 3 face cards +100% damage.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("그림 카드 3장에 데미지 +100% 부여.")
            }
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let deck = &game_state.deck;
        let mut cards = deck.all_cards().to_vec();
        cards.sort_by_key(|c| std::cmp::Reverse((c.rank as i32, c.suit as i32))); // high rank/suit 우선
        cards.iter().take(3).map(|c| vec![c.id]).collect()
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_brush_card_service,
        || crate::Rarity::Common,
    );

fn generate_brush_card_service() -> CardService {
    BrushCardService::new(1.0).into_card_service()
}
