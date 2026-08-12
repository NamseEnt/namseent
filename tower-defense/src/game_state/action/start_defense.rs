use crate::game_state::{
    GameEffectEvent, GameState,
    flow::{DefenseFlow, GameFlow},
    monster_spawn::start_spawn,
};
use crate::sound;
use namui::Duration;

pub(super) fn discard_unplaced_towers(game_state: &mut GameState) {
    let tower_slot_ids = game_state.hand.active_slot_ids();
    if !tower_slot_ids.is_empty() {
        game_state.hand.delete_slots(&tower_slot_ids);
    }
}

pub(super) fn set_defense_flow(game_state: &mut GameState) {
    if matches!(game_state.flow, GameFlow::PlacingTower) {
        discard_unplaced_towers(game_state);
    }
    game_state.flow = GameFlow::Defense(DefenseFlow::new(game_state));
}

pub(super) fn play_fanfare_sound(game_state: &mut GameState) {
    game_state.effect_events.push(GameEffectEvent::PlaySound(
        sound::EmitSoundParams::one_shot(
            sound::random_trumpet_fanfares(),
            sound::SoundGroup::Ui,
            sound::VolumePreset::High,
            sound::SpatialMode::NonSpatial,
        )
        .with_max_duration(Duration::from_secs(6)),
    ));
}

pub(super) fn begin_monster_spawn(game_state: &mut GameState) {
    start_spawn(game_state);
}
