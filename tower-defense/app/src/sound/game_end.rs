use super::{EmitSoundParams, SoundGroup, SpatialMode, VolumePreset, emit_sound_after_at};
use crate::PresentationInstant;

#[derive(Clone, Copy)]
pub enum GameEndKind {
    Victory,
    Defeat,
}

pub fn play_game_end_sound(kind: GameEndKind) {
    play_game_end_sound_at(kind, PresentationInstant::capture());
}

pub fn play_game_end_sound_at(kind: GameEndKind, presentation_instant: PresentationInstant) {
    let asset = match kind {
        GameEndKind::Victory => super::random_orch_hit(),
        GameEndKind::Defeat => super::random_fail(),
    };

    emit_sound_after_at(
        EmitSoundParams::one_shot(
            asset,
            SoundGroup::Sfx,
            VolumePreset::High,
            SpatialMode::NonSpatial,
        ),
        std::time::Duration::ZERO.into(),
        presentation_instant,
    );
}
