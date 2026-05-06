use crate::game_state::{
    GameState,
    tower::{Tower, TowerTemplate},
};

#[cfg(test)]
pub fn create_mock_game_state() -> GameState {
    crate::game_state::create_initial_game_state()
}

#[cfg(test)]
pub fn first_hand_tower_template(game_state: &GameState) -> TowerTemplate {
    let slot_id = game_state
        .hand
        .get_slot_id_by_index(0)
        .expect("expected at least one hand slot after placing flow");
    game_state
        .hand
        .get_item(slot_id)
        .and_then(|item| item.as_tower())
        .cloned()
        .expect("expected first hand item to be tower template")
}

#[cfg(test)]
pub fn assert_tower_cached_damage_mul(tower: &Tower, expected_mul: f32) {
    let base_damage = tower.calculate_projectile_damage(&[], 1.0);
    let boosted_damage = tower.cached_upgrade_damage();
    assert!((boosted_damage / base_damage - expected_mul).abs() < f32::EPSILON);
}
