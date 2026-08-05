use super::*;
use crate::{
    card::{CardId, Engraving},
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

const ENGRAVE_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MagnetCardService;

impl MagnetCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Magnet(self)
    }
}

impl CardServiceBehavior for MagnetCardService {
    fn key(&self) -> &'static str {
        "magnet"
    }

    fn purchase_block_reasons(
        &self,
        context: &CardServicePurchaseContext,
    ) -> Vec<CardServicePurchaseBlockReason> {
        let available = context.unengraved_card_count;
        if available < 2 {
            vec![CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                required: 2,
                available,
            }]
        } else {
            Vec::new()
        }
    }

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let title = match game_state.locale.language {
            crate::l10n::locale::Language::English => "Select 2 cards to engrave",
            crate::l10n::locale::Language::Korean => "각인할 카드 2장을 선택하세요",
        }
        .to_string();

        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![crate::game_state::modal::deck::CardSelectionStep {
                title,
                count: ENGRAVE_COUNT,
                filter: crate::game_state::modal::deck::CardSelectionFilter::NotEngraved,
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
                            changes: vec![DeckEditChange::SetEngraving(Some(Engraving::Magnet))],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, _stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::MAGNET,
            wh,
            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Magnet",
            crate::l10n::locale::Language::Korean => "자석",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Engraves a magnet on 2 cards.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드 2장에 자석을 각인합니다.")
            }
        };
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            crate::l10n::word::Word::Engraving(Some(Engraving::Magnet)).tooltip_section(locale),
        ]
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let mut candidates: Vec<_> = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.engraving().is_none())
            .collect();
        candidates.sort_by(|a, b| {
            b.polish_pct()
                .total_cmp(&a.polish_pct())
                .then_with(|| b.rank.ordinal().cmp(&a.rank.ordinal()))
        });

        vec![
            candidates
                .iter()
                .take(ENGRAVE_COUNT)
                .map(|card| card.id)
                .collect(),
        ]
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_magnet_card_service,
        || crate::Rarity::Common,
    );

fn generate_magnet_card_service() -> CardService {
    MagnetCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn magnet_heuristic_selects_two_cards() {
        let game_state = crate::game_state::create_initial_game_state();

        let selected = MagnetCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].len(), ENGRAVE_COUNT);
        for card_id in &selected[0] {
            assert_eq!(game_state.deck.get_card(*card_id).unwrap().rank, Rank::Ace);
        }
    }

    #[test]
    fn magnet_heuristic_prefers_the_most_polished_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let polished: Vec<_> = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.rank == Rank::Two)
            .take(ENGRAVE_COUNT)
            .map(|card| card.id)
            .collect();
        for card_id in &polished {
            game_state.deck.modify_card(*card_id, |card| {
                card.add_polish_pct(1.0);
            });
        }

        let selected = MagnetCardService.heuristic_best_selection(&game_state);

        for card_id in &polished {
            assert!(selected[0].contains(card_id));
        }
    }

    #[test]
    fn magnet_heuristic_skips_already_engraved_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let engraved: Vec<_> = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.rank == Rank::Ace)
            .map(|card| card.id)
            .collect();
        for card_id in &engraved {
            game_state.deck.modify_card(*card_id, |card| {
                card.effects.engraving = Some(Engraving::Magnet);
            });
        }

        let selected = MagnetCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected[0].len(), ENGRAVE_COUNT);
        for card_id in &selected[0] {
            assert!(!engraved.contains(card_id));
            assert_eq!(game_state.deck.get_card(*card_id).unwrap().rank, Rank::King);
        }
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn magnet_headless_use_card_service_engraves_the_selected_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = MagnetCardService;
        let selected = service.heuristic_best_selection(&game_state)[0].clone();
        game_state.headless = true;

        game_state.action(crate::game_state::GameStateAction::UseCardService(
            service.into_card_service(),
        ));

        for card_id in &selected {
            assert_eq!(
                game_state.deck.get_card(*card_id).unwrap().engraving(),
                Some(Engraving::Magnet)
            );
        }
        assert_eq!(
            game_state
                .deck
                .all_cards()
                .iter()
                .filter(|card| card.engraving() == Some(Engraving::Magnet))
                .count(),
            ENGRAVE_COUNT
        );
    }
}
