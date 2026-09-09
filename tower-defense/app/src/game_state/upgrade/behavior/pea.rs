use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const PEA_HP_PLUS: HealthDelta = HealthDelta::from_integer(3);

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PeaUpgrade;

impl UpgradePresentation for PeaUpgrade {
    fn key(&self) -> &'static str {
        "pea"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::PEA)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Pea",
            crate::l10n::locale::Language::Korean => "완두콩",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .with_bold("Max Health")
                .static_text(" increased by ")
                .with_bold(format!("{:.0}", PEA_HP_PLUS))
                .static_text(", ")
                .with_bold("Health")
                .static_text(" fully recovered."),
            crate::l10n::locale::Language::Korean => builder
                .with_bold("최대 체력")
                .static_text("을 ")
                .with_bold(format!("{:.0}", PEA_HP_PLUS))
                .static_text(" 늘리고, ")
                .with_bold("체력을 ")
                .static_text("모두 회복합니다."),
        };
    }
}

impl PeaUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Pea(PeaUpgrade)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn pea_increases_max_hp_and_fully_heals() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = crate::Health::from_integer(1);

        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::PeaUpgrade::into_upgrade(),
            None,
        ));

        assert_eq!(
            game_state.upgrade_state.max_hp_plus(),
            crate::HealthDelta::from_integer(3)
        );
        assert_eq!(
            game_state.max_hp(),
            crate::Health::from_raw(game_state.config.player.max_hp_raw)
                .saturating_add_delta(crate::HealthDelta::from_integer(3))
        );
        assert_eq!(game_state.hp, game_state.max_hp());
    }
}
