use crate::game_state::*;
use crate::game_state::effect::run_effect;

pub(crate) fn use_item(game_state: &mut GameState, item: &item::Item) {
    if game_state.stage_modifiers.is_item_use_disabled() {
        return;
    }

    game_state.item_used = true;
    run_effect(game_state, &item.effect);
    game_state.record_event(
        crate::game_state::play_history::HistoryEventType::ItemUsed {
            item: item.clone(),
        },
    );
}
