use super::*;
#[cfg(test)]
use crate::game_state::GameState;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrushCardService {
    pub polish_pct: crate::FixedRatio,
}

impl BrushCardService {
    pub fn new(polish_pct: crate::FixedRatio) -> Self {
        Self { polish_pct }
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Brush(self)
    }
}

impl CardServiceBehavior for BrushCardService {
    fn key(&self) -> &'static str {
        "brush"
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
            filter: crate::game_state::modal::deck::CardSelectionFilter::Face,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BRUSH)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Brush",
            crate::l10n::locale::Language::Korean => "붓",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select one face card and give it +300% damage.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("그림 카드 1장을 선택해 데미지 +300%를 부여합니다.")
            }
        };
    }

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let mut cards = headed_cards_from_raw_core(game_state.raw_core_state());
        cards.sort_by_key(|c| std::cmp::Reverse((c.rank as i32, c.suit as i32))); // high rank/suit 우선
        cards.iter().take(3).map(|c| vec![c.id]).collect()
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
        generate_brush_card_service,
        || crate::Rarity::Common,
    );

fn generate_brush_card_service() -> CardService {
    BrushCardService::new(crate::FixedRatio::from_integer(3)).into_card_service()
}
