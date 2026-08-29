use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FangUpgrade {
    pub(crate) add: usize,
}

impl UpgradePresentation for FangUpgrade {
    fn key(&self) -> &'static str {
        "fang"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::FANG)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("{}", self.add),
            crate::theme::palette::RED,
        )]
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Fang",
            crate::l10n::locale::Language::Korean => "송곳니",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .l10n(Word::Health.name(), locale)
                .with_bold(format!(" +{}", self.add))
                .static_text(" when a monster dies"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("적 처시 시 ")
                .l10n(Word::Health.name(), locale)
                .with_bold(format!(" +{}", self.add)),
        };
    }
}

impl FangUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Fang(FangUpgrade { add: 1 })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fang_recovers_hp_when_monster_dies() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = crate::Health::from_integer(10);

        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::FangUpgrade::into_upgrade(),
            None,
        ));
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::MonsterDeath);

        assert_eq!(game_state.hp, crate::Health::from_integer(11));
    }

    #[test]
    fn fang_recovery_respects_current_max_hp() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = game_state.max_hp();

        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::FangUpgrade::into_upgrade(),
            None,
        ));
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::MonsterDeath);

        assert_eq!(game_state.hp, game_state.max_hp());
    }
}
