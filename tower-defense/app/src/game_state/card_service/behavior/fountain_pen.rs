use super::*;
#[cfg(test)]
use crate::game_state::GameState;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FountainPenCardService {
    pub polish_pct: crate::FixedRatio,
}

impl FountainPenCardService {
    pub fn new(polish_pct: crate::FixedRatio) -> Self {
        Self { polish_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::FountainPen(self)
    }
}

impl CardServiceBehavior for FountainPenCardService {
    fn key(&self) -> &'static str {
        "fountain_pen"
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
            filter: crate::game_state::modal::deck::CardSelectionFilter::Number,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::FOUNTAIN_PEN)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Fountain Pen",
            crate::l10n::locale::Language::Korean => "만년필",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select one number card and give it +300% damage.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("숫자 카드 1장을 선택해 데미지 +300%를 부여합니다.")
            }
        };
    }

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        // FountainPen: damage bonus to high rank/potential cards.
        let mut cards = headed_cards_from_raw_core(game_state.raw_core_state());
        cards.sort_by_key(|c| std::cmp::Reverse(c.rank as u8)); // highest rank first
        cards.iter().rev().take(3).map(|c| vec![c.id]).collect()
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
        generate_fountain_pen_card_service,
        || crate::Rarity::Common,
    );

fn generate_fountain_pen_card_service() -> CardService {
    FountainPenCardService::new(crate::FixedRatio::from_integer(3)).into_card_service()
}
