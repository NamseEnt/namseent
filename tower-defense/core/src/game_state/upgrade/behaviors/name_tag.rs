use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, push_acquired_upgrade, recovery_none, set_optional_id,
    tower_template_bonus_none,
};

fn mirror_payload(u: &mut UpgradeEntryState) {
    u.ratio_values_raw.push(2_000_000);
    u.optional_ids.push(None);
}

fn tower_placed_mirror(
    core: &mut crate::CoreState,
    index: usize,
    tower_id: u64,
    _: bool,
    _: &crate::TowerTemplateState,
    _: &mut usize,
) -> bool {
    if core.upgrades.upgrades[index]
        .optional_ids
        .first()
        .copied()
        .flatten()
        .is_none()
    {
        return set_optional_id(&mut core.upgrades.upgrades[index], 0, Some(tower_id));
    }
    false
}

fn tower_bonus_mirror(upgrade: &UpgradeEntryState, tower: &crate::TowerState) -> i64 {
    if upgrade.optional_ids.first().copied().flatten() == tower.id {
        upgrade.ratio_values_raw.first().copied().unwrap_or(0)
    } else {
        0
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::NameTag,
    generate_payload: mirror_payload,
    rarity: crate::Rarity::Epic,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_mirror,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: tower_placed_mirror,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
