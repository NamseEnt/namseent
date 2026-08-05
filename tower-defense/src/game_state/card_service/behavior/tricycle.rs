use super::*;
use crate::{
    card::{CardId, Rank},
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TricycleCardService {
    pub polish_pct: f32,
}

impl TricycleCardService {
    pub fn new(polish_pct: f32) -> Self {
        Self { polish_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Tricycle(self)
    }
}

impl CardServiceBehavior for TricycleCardService {
    fn key(&self) -> &'static str {
        "tricycle"
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
                filter: crate::game_state::modal::deck::CardSelectionFilter::Or(vec![
                    crate::game_state::modal::deck::CardSelectionFilter::Rank(Rank::Ace),
                    crate::game_state::modal::deck::CardSelectionFilter::Rank(Rank::Two),
                    crate::game_state::modal::deck::CardSelectionFilter::Rank(Rank::Three),
                ]),
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
                            changes: vec![DeckEditChange::AddPolishPct(self.polish_pct)],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::TRICYCLE,
            wh,
            stroke_px,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Tricycle",
            crate::l10n::locale::Language::Korean => "세발자전거",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select one Ace, Two, or Three card and give it +200% damage.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("A, 2, 3 카드 중 1장을 선택해 데미지 +200%를 부여합니다.")
            }
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let deck = &game_state.deck;
        let mut cards = deck
            .all_cards()
            .iter()
            .filter(|card| matches!(card.rank, Rank::Ace | Rank::Two | Rank::Three))
            .collect::<Vec<_>>();
        cards.sort_by_key(|c| c.rank.ace_low_value());
        cards.iter().take(1).map(|c| vec![c.id]).collect()
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            crate::l10n::word::Word::Polish(None).tooltip_section(locale),
        ]
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_tricycle_card_service,
        || crate::Rarity::Common,
    );

fn generate_tricycle_card_service() -> CardService {
    TricycleCardService::new(2.0).into_card_service()
}
