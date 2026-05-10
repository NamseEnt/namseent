use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, *};

pub(crate) fn place_tower(game_state: &mut GameState, mut tower: Box<Tower>) {
    let rank = tower.rank;
    let suit = tower.suit;
    let hand = tower.kind;
    let left_top = tower.left_top;

    tower.refresh_status_effects_from_template();
    tower.refresh_cached_upgrade_damage(
        game_state.upgrade_state.revision,
        &game_state.upgrade_state.tower_upgrade_damage_bonuses(),
    );

    let tower_count_before = game_state.towers.iter().count();
    game_state.towers.place_tower(*tower.clone());
    game_state.route =
        calculate_routes(&game_state.towers.coords(), &TRAVEL_POINTS, MAP_SIZE).unwrap();

    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::TowerPlaced {
            tower_kind: hand,
            rank,
            suit,
            left_top,
        },
    );

    let tower_placed = game_state.towers.iter().count() > tower_count_before;
    if tower_placed {
        game_state.handle_upgrade_trigger(UpgradeTriggerEvent::TowerPlaced { tower: &tower });

        game_state.effect_events.push(GameEffectEvent::PlaySound(
            sound::EmitSoundParams::one_shot(
                sound::random_luggage_drop(),
                sound::SoundGroup::Sfx,
                sound::VolumePreset::High,
                sound::SpatialMode::NonSpatial,
            ),
        ));
    }
}
