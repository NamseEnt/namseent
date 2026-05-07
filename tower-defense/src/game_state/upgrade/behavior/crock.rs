use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CrockUpgrade;

impl UpgradeBehavior for CrockUpgrade {
    fn tower_upgrade_damage_bonus(
        &self,
        game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if game_state.gold >= 1000 {
            Some((TowerUpgradeTarget::Global, (game_state.gold / 1000) as f32))
        } else {
            None
        }
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Crock",
            crate::l10n::locale::Language::Korean => "항아리",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Damage increases with your gold",
            crate::l10n::locale::Language::Korean => "골드가 많을수록 피해가 증가합니다",
        });
    }
}

impl CrockUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Crock(CrockUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    CrockUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn crock_increases_tower_damage_for_existing_towers() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::FullHouse,
            crate::card::Suit::Hearts,
            crate::card::Rank::Queen,
        );
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.place_tower(tower);

        let tower_id = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower to be placed")
            .id();
        let before_damage = game_state
            .towers
            .iter()
            .find(|tower| tower.id() == tower_id)
            .expect("expected placed tower")
            .calculate_projectile_damage(&[], 1.0);

        game_state.gold = 2500;
        game_state.upgrade(crate::game_state::upgrade::CrockUpgrade::into_upgrade());

        let after_damage = {
            let tower = game_state
                .towers
                .iter()
                .find(|tower| tower.id() == tower_id)
                .expect("expected placed tower");
            let upgrade_bonuses = game_state
                .upgrade_state
                .tower_upgrade_damage_bonuses(&game_state);
            tower.calculate_projectile_damage(&upgrade_bonuses, 1.0)
        };

        assert!(before_damage > 0.0);
        assert!(after_damage > before_damage);
    }
}

