use super::*;
use crate::game_state::GameStateAction;
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

    let perfect_clear = if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        !defense_flow.took_damage
    } else {
        false
    };

    let gold = game_state.gold;
    let item_count = game_state.items.len();
    game_state.action(GameStateAction::StageEnd {
        perfect_clear,
        gold,
        item_count,
    });

    let is_boss_stage = is_boss_stage(game_state.stage);
    game_state.stage += 1;
    let max_stage = game_state.config.player.max_stages;
    if game_state.stage > max_stage {
        game_state.stage -= 1;
        sound::play_game_end_sound(GameEndKind::Victory);
        game_state.action(crate::game_state::GameStateAction::GameOver);
        return;
    }

    if is_boss_stage {
        game_state.action(crate::game_state::GameStateAction::StartTreasureSelection);
        return;
    }

    game_state.action(crate::game_state::GameStateAction::StartStage {
        stage: game_state.stage,
    });
}
