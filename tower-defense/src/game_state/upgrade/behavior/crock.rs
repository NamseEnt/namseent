use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CrockUpgrade {
    pub current_step: usize,
}

const CROCK_GOLD_PER_DAMAGE: usize = 1000;
const CROCK_DAMAGE_PER_STEP: f32 = 1.0;

impl CrockUpgrade {
    fn current_damage_bonus(&self) -> f32 {
        self.current_step as f32 * CROCK_DAMAGE_PER_STEP
    }

    fn update_step_from_gold(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        let next_step = game_state.gold / CROCK_GOLD_PER_DAMAGE;
        if next_step == self.current_step {
            return UpgradeUpdateFlags::NONE;
        }

        self.current_step = next_step;
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl UpgradeBehavior for CrockUpgrade {
    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.current_step > 0 {
            Some((TowerUpgradeTarget::Global, self.current_damage_bonus()))
        } else {
            None
        }
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        let mut upgrade = self;
        let flags = upgrade.update_step_from_gold(game_state);
        game_state.upgrade_state.upgrades.push(upgrade.into());
        flags
    }

    fn on_gold_earned(&mut self, game_state: &mut GameState, _earned: usize) -> UpgradeUpdateFlags {
        self.update_step_from_gold(game_state)
    }

    fn on_gold_spent(&mut self, game_state: &mut GameState, _spent: usize) -> UpgradeUpdateFlags {
        self.update_step_from_gold(game_state)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Crock",
            crate::l10n::locale::Language::Korean => "항아리",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => format!(
                "Gain +{:.0}% damage for every {} gold (currently +{:.0}%)",
                CROCK_DAMAGE_PER_STEP,
                CROCK_GOLD_PER_DAMAGE,
                self.current_damage_bonus(),
            ),
            crate::l10n::locale::Language::Korean => format!(
                "골드 {}당 피해 +{:.0}% (현재 +{:.0}%)",
                CROCK_GOLD_PER_DAMAGE,
                CROCK_DAMAGE_PER_STEP,
                self.current_damage_bonus(),
            ),
        });
    }
}

impl CrockUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Crock(CrockUpgrade { current_step: 0 })
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
        game_state.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(tower),
            None,
        ));

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
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::CrockUpgrade::into_upgrade(),
            None,
        ));

        let after_damage = {
            let tower = game_state
                .towers
                .iter()
                .find(|tower| tower.id() == tower_id)
                .expect("expected placed tower");
            let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();
            tower.calculate_projectile_damage(&upgrade_bonuses, 1.0)
        };

        assert!(before_damage > 0.0);
        assert!(after_damage > before_damage);
    }
}
