use super::*;
#[cfg(test)]
use crate::game_state::GameState;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserCardService;

impl EraserCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Eraser(self)
    }
}

impl CardServiceBehavior for EraserCardService {
    fn key(&self) -> &'static str {
        "eraser"
    }

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep> {
        let title = match locale.language {
            crate::l10n::locale::Language::English => "Select a card to remove",
            crate::l10n::locale::Language::Korean => "제거할 카드를 선택하세요",
        }
        .to_string();

        vec![crate::game_state::modal::deck::CardSelectionStep {
            title,
            count: 1,
            filter: crate::game_state::modal::deck::CardSelectionFilter::Any,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::ERASER)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Eraser",
            crate::l10n::locale::Language::Korean => "지우개",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select 1 card and remove it from the deck.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드를 1장 선택해 덱에서 제거합니다.")
            }
        };
    }

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        // Eraser: remove lowest rank card (least valuable).
        let mut cards = headed_cards_from_raw_core(game_state.raw_core_state());
        cards.sort_by_key(|c| c.rank as u8); // lowest first
        cards.iter().take(1).map(|c| vec![c.id]).collect()
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_eraser_card_service,
        || crate::Rarity::Rare,
    );

fn generate_eraser_card_service() -> CardService {
    EraserCardService::new().into_card_service()
}
