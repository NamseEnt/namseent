use super::*;
use crate::game_state::play_history::HistoryEventType;
use crate::sound::{self, GameEndKind};

pub fn check_defense_end(game_state: &mut GameState) {
    let GameFlow::Defense(_) = game_state.flow else {
        return;
    };
    if !game_state.monster_spawn_state.is_idle() {
        return;
    }
    if !game_state.monsters.is_empty() {
        return;
    }

    #[cfg(feature = "debug-tools")]
    {
        if debug_tools::monster_hp_balance::get_balance_state().is_some() {
            debug_tools::monster_hp_balance::check_and_adjust_hp_balance(game_state);
            return;
        }
    }

    let completed_stage = game_state.stage;
    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        if !defense_flow.took_damage {
            game_state.record_event(HistoryEventType::StagePerfectClear {
                stage: completed_stage,
            });
            game_state.metrics.current_consecutive_perfect_clears += 1;
            game_state.metrics.max_consecutive_perfect_clears = game_state
                .metrics
                .max_consecutive_perfect_clears
                .max(game_state.metrics.current_consecutive_perfect_clears);
            game_state.upgrade_state.record_perfect_clear();
        } else {
            game_state.metrics.current_consecutive_perfect_clears = 0;
        }
    }

    if game_state.upgrade_state.has_piggy_bank() && game_state.gold >= 500 {
        game_state.earn_gold(50);
    }
    if game_state.upgrade_state.has_gift_box() {
        game_state.earn_gold(10 * game_state.items.len());
    }

    let is_boss_stage = is_boss_stage(game_state.stage);
    game_state.stage += 1;
    if game_state.stage > 50 {
        game_state.stage -= 1;
        sound::play_game_end_sound(GameEndKind::Victory);
        game_state.goto_result();
        return;
    }

    if is_boss_stage {
        game_state.goto_treasure_selection();
        return;
    }

    game_state.goto_next_stage();
}
