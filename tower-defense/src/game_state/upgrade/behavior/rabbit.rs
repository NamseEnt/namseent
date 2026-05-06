use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct RabbitUpgrade;

impl UpgradeBehavior for RabbitUpgrade {
    fn skip_rank_for_straight(&self) -> bool {
        true
    }
}

impl RabbitUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Rabbit(RabbitUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    RabbitUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.skip_rank_for_straight() as usize, 1))
}