use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SpannerUpgrade;

impl UpgradeBehavior for SpannerUpgrade {
    fn clear_shield_on_stage_start(&self) -> bool {
        false
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
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Keep shield across stage transitions",
            crate::l10n::locale::Language::Korean => "스테이지 전환 시 보호막을 유지합니다",
        });
    }
}

impl SpannerUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Spanner(SpannerUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    SpannerUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn spanner_keeps_shield_across_stage_transition() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.shield = 50.0;
        gs.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::SpannerUpgrade::into_upgrade(),
            None,
        ));

        gs.goto_next_stage();

        assert_eq!(gs.shield, 50.0);
    }
}
