use super::{EmitSoundParams, SoundGroup, SpatialMode, VolumePreset, emit_sound};

pub fn play_coin_sound_for_gold() {
    emit_sound(EmitSoundParams::one_shot(
        super::random_coin_sounds(),
        SoundGroup::Ui,
        VolumePreset::High,
        SpatialMode::NonSpatial,
    ));
}

pub fn play_coin_sound_for_gold_at(presentation_instant: crate::PresentationInstant) {
    super::emit_sound_after_at(
        EmitSoundParams::one_shot(
            super::random_coin_sounds(),
            SoundGroup::Ui,
            VolumePreset::High,
            SpatialMode::NonSpatial,
        ),
        namui::Duration::ZERO,
        presentation_instant,
    );
}
