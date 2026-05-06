use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PairChopsticksUpgrade {
    pub damage_multiplier: f32,
}

impl UpgradeBehavior for PairChopsticksUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::EvenOdd { even: true },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl PairChopsticksUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::PairChopsticks(PairChopsticksUpgrade { damage_multiplier })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PairChopsticksUpgrade::into_upgrade(1.4)
}