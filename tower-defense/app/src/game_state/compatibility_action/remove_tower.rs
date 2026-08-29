use crate::game_state::*;

pub(crate) fn play_removal_sound(game_state: &mut GameState) {
    game_state.push_presentation_event(PresentationEvent::PlaySoundCue {
        cue: SoundCue::LuggageDrop,
        position: None,
        volume: SoundVolume::High,
        max_duration_ms: None,
    });
}

pub(crate) fn record_history_event(game_state: &mut GameState, tower_id: TowerId) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::TowerRemovedById { tower_id },
    );
}
