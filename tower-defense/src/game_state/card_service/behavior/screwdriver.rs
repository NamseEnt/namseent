use super::*;
use crate::{
    card::{CardId, RANKS, Rank},
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ScrewdriverCardService;

impl ScrewdriverCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Screwdriver(self)
    }
}

impl CardServiceBehavior for ScrewdriverCardService {
    fn key(&self) -> &'static str {
        "screwdriver"
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
                            changes: vec![DeckEditChange::SetRank(next_rank(
                                game_state
                                    .deck
                                    .get_card(card_id)
                                    .map(|card| card.rank)
                                    .unwrap_or(Rank::Ace),
                            ))],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::SCREWDRIVER)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Screwdriver",
            crate::l10n::locale::Language::Korean => "드라이버",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select one card and increase its rank by 1.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드 1장을 선택해 랭크를 1 올립니다.")
            }
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let card_id = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.rank != Rank::Ace)
            .max_by_key(|card| card.rank.ordinal())
            .map(|card| card.id)
            .into_iter()
            .collect();
        vec![card_id]
    }
}

fn next_rank(rank: Rank) -> Rank {
    RANKS[(rank.ordinal() + 1) % RANKS.len()]
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_screwdriver_card_service,
        || crate::Rarity::Common,
    );

fn generate_screwdriver_card_service() -> CardService {
    ScrewdriverCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn screwdriver_increases_ranks_in_order() {
        assert_eq!(next_rank(Rank::Queen), Rank::King);
        assert_eq!(next_rank(Rank::King), Rank::Ace);
        assert_eq!(next_rank(Rank::Ace), Rank::Two);
    }

    #[test]
    fn screwdriver_heuristic_selects_the_highest_non_ace_card() {
        let game_state = crate::game_state::create_initial_game_state();
        let selected_card_id = ScrewdriverCardService.heuristic_best_selection(&game_state)[0][0];

        assert_eq!(
            game_state.deck.get_card(selected_card_id).unwrap().rank,
            Rank::King
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn screwdriver_headless_use_card_service_increases_the_selected_card_rank() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = ScrewdriverCardService;
        let selected_card_id = service.heuristic_best_selection(&game_state)[0][0];
        game_state.headless = true;

        game_state.action(crate::game_state::GameStateAction::UseCardService(
            service.into_card_service(),
        ));

        assert_eq!(
            game_state.deck.get_card(selected_card_id).unwrap().rank,
            Rank::Ace
        );
    }
}
