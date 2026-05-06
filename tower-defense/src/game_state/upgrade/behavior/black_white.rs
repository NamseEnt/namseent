use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BlackWhiteUpgrade;

impl UpgradeBehavior for BlackWhiteUpgrade {
    fn treat_suits_as_same(&self) -> bool {
        true
    }
}

impl BlackWhiteUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BlackWhite(BlackWhiteUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BlackWhiteUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.treat_suits_as_same() as usize, 1))
}