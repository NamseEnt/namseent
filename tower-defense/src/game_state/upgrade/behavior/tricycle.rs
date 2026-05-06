use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TricycleUpgrade {
    pub damage_multiplier: f32,
}

impl UpgradeBehavior for TricycleUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl TricycleUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::Tricycle(TricycleUpgrade { damage_multiplier })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    TricycleUpgrade::into_upgrade(1.75)
}