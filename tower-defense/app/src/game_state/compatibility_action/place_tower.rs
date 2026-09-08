use crate::game_state::{
    card::{Rank, Suit},
    *,
};

pub(crate) fn record_history_event(game_state: &mut GameState, tower: &Tower) {
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::TowerPlaced {
            tower_kind: tower.kind,
            rank: tower.rank().unwrap_or(Rank::Ace),
            suit: tower.suit().unwrap_or(Suit::Spades),
            left_top: tower.left_top,
        },
    );
}

pub(crate) fn play_placement_sound(game_state: &mut GameState) {
    game_state.push_presentation_event(PresentationEvent::PlaySoundCue {
        cue: SoundCue::LuggageDrop,
        position: None,
        volume: SoundVolume::High,
        max_duration_ms: None,
    });
}
