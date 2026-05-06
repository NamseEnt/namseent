use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EnergyDrinkUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for EnergyDrinkUpgrade {
    fn shop_item_price_minus(&self) -> usize {
        self.add
    }
}

impl EnergyDrinkUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::EnergyDrink(EnergyDrinkUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    EnergyDrinkUpgrade::into_upgrade(5)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.shop_item_price_minus(),
        super::MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE,
    ))
}
