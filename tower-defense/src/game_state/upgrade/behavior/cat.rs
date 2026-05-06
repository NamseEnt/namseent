use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CatUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for CatUpgrade {
    fn gold_earn_plus(&self) -> usize {
        self.add
    }
}

impl CatUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Cat(CatUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(upgrade_state: &UpgradeState) -> Upgrade {
    CatUpgrade::into_upgrade(next_cat_add(upgrade_state.gold_earn_plus()))
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.gold_earn_plus(), super::MAX_GOLD_EARN_PLUS))
}

fn next_cat_add(gold_earn_plus: usize) -> usize {
    match gold_earn_plus {
        0 | 1 => 1,
        2 => 2,
        4 => 4,
        8 => 8,
        _ => 0,
    }
}
