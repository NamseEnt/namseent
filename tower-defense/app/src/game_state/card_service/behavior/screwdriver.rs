use super::*;
#[cfg(test)]
use crate::{card::Rank, game_state::GameState};

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

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep> {
        let title = match locale.language {
            crate::l10n::locale::Language::English => "Select a card",
            crate::l10n::locale::Language::Korean => "카드를 선택하세요",
        }
        .to_string();

        vec![crate::game_state::modal::deck::CardSelectionStep {
            title,
            count: 1,
            filter: crate::game_state::modal::deck::CardSelectionFilter::Any,
        }]
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

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let cards = headed_cards_from_raw_core(game_state.raw_core_state());
        let card_id = cards
            .iter()
            .filter(|card| card.rank != Rank::Ace)
            .max_by_key(|card| card.rank.ordinal())
            .map(|card| card.id)
            .into_iter()
            .collect();
        vec![card_id]
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_screwdriver_card_service,
        || crate::Rarity::Rare,
    );

fn generate_screwdriver_card_service() -> CardService {
    ScrewdriverCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn screwdriver_heuristic_selects_the_highest_non_ace_card() {
        let game_state = crate::game_state::create_initial_game_state();
        let selected_card_id = ScrewdriverCardService.heuristic_best_selection(&game_state)[0][0];

        assert_eq!(
            game_state.deck.get_card(selected_card_id).unwrap().rank,
            Rank::King
        );
    }

    #[cfg(test)]
    #[test]
    fn screwdriver_headless_use_card_service_increases_the_selected_card_rank() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = ScrewdriverCardService;
        let selected_card_id = service.heuristic_best_selection(&game_state)[0][0];
        game_state.headless = true;

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::UseCardService {
                card_service: service.into_card_service(),
                locale: crate::l10n::Locale::KOREAN,
            },
        );

        assert_eq!(
            game_state.deck.get_card(selected_card_id).unwrap().rank,
            Rank::Ace
        );
    }
}
