use super::*;
use crate::card::Engraving;
#[cfg(test)]
use crate::game_state::GameState;

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

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep> {
        let title = match locale.language {
            crate::l10n::locale::Language::English => "Select 2 cards to engrave",
            crate::l10n::locale::Language::Korean => "각인할 카드 2장을 선택하세요",
        }
        .to_string();

        vec![crate::game_state::modal::deck::CardSelectionStep {
            title,
            count: ENGRAVE_COUNT,
            filter: crate::game_state::modal::deck::CardSelectionFilter::NotEngraved,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::MAGNET)
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

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let mut candidates: Vec<_> = headed_cards_from_raw_core(game_state.raw_core_state())
            .into_iter()
            .filter(|card| card.engraving().is_none())
            .collect();
        candidates.sort_by(|a, b| {
            b.polish_pct()
                .cmp(&a.polish_pct())
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
        || crate::Rarity::Epic,
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
                card.add_polish_pct(crate::FixedRatio::ONE);
            });
        }
        game_state.sync_raw_core_from_projection();

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
        game_state.sync_raw_core_from_projection();

        let selected = MagnetCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected[0].len(), ENGRAVE_COUNT);
        for card_id in &selected[0] {
            assert!(!engraved.contains(card_id));
            assert_eq!(game_state.deck.get_card(*card_id).unwrap().rank, Rank::King);
        }
    }

    #[cfg(test)]
    #[test]
    fn magnet_headless_use_card_service_engraves_the_selected_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = MagnetCardService;
        let selected = service.heuristic_best_selection(&game_state)[0].clone();
        game_state.headless = true;

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::UseCardService {
                card_service: service.into_card_service(),
                locale: crate::l10n::Locale::KOREAN,
            },
        );

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
