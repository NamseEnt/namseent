use super::*;

// Treasure upgrades (simple gold/shop bonuses)
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CatUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for CatUpgrade {
    fn gold_earn_plus(&self) -> usize {
        self.add
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BackpackUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for BackpackUpgrade {
    fn shop_slot_expand(&self) -> usize {
        self.add
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DiceBundleUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for DiceBundleUpgrade {
    fn dice_chance_plus(&self) -> usize {
        self.add
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EnergyDrinkUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for EnergyDrinkUpgrade {
    fn shop_item_price_minus(&self) -> usize {
        self.add
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for EraserUpgrade {
    fn removed_number_rank_count(&self) -> usize {
        self.add
    }
}
