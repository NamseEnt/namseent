use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for EraserUpgrade {
    fn removed_number_rank_count(&self) -> usize {
        self.add
    }
}

impl EraserUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Eraser(EraserUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    EraserUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.removed_number_rank_count(),
        super::MAX_REMOVE_NUMBER_RANKS,
    ))
}