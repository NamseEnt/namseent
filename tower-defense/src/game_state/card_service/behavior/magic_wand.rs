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
pub struct MagicWandCardService;

impl MagicWandCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::MagicWand(self)
    }
}

impl CardServiceBehavior for MagicWandCardService {
    fn key(&self) -> &'static str {
        "magic_wand"
    }

    fn purchase_block_reasons(
        &self,
        context: &CardServicePurchaseContext,
    ) -> Vec<CardServicePurchaseBlockReason> {
        let mut reasons = Vec::new();
        if context.engraved_card_count == 0 {
            reasons.push(CardServicePurchaseBlockReason::NoEngravedCard);
        }
        let available = context.unengraved_card_count;
        if available < 1 {
            reasons.push(CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                required: 1,
                available,
            });
        }
        reasons
    }

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let purchase_context = CardServicePurchaseContext::from_game_state(game_state);
        if !self.purchase_block_reasons(&purchase_context).is_empty() {
            return;
        }

        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![
                crate::game_state::modal::deck::CardSelectionStep {
                    title: match game_state.locale.language {
                        crate::l10n::locale::Language::English => {
                            "Select an engraved card whose engraving will be moved"
                        }
                        crate::l10n::locale::Language::Korean => {
                            "옮길 각인이 있는 카드를 선택하세요"
                        }
                    }
                    .to_string(),
                    count: 1,
                    filter: crate::game_state::modal::deck::CardSelectionFilter::Engraved,
                },
                crate::game_state::modal::deck::CardSelectionStep {
                    title: match game_state.locale.language {
                        crate::l10n::locale::Language::English => {
                            "Select an unengraved card to receive the engraving"
                        }
                        crate::l10n::locale::Language::Korean => "각인을 받을 카드를 선택하세요",
                    }
                    .to_string(),
                    count: 1,
                    filter: crate::game_state::modal::deck::CardSelectionFilter::NotEngraved,
                },
            ],
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
        let Some(source_card_id) = selected_card_ids
            .first()
            .and_then(|card_ids| card_ids.first())
            .copied()
        else {
            return;
        };
        let Some(target_card_id) = selected_card_ids
            .get(1)
            .and_then(|card_ids| card_ids.first())
            .copied()
        else {
            return;
        };
        if source_card_id == target_card_id {
            return;
        }
        let Some(engraving) = game_state
            .deck
            .get_card(source_card_id)
            .and_then(|card| card.engraving())
        else {
            return;
        };
        let Some(target_card) = game_state.deck.get_card(target_card_id) else {
            return;
        };
        if target_card.engraving().is_some() {
            return;
        }

        game_state.action(crate::game_state::GameStateAction::ModifyDeck(
            DeckEdit::Enhance {
                enhances: vec![
                    DeckEnhance {
                        card_id: source_card_id,
                        changes: vec![DeckEditChange::SetEngraving(None)],
                    },
                    DeckEnhance {
                        card_id: target_card_id,
                        changes: vec![DeckEditChange::SetEngraving(Some(engraving))],
                    },
                ],
            },
        ));
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::MAGIC_WAND)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Magic Wand",
            crate::l10n::locale::Language::Korean => "마법 지팡이",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder.static_text(
                "Select an engraved card and an unengraved card to move the engraving.",
            ),
            crate::l10n::locale::Language::Korean => builder
                .static_text("각인된 카드 1장과 각인되지 않은 카드 1장을 선택해 각인을 옮깁니다."),
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let source_card_id = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.engraving().is_some())
            .min_by(|a, b| {
                a.polish_pct()
                    .total_cmp(&b.polish_pct())
                    .then_with(|| a.rank.ordinal().cmp(&b.rank.ordinal()))
            })
            .map(|card| card.id);
        let target_card_id = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.engraving().is_none())
            .max_by(|a, b| {
                a.polish_pct()
                    .total_cmp(&b.polish_pct())
                    .then_with(|| a.rank.ordinal().cmp(&b.rank.ordinal()))
            })
            .map(|card| card.id);

        match (source_card_id, target_card_id) {
            (Some(source), Some(target)) => vec![vec![source], vec![target]],
            _ => vec![vec![], vec![]],
        }
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_magic_wand_card_service,
        || crate::Rarity::Common,
    );

fn generate_magic_wand_card_service() -> CardService {
    MagicWandCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Engraving;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn heuristic_returns_source_and_target_selection_steps() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let source = game_state.deck.all_cards()[0].id;
        game_state.deck.modify_card(source, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
        });

        let selected = MagicWandCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].len(), 1);
        assert_eq!(selected[1].len(), 1);
        assert_eq!(
            game_state
                .deck
                .get_card(selected[0][0])
                .unwrap()
                .engraving(),
            Some(Engraving::Cactus)
        );
        assert_eq!(
            game_state
                .deck
                .get_card(selected[1][0])
                .unwrap()
                .engraving(),
            None
        );
    }

    #[test]
    fn heuristic_handles_missing_source_or_target_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        assert_eq!(
            MagicWandCardService.heuristic_best_selection(&game_state),
            vec![vec![], vec![]]
        );

        for card in game_state.deck.all_cards().to_vec() {
            game_state.deck.modify_card(card.id, |card| {
                card.effects.engraving = Some(Engraving::Magnet);
            });
        }
        assert_eq!(
            MagicWandCardService.heuristic_best_selection(&game_state),
            vec![vec![], vec![]]
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn selection_moves_the_selected_engraving_and_preserves_unrelated_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let source = game_state.deck.all_cards()[0].id;
        let target = game_state.deck.all_cards()[1].id;
        let unrelated = game_state.deck.all_cards()[2].id;
        game_state.deck.modify_card(source, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
            card.add_polish_pct(1.0);
        });
        let source_polish = game_state.deck.get_card(source).unwrap().polish_pct();

        MagicWandCardService.select_cards(&mut game_state, vec![vec![source], vec![target]]);

        assert_eq!(game_state.deck.get_card(source).unwrap().engraving(), None);
        assert_eq!(
            game_state.deck.get_card(target).unwrap().engraving(),
            Some(Engraving::Cactus)
        );
        assert_eq!(
            game_state.deck.get_card(source).unwrap().polish_pct(),
            source_polish
        );
        assert_eq!(
            game_state.deck.get_card(unrelated).unwrap().engraving(),
            None
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn headless_use_card_service_moves_the_engraving() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let source = game_state.deck.all_cards()[0].id;
        game_state.deck.modify_card(source, |card| {
            card.effects.engraving = Some(Engraving::Overcharge);
        });
        let selected = MagicWandCardService.heuristic_best_selection(&game_state);
        let target = selected[1][0];
        game_state.headless = true;

        game_state.action(crate::game_state::GameStateAction::UseCardService(
            MagicWandCardService.into_card_service(),
        ));

        assert_eq!(game_state.deck.get_card(source).unwrap().engraving(), None);
        assert_eq!(
            game_state.deck.get_card(target).unwrap().engraving(),
            Some(Engraving::Overcharge)
        );
    }
}
