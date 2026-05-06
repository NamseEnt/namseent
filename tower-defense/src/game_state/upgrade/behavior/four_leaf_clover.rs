use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FourLeafCloverUpgrade;

impl UpgradeBehavior for FourLeafCloverUpgrade {
    fn shorten_straight_flush_to_4_cards(&self) -> bool {
        true
    }
}

impl FourLeafCloverUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::FourLeafClover(FourLeafCloverUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    FourLeafCloverUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.shorten_straight_flush_to_4_cards() as usize, 1))
}