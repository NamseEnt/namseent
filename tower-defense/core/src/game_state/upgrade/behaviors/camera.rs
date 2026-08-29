use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, empty_payload, no_cache, no_limit, push_acquired_upgrade, recovery_none,
    tower_bonus_none, tower_template_bonus_none,
};

const CAMERA_GOLD_REWARD: usize = 50;

fn tower_placed_camera(
    _: &mut crate::CoreState,
    _: usize,
    _: u64,
    is_face: bool,
    _: &crate::TowerTemplateState,
    camera_reward: &mut usize,
) -> bool {
    if is_face {
        *camera_reward = camera_reward.saturating_add(CAMERA_GOLD_REWARD);
    }
    false
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Camera,
    generate_payload: empty_payload,
    rarity: crate::Rarity::Epic,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: tower_placed_camera,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
