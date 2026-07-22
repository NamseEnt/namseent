use super::*;
use crate::{
    card::CardId,
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FountainPenCardService {
    pub damage_bonus_pct: f32,
}

impl FountainPenCardService {
    pub fn new(damage_bonus_pct: f32) -> Self {
        Self { damage_bonus_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::FountainPen(self)
    }
}

impl CardServiceBehavior for FountainPenCardService {
    fn key(&self) -> &'static str {
        "fountain_pen"
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
                            changes: vec![DeckEditChange::AddDamageBonusPct(self.damage_bonus_pct)],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, _stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::FOUNTAIN_PEN,
            wh,
            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Fountain Pen",
            crate::l10n::locale::Language::Korean => "만년필",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Give 3 number cards +100% damage.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("숫자 카드 3장에 데미지 +100% 부여.")
            }
        };
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_fountain_pen_card_service,
        || crate::Rarity::Common,
    );

fn generate_fountain_pen_card_service() -> CardService {
    FountainPenCardService::new(1.0).into_card_service()
}
