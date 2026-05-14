use crate::game_state::{
    GameEffectEvent, GameState,
    flow::{DefenseFlow, GameFlow},
    monster_spawn::start_spawn,
};
use crate::sound;
use namui::Duration;

pub(super) fn set_defense_flow(game_state: &mut GameState) {
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
