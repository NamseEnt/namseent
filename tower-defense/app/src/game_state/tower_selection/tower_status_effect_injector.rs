use crate::game_state::tower::TowerTemplate;
use crate::game_state::upgrade::UpgradeState;

pub fn inject_status_effects(
    tower: &mut TowerTemplate,
    _upgrade_state: &UpgradeState,
    rerolled_count: usize,
) {
    tower.rerolled_count = rerolled_count;
}
