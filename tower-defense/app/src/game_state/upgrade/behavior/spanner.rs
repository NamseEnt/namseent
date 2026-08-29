use crate::l10n::word::Word;

use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SpannerUpgrade;

impl UpgradePresentation for SpannerUpgrade {
    fn key(&self) -> &'static str {
        "spanner"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::SPANNER)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Spanner",
            crate::l10n::locale::Language::Korean => "스패너",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::Language::Korean => builder
                .static_text("스테이지 종료 시 ")
                .l10n(Word::Shield.name(), locale)
                .static_text("이 사라지지 않습니다"),
            crate::l10n::Language::English => builder
                .static_text("Keep ")
                .l10n(Word::Shield.name(), locale)
                .static_text(" on stage ends"),
        };
    }
}

impl SpannerUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Spanner(SpannerUpgrade)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn spanner_keeps_shield_across_stage_transition() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.shield = crate::Shield::from_integer(50).raw();
        gs.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::SpannerUpgrade::into_upgrade(),
            None,
        ));

        gs.apply_compatibility_action(crate::game_state::CompatibilityAction::StartStage {
            stage: gs.stage,
        });

        assert_eq!(gs.shield_amount(), crate::Shield::from_integer(50));
    }
}
