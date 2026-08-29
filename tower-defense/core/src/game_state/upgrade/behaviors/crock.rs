use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, push_acquired_upgrade, recovery_none, scalar, scalar_zero,
    set_scalar,
};

const CROCK_DAMAGE_PER_STEP_RAW: i64 = 250_000;

const CROCK_GOLD_PER_DAMAGE: usize = 100;

fn acquire_crock(core: &mut crate::CoreState, mut u: UpgradeEntryState) -> usize {
    set_scalar(
        &mut u,
        0,
        (core.progress.gold / CROCK_GOLD_PER_DAMAGE) as u64,
    );
    push_acquired_upgrade(core, u)
}

fn update_crock(core: &mut crate::CoreState, index: usize) -> bool {
    let next_step = (core.progress.gold / CROCK_GOLD_PER_DAMAGE).min(u64::MAX as usize) as u64;
    set_scalar(&mut core.upgrades.upgrades[index], 0, next_step)
}

fn update_crock_shop(core: &mut crate::CoreState, index: usize, _: bool) -> bool {
    update_crock(core, index)
}

fn tower_bonus_crock(upgrade: &UpgradeEntryState, _: &crate::TowerState) -> i64 {
    scalar(upgrade, 0)
        .min(i64::MAX as usize)
        .try_into()
        .unwrap_or(i64::MAX)
        .saturating_mul(CROCK_DAMAGE_PER_STEP_RAW)
}

fn tower_template_bonus_crock(upgrade: &UpgradeEntryState, _: &crate::TowerTemplateState) -> i64 {
    scalar(upgrade, 0)
        .min(i64::MAX as usize)
        .try_into()
        .unwrap_or(i64::MAX)
        .saturating_mul(CROCK_DAMAGE_PER_STEP_RAW)
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Crock,
    generate_payload: scalar_zero,
    rarity: crate::Rarity::Epic,
    cache: no_cache,
    acquire: acquire_crock,
    recovery: recovery_none,
    tower_bonus: tower_bonus_crock,
    tower_bonus_for_template: tower_template_bonus_crock,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: update_crock,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: update_crock_shop,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
