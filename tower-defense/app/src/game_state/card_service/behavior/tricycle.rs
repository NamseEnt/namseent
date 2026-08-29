use super::*;
use crate::card::Rank;
#[cfg(test)]
use crate::game_state::GameState;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TricycleCardService {
    pub polish_pct: crate::FixedRatio,
}

impl TricycleCardService {
    pub fn new(polish_pct: crate::FixedRatio) -> Self {
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
            filter: crate::game_state::modal::deck::CardSelectionFilter::Or(vec![
                crate::game_state::modal::deck::CardSelectionFilter::Rank(Rank::Ace),
                crate::game_state::modal::deck::CardSelectionFilter::Rank(Rank::Two),
                crate::game_state::modal::deck::CardSelectionFilter::Rank(Rank::Three),
            ]),
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::TRICYCLE)
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

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let mut cards = headed_cards_from_raw_core(game_state.raw_core_state())
            .into_iter()
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
    TricycleCardService::new(crate::FixedRatio::from_integer(2)).into_card_service()
}
