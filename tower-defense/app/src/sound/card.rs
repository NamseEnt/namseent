use super::{
    EmitSoundParams, SoundGroup, SpatialMode, VolumePreset, emit_sound_after, random_pick_up_cards,
};
use namui::*;
use rand::Rng;

const DRAW_SOUND_REPEAT_MAX: usize = 5;
const DRAW_SOUND_INTERVAL_MIN_MS: i64 = 25;
const DRAW_SOUND_INTERVAL_MAX_MS: i64 = 100;

pub fn play_card_draw_sounds(card_count: usize) {
    let play_count = card_count.min(DRAW_SOUND_REPEAT_MAX);
    if play_count == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let mut accumulated_delay_ms = 0i64;

    for index in 0..play_count {
        play_pick_up_cards_after(Duration::from_millis(accumulated_delay_ms));

        if index + 1 < play_count {
            accumulated_delay_ms +=
                rng.gen_range(DRAW_SOUND_INTERVAL_MIN_MS..=DRAW_SOUND_INTERVAL_MAX_MS);
        }
    }
}

pub fn play_card_selected_sound() {
    play_pick_up_cards_after(Duration::ZERO);
}

pub fn play_card_deselected_sound() {
    play_pick_up_cards_after(Duration::ZERO);
}

fn play_pick_up_cards_after(delay: Duration) {
    emit_sound_after(
        EmitSoundParams::one_shot(
            random_pick_up_cards(),
            SoundGroup::Ui,
            VolumePreset::Medium,
            SpatialMode::NonSpatial,
        ),
        delay,
    );
}
