use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, *};

pub(super) fn remove_tower(game_state: &mut GameState, tower_id: usize) -> bool {
    game_state.towers.remove_tower(tower_id)
}

pub(super) fn recalculate_route(game_state: &mut GameState) {
    game_state.route = calculate_routes(&game_state.towers.coords(), &TRAVEL_POINTS, MAP_SIZE)
        .expect("route should exist after removing a tower");
}

pub(super) fn trigger_upgrades(game_state: &mut GameState) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::TowerRemoved);
}

pub(super) fn play_removal_sound(game_state: &mut GameState) {
    game_state.effect_events.push(GameEffectEvent::PlaySound(
        sound::EmitSoundParams::one_shot(
            sound::random_luggage_drop(),
            sound::SoundGroup::Sfx,
            sound::VolumePreset::High,
            sound::SpatialMode::NonSpatial,
        ),
    ));
}

pub(super) fn record_history_event(game_state: &mut GameState, tower_id: usize) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::TowerRemovedById { tower_id },
    );
}
