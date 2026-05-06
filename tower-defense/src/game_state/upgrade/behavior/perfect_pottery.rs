use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PerfectPotteryUpgrade {
    pub damage_multiplier: f32,
}

impl UpgradeBehavior for PerfectPotteryUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl PerfectPotteryUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::PerfectPottery(PerfectPotteryUpgrade { damage_multiplier })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PerfectPotteryUpgrade::into_upgrade(1.0)
}