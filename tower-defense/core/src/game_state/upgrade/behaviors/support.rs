use super::super::UpgradeCacheContribution;
use crate::CoreState;
use crate::{TowerState, TowerTemplateState};

use super::super::UpgradeAcquireRecovery;
use super::super::UpgradeEntryState;

pub(crate) fn scalar(upgrade: &UpgradeEntryState, index: usize) -> usize {
    upgrade
        .scalar_values
        .get(index)
        .copied()
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(0)
}

pub(crate) fn ratio(upgrade: &UpgradeEntryState, index: usize) -> i64 {
    upgrade.ratio_values_raw.get(index).copied().unwrap_or(0)
}

pub(crate) fn set_scalar(upgrade: &mut UpgradeEntryState, index: usize, value: u64) -> bool {
    if upgrade.scalar_values.len() <= index {
        upgrade.scalar_values.resize(index + 1, 0);
    }
    let changed = upgrade.scalar_values[index] != value;
    upgrade.scalar_values[index] = value;
    changed
}

pub(crate) fn set_ratio(upgrade: &mut UpgradeEntryState, index: usize, value: i64) -> bool {
    if upgrade.ratio_values_raw.len() <= index {
        upgrade.ratio_values_raw.resize(index + 1, 0);
    }
    let changed = upgrade.ratio_values_raw[index] != value;
    upgrade.ratio_values_raw[index] = value;
    changed
}

pub(crate) fn set_bool(upgrade: &mut UpgradeEntryState, index: usize, value: bool) -> bool {
    if upgrade.bool_values.len() <= index {
        upgrade.bool_values.resize(index + 1, false);
    }
    let changed = upgrade.bool_values[index] != value;
    upgrade.bool_values[index] = value;
    changed
}

pub(crate) fn set_optional_id(
    upgrade: &mut UpgradeEntryState,
    index: usize,
    value: Option<u64>,
) -> bool {
    if upgrade.optional_ids.len() <= index {
        upgrade.optional_ids.resize(index + 1, None);
    }
    let changed = upgrade.optional_ids[index] != value;
    upgrade.optional_ids[index] = value;
    changed
}

pub(crate) fn popcorn_damage_bonus_raw(upgrade: &UpgradeEntryState) -> i64 {
    popcorn_damage_bonus_raw_with_waves(upgrade, scalar(upgrade, 1))
}

pub(crate) fn popcorn_damage_bonus_raw_with_waves(
    upgrade: &UpgradeEntryState,
    waves_remaining: usize,
) -> i64 {
    let max_multiplier = upgrade.ratio_values_raw.first().copied().unwrap_or(0);
    let duration = scalar(upgrade, 0).max(1);
    if waves_remaining == 0 {
        return 0;
    }
    let elapsed = duration.saturating_sub(waves_remaining);
    let multiplier = if duration <= 1 {
        max_multiplier
    } else {
        let step = max_multiplier
            .saturating_sub(crate::RATIO_SCALE)
            .max(0)
            .saturating_div((duration - 1).min(i64::MAX as usize) as i64);
        max_multiplier
            .saturating_sub(step.saturating_mul(elapsed.min(i64::MAX as usize) as i64))
            .max(crate::RATIO_SCALE)
    };
    multiplier.saturating_sub(crate::RATIO_SCALE)
}

pub(super) fn empty_payload(_: &mut UpgradeEntryState) {}

pub(super) fn scalar_one(upgrade: &mut UpgradeEntryState) {
    upgrade.scalar_values.push(1);
}

pub(super) fn scalar_five(upgrade: &mut UpgradeEntryState) {
    upgrade.scalar_values.push(5);
}

pub(super) fn scalar_ten(upgrade: &mut UpgradeEntryState) {
    upgrade.scalar_values.push(10);
}

pub(super) fn scalar_zero(upgrade: &mut UpgradeEntryState) {
    upgrade.scalar_values.push(0);
}

pub(super) fn ratio_half(upgrade: &mut UpgradeEntryState) {
    upgrade.ratio_values_raw.push(500_000);
}

pub(super) fn bool_true(upgrade: &mut UpgradeEntryState) {
    upgrade.bool_values.push(true);
}

pub(super) fn base_cache() -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        clear_shield_on_stage_start: true,
        ..Default::default()
    }
}

pub(super) fn no_cache(_: &UpgradeEntryState) -> UpgradeCacheContribution {
    base_cache()
}

pub(super) fn cache_hp(amount: i64) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        max_hp_plus_raw: amount,
        clear_shield_on_stage_start: true,
        ..Default::default()
    }
}

pub(super) fn recovery_none() -> UpgradeAcquireRecovery {
    UpgradeAcquireRecovery::None
}

pub(super) fn recovery_amount(amount: i64) -> UpgradeAcquireRecovery {
    UpgradeAcquireRecovery::Amount(amount)
}

pub(super) fn tower_bonus_none(_: &UpgradeEntryState, _: &TowerState) -> i64 {
    0
}

pub(super) fn tower_template_bonus_none(_: &UpgradeEntryState, _: &TowerTemplateState) -> i64 {
    0
}

pub(super) fn no_limit(_: &CoreState) -> Option<(usize, usize)> {
    None
}

pub(super) fn push_acquired_upgrade(core: &mut CoreState, mut upgrade: UpgradeEntryState) -> usize {
    upgrade.id = core.next_upgrade_id();
    core.upgrades.upgrades.push(upgrade);
    0
}

pub(super) fn merge_scalar_upgrade(core: &mut CoreState, mut upgrade: UpgradeEntryState) -> usize {
    let add = scalar(&upgrade, 0);
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.upgrade_kind().ok() == upgrade.upgrade_kind().ok())
    {
        let total = scalar(existing, 0).saturating_add(add);
        set_scalar(existing, 0, total as u64);
    } else {
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
    0
}

pub(super) fn merge_ratio_upgrade(core: &mut CoreState, mut upgrade: UpgradeEntryState) -> usize {
    let add = ratio(&upgrade, 0);
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.upgrade_kind().ok() == upgrade.upgrade_kind().ok())
    {
        set_ratio(existing, 0, ratio(existing, 0).saturating_add(add));
    } else {
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
    0
}

pub(super) fn noop_monster_death(_: &CoreState, _: usize, _: &mut usize, _: &mut i64) {}

pub(super) fn noop_bool_trigger(_: &mut CoreState, _: usize) -> bool {
    false
}

pub(super) fn noop_shop_purchase(_: &mut CoreState, _: usize, _: bool) -> bool {
    false
}

pub(super) fn noop_tower_placed(
    _: &mut CoreState,
    _: usize,
    _: u64,
    _: bool,
    _: &TowerTemplateState,
    _: &mut usize,
) -> bool {
    false
}

pub(super) fn noop_tower_removed(_: &mut CoreState, _: usize, _: usize) {}

pub(super) fn noop_stage_start(_: &mut CoreState, _: usize, _: usize) -> bool {
    false
}

pub(super) fn noop_stage_end(
    _: &mut CoreState,
    _: usize,
    _: bool,
    _: usize,
    _: usize,
) -> (bool, usize) {
    (false, 0)
}

pub(super) const NO_TRIGGERS: super::super::definition::UpgradeTriggerDefinition =
    super::super::definition::UpgradeTriggerDefinition {
        monster_death: noop_monster_death,
        gold_earned: noop_bool_trigger,
        card_rerolled: noop_bool_trigger,
        shop_purchase: noop_shop_purchase,
        tower_placed: noop_tower_placed,
        tower_removed: noop_tower_removed,
        stage_start: noop_stage_start,
        stage_end: noop_stage_end,
    };
