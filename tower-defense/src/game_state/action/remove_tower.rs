use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, *};

pub(crate) fn remove_tower(game_state: &mut GameState, tower_id: usize) -> bool {
    let tower_count_before = game_state.towers.iter().count();
    let removed_tower_left_top = game_state
        .towers
        .iter()
        .find(|tower| tower.id() == tower_id)
        .map(|tower| tower.left_top);
    let tower_center_xy = game_state
        .towers
        .iter()
        .find(|tower| tower.id() == tower_id)
        .map(|tower| {
            let center = tower.center_xy_f32();
            (center.x, center.y)
        });
    game_state.towers.remove_tower(tower_id);
    let tower_removed = game_state.towers.iter().count() < tower_count_before;
    if tower_removed {
        game_state.route = calculate_routes(&game_state.towers.coords(), &TRAVEL_POINTS, MAP_SIZE)
            .expect("route should exist after removing a tower");
        game_state.handle_upgrade_trigger(UpgradeTriggerEvent::TowerRemoved);
        if let Some(left_top) = removed_tower_left_top {
            game_state.record_event(
                crate::game_state::play_history::HistoryEventType::TowerRemoved { left_top },
            );
        }
        if let Some(center_xy) = tower_center_xy {
            game_state
                .effect_events
                .push(GameEffectEvent::SpawnTowerRemoveDustBurst(
                    center_xy,
                    game_state.now(),
                ));
        }
    }

    tower_removed
}
