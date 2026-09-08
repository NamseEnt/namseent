use super::*;
#[cfg(test)]
use crate::game_state::GameState;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PliersCardService;

impl PliersCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Pliers(self)
    }
}

impl CardServiceBehavior for PliersCardService {
    fn key(&self) -> &'static str {
        "pliers"
    }

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep> {
        let title = match locale.language {
            crate::l10n::locale::Language::English => {
                "Select an engraved card to remove its engraving"
            }
            crate::l10n::locale::Language::Korean => "각인을 제거할 카드를 선택하세요",
        }
        .to_string();

        vec![crate::game_state::modal::deck::CardSelectionStep {
            title,
            count: 1,
            filter: crate::game_state::modal::deck::CardSelectionFilter::Engraved,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::PLIERS)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Pliers",
            crate::l10n::locale::Language::Korean => "플라이어",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select 1 engraved card and remove its engraving.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("각인된 카드 1장을 선택해 각인을 제거합니다.")
            }
        };
    }

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let cards = headed_cards_from_raw_core(game_state.raw_core_state());
        let card_id = cards
            .iter()
            .filter(|card| card.engraving().is_some())
            .max_by(|a, b| {
                a.polish_pct()
                    .cmp(&b.polish_pct())
                    .then_with(|| a.rank.ordinal().cmp(&b.rank.ordinal()))
            })
            .map(|card| card.id)
            .into_iter()
            .collect();
        vec![card_id]
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_pliers_card_service,
        || crate::Rarity::Rare,
    );

fn generate_pliers_card_service() -> CardService {
    PliersCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Engraving;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn heuristic_selects_one_engraved_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let engraved = game_state.deck.all_cards()[0].id;
        game_state.deck.modify_card(engraved, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
        });
        game_state.sync_raw_core_from_projection();

        let selected = PliersCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected, vec![vec![engraved]]);
    }

    #[test]
    fn heuristic_handles_a_deck_without_engraved_cards() {
        let game_state = crate::game_state::create_initial_game_state();

        assert_eq!(
            PliersCardService.heuristic_best_selection(&game_state),
            vec![vec![]]
        );
    }

    #[cfg(test)]
    #[test]
    fn headless_use_card_service_removes_the_selected_engraving() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let selected = game_state.deck.all_cards()[0].id;
        game_state.deck.modify_card(selected, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
        });
        game_state.headless = true;

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::UseCardService {
                card_service: PliersCardService.into_card_service(),
                locale: crate::l10n::Locale::KOREAN,
            },
        );

        assert_eq!(
            game_state.deck.get_card(selected).unwrap().engraving(),
            None
        );
    }
}
