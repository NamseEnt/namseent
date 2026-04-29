use crate::game_state::{GameState, tower::{Tower, TowerStatusEffectKind, TowerTemplate}};

pub fn create_mock_game_state() -> GameState {
    crate::game_state::create_initial_game_state_for_tests()
}

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

pub fn assert_template_has_damage_mul(template: &TowerTemplate, expected_mul: f32) {
    assert!(template.default_status_effects.iter().any(|effect| {
        matches!(effect.kind, TowerStatusEffectKind::DamageMul { mul } if (mul - expected_mul).abs() < f32::EPSILON)
    }));
}

pub fn assert_tower_has_damage_mul(tower: &Tower, expected_mul: f32) {
    assert!(tower.status_effects.iter().any(|effect| {
        matches!(effect.kind, TowerStatusEffectKind::DamageMul { mul } if (mul - expected_mul).abs() < f32::EPSILON)
    }));
}