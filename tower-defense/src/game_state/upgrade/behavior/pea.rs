use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PeaUpgrade;

impl UpgradeBehavior for PeaUpgrade {
    fn max_hp_plus(&self) -> f32 {
        10.0
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::PLAYER_STATS
    }

    fn on_upgrade_acquired_mut(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        let flags = self.on_upgrade_acquired(game_state);
        game_state.hp = game_state.max_hp();
        flags
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Pea",
            crate::l10n::locale::Language::Korean => "완두콩",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Increase max HP by 10 and heal to full",
            crate::l10n::locale::Language::Korean => "최대 체력이 10 증가하고 즉시 회복합니다",
        });
    }
}

impl PeaUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Pea(PeaUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PeaUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn pea_increases_max_hp_and_fully_heals() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = 1.0;

        game_state.upgrade(crate::game_state::upgrade::PeaUpgrade::into_upgrade());

        assert_eq!(game_state.upgrade_state.max_hp_plus(), 10);
        assert!((game_state.max_hp() - (game_state.config.player.max_hp + 10.0)).abs() < f32::EPSILON);
        assert!((game_state.hp - game_state.max_hp()).abs() < f32::EPSILON);
    }
}

