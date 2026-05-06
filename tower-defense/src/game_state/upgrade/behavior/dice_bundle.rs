use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DiceBundleUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for DiceBundleUpgrade {
    fn dice_chance_plus(&self) -> usize {
        self.add
    }
}

impl DiceBundleUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::DiceBundle(DiceBundleUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    DiceBundleUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.dice_chance_plus(),
        super::MAX_DICE_CHANCE_PLUS,
    ))
}
