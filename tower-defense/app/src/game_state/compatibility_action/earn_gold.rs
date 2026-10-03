use crate::game_state::*;

const GOLD_SOUND_INTERVAL_MS: i64 = 100;
const MAX_GOLD_SOUND_COUNT: usize = 10;

pub(crate) fn play_earn_sound(game_state: &mut GameState, amount: usize) {
    if amount == 0 {
        return;
    }

    for sound_index in 0..amount.min(MAX_GOLD_SOUND_COUNT) {
        game_state.push_presentation_event(PresentationEvent::PlaySoundCueDelayed {
            cue: SoundCue::Coin,
            position: None,
            volume: SoundVolume::High,
            delay_ms: sound_index as i64 * GOLD_SOUND_INTERVAL_MS,
        });
    }
}
