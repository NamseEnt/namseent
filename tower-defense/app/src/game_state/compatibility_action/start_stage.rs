use crate::game_state::{self, GameState};

pub(crate) fn apply_presentation_effects(
    game_state: &mut GameState,
    stage: usize,
    card_count: usize,
) {
    game_state.push_presentation_event(crate::game_state::PresentationEvent::PlayCardDrawSounds {
        card_count,
    });
    game_state.record_event(game_state::play_history::HistoryEventType::StageStart {
        stage,
        boss: game_state::is_boss_stage(stage),
    });
    game_state.discover_shop();
    game_state.push_presentation_event(crate::game_state::PresentationEvent::SaveDebugSnapshot);
}
