use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, push_acquired_upgrade, recovery_none, scalar, scalar_zero,
    set_scalar, tower_bonus_none, tower_template_bonus_none,
};

const DICE_BONUS: usize = 2;

const METRONOME_STAGE_INTERVAL: usize = 2;

fn acquire_stage_marker(core: &mut crate::CoreState, mut u: UpgradeEntryState) -> usize {
    set_scalar(&mut u, 0, core.progress.stage as u64);
    push_acquired_upgrade(core, u)
}

fn stage_start_metronome(core: &mut crate::CoreState, index: usize, stage: usize) -> bool {
    let acquired_stage = scalar(&core.upgrades.upgrades[index], 0);
    if stage.saturating_sub(acquired_stage) % METRONOME_STAGE_INTERVAL
        == METRONOME_STAGE_INTERVAL - 1
    {
        core.progress.left_dice = core.progress.left_dice.saturating_add(DICE_BONUS);
    }
    false
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Metronome,
    generate_payload: scalar_zero,
    rarity: crate::Rarity::Common,
    cache: no_cache,
    acquire: acquire_stage_marker,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: stage_start_metronome,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
