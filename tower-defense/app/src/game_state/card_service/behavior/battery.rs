use super::*;
use crate::card::Engraving;
#[cfg(test)]
use crate::game_state::GameState;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BatteryCardService;

impl BatteryCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Battery(self)
    }
}

impl CardServiceBehavior for BatteryCardService {
    fn key(&self) -> &'static str {
        "battery"
    }

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep> {
        let title = match locale.language {
            crate::l10n::locale::Language::English => "Select a card to engrave",
            crate::l10n::locale::Language::Korean => "각인할 카드를 선택하세요",
        }
        .to_string();

        vec![crate::game_state::modal::deck::CardSelectionStep {
            title,
            count: 1,
            filter: crate::game_state::modal::deck::CardSelectionFilter::NotEngraved,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BATTERY)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Battery",
            crate::l10n::locale::Language::Korean => "배터리",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Engraves overcharge on 1 card.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드 1장에 과충전을 각인합니다.")
            }
        };
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            crate::l10n::word::Word::Engraving(Some(Engraving::Overcharge)).tooltip_section(locale),
        ]
    }

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let cards = headed_cards_from_raw_core(game_state.raw_core_state());
        let card_id = cards
            .iter()
            .filter(|card| card.engraving().is_none())
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
        generate_battery_card_service,
        || crate::Rarity::Epic,
    );

fn generate_battery_card_service() -> CardService {
    BatteryCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Rank;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn battery_heuristic_selects_one_unengraved_card() {
        let game_state = crate::game_state::create_initial_game_state();

        let selected = BatteryCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].len(), 1);
        assert_eq!(
            game_state.deck.get_card(selected[0][0]).unwrap().rank,
            Rank::Ace
        );
    }

    #[test]
    fn battery_heuristic_prefers_the_most_polished_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let polished = game_state
            .deck
            .all_cards()
            .iter()
            .find(|card| card.rank == Rank::Two)
            .unwrap()
            .id;
        game_state.deck.modify_card(polished, |card| {
            card.add_polish_pct(crate::FixedRatio::ONE);
        });
        game_state.sync_raw_core_from_projection();

        let selected = BatteryCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected[0][0], polished);
    }

    #[test]
    fn battery_heuristic_skips_already_engraved_cards() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let aces: Vec<_> = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.rank == Rank::Ace)
            .map(|card| card.id)
            .collect();
        for card_id in &aces {
            game_state.deck.modify_card(*card_id, |card| {
                card.effects.engraving = Some(Engraving::Magnet);
            });
        }
        game_state.sync_raw_core_from_projection();

        let selected = BatteryCardService.heuristic_best_selection(&game_state);

        assert!(!aces.contains(&selected[0][0]));
        assert_eq!(
            game_state.deck.get_card(selected[0][0]).unwrap().rank,
            Rank::King
        );
    }

    #[cfg(test)]
    #[test]
    fn battery_headless_use_card_service_engraves_overcharge() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = BatteryCardService;
        let selected = service.heuristic_best_selection(&game_state)[0][0];
        game_state.headless = true;

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::UseCardService {
                card_service: service.into_card_service(),
                locale: crate::l10n::Locale::KOREAN,
            },
        );

        assert_eq!(
            game_state.deck.get_card(selected).unwrap().engraving(),
            Some(Engraving::Overcharge)
        );
    }
}
