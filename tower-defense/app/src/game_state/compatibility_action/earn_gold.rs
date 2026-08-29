use crate::game_state::*;

pub(crate) fn play_earn_sound(game_state: &mut GameState, amount: usize) {
    if amount == 0 {
        return;
    }
    game_state.push_presentation_event(PresentationEvent::PlaySoundCue {
        cue: SoundCue::Coin,
        position: None,
        volume: SoundVolume::High,
        max_duration_ms: None,
    });
}
