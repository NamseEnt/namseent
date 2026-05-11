use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, upgrade::UpgradeState, *};

pub(super) fn prepare_tower_stats(tower: &mut Tower, upgrade_state: &UpgradeState) {
    tower.refresh_status_effects_from_template();
    tower.refresh_cached_upgrade_damage(
        upgrade_state.revision,
        &upgrade_state.tower_upgrade_damage_bonuses(),
    );
}
pub(super) fn place_tower(game_state: &mut GameState, tower: &Tower) -> bool {
    game_state.towers.place_tower(tower.clone())
}

pub(super) fn record_history_event(game_state: &mut GameState, tower: &Tower) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::TowerPlaced {
            tower_kind: tower.kind,
            rank: tower.rank,
            suit: tower.suit,
            left_top: tower.left_top,
        },
    );
}

pub(super) fn trigger_upgrades(game_state: &mut GameState, tower: &Tower) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::TowerPlaced { tower });
}

pub(super) fn play_placement_sound(game_state: &mut GameState) {
    game_state.effect_events.push(GameEffectEvent::PlaySound(
        sound::EmitSoundParams::one_shot(
            sound::random_luggage_drop(),
            sound::SoundGroup::Sfx,
            sound::VolumePreset::High,
            sound::SpatialMode::NonSpatial,
        ),
    ));
}

pub(super) fn recalculate_route(game_state: &mut GameState) {
    game_state.route =
        calculate_routes(&game_state.towers.coords(), &TRAVEL_POINTS, MAP_SIZE).unwrap();
}
