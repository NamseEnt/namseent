use super::super::UpgradeCacheContribution;
use crate::CoreState;

use super::super::UpgradeAcquireRecovery;

pub(super) fn base_cache() -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        clear_shield_on_stage_start: true,
        ..Default::default()
    }
}

pub(super) fn cache_hp(amount: i64) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        max_hp_plus_raw: amount,
        clear_shield_on_stage_start: true,
        ..Default::default()
    }
}

pub(super) fn recovery_amount(amount: i64) -> UpgradeAcquireRecovery {
    UpgradeAcquireRecovery::Amount(amount)
}

pub(super) fn push_acquired_upgrade(
    core: &mut CoreState,
    mut upgrade: super::super::UpgradeEntry,
) -> usize {
    upgrade.id = core.next_upgrade_id();
    core.upgrades.upgrades.push(upgrade);
    0
}
