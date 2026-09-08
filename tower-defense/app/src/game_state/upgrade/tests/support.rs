use crate::game_state::{
    GameState, presentation_effect,
    tower::{Tower, TowerTemplate},
};

#[cfg(test)]
pub fn create_mock_game_state() -> GameState {
    crate::game_state::create_initial_game_state()
}

#[cfg(test)]
pub fn check_defense_end_for_test(
    game_state: &mut GameState,
) -> Option<td_core::DefenseEndOutputState> {
    game_state.sync_raw_core_from_projection();
    let completed_stage = game_state.stage;
    let mut raw_state = game_state.raw_core.state().clone();
    let raw_output = raw_state.resolve_defense_end()?;

    raw_state.update_clear_metrics(raw_output.perfect_clear);
    raw_state.trigger_stage_end_upgrades(
        raw_output.perfect_clear,
        raw_output.gold,
        raw_output.item_count,
    );
    let card_count = raw_state.apply_defense_end_transition(raw_output.transition);
    match raw_output.transition {
        td_core::DefenseEndTransitionState::GameOver => {
            raw_state.extend_events([td_core::CoreEvent::GameFinished { victory: true }]);
        }
        td_core::DefenseEndTransitionState::StartStage { stage } => {
            raw_state.extend_events([td_core::CoreEvent::StageStarted { stage, card_count }]);
        }
        td_core::DefenseEndTransitionState::TreasureSelection => {}
    }
    game_state.restore_raw_core_projection(raw_state).ok()?;
    presentation_effect::apply_stage_end(game_state, completed_stage, raw_output.perfect_clear);
    if matches!(
        raw_output.transition,
        td_core::DefenseEndTransitionState::TreasureSelection
    ) {
        game_state.discover_treasure_options();
    }
    Some(raw_output)
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
    let base_damage = tower.calculate_projectile_damage(&[], crate::FixedRatio::ONE);
    let boosted_damage = tower.cached_upgrade_damage();
    assert_eq!(
        boosted_damage.ratio_of(base_damage),
        crate::FixedRatio::from_f64(expected_mul as f64).unwrap()
    );
}
