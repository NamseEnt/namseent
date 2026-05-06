use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BackpackUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for BackpackUpgrade {
    fn shop_slot_expand(&self) -> usize {
        self.add
    }
}

impl BackpackUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Backpack(BackpackUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BackpackUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.shop_slot_expand(),
        super::MAX_SHOP_SLOT_EXPAND,
    ))
}
