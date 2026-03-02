use super::{EmitSoundParams, SoundGroup, SpatialMode, VolumePreset, emit_sound};

pub fn play_coin_sound_for_gold() {
    emit_sound(EmitSoundParams::one_shot(
        super::random_coin_sounds(),
        SoundGroup::Ui,
        VolumePreset::High,
        SpatialMode::NonSpatial,
    ));
}
