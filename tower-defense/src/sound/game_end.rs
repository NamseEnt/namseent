use super::{EmitSoundParams, SoundGroup, SpatialMode, VolumePreset, emit_sound};

#[derive(Clone, Copy)]
pub enum GameEndKind {
    Victory,
    Defeat,
}

pub fn play_game_end_sound(kind: GameEndKind) {
    let asset = match kind {
        GameEndKind::Victory => super::random_orch_hit(),
        GameEndKind::Defeat => super::random_fail(),
    };

    emit_sound(EmitSoundParams::one_shot(
        asset,
        SoundGroup::Sfx,
        VolumePreset::High,
        SpatialMode::NonSpatial,
    ));
}
