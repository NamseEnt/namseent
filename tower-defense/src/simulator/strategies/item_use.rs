//! Item use strategies.

use super::ItemUseStrategy;
use crate::game_state::GameState;
use crate::game_state::effect::Effect;
use crate::game_state::item::ItemKind;

/// Heuristic item use strategy that immediately uses barricade grant items and preserves heal/shield.
pub struct HeuristicItemUseStrategy;

impl ItemUseStrategy for HeuristicItemUseStrategy {
    fn name(&self) -> &str {
        "smart_item_use"
    }

    fn on_before_defense(&self, game_state: &mut GameState) {
        use_grant_barricades(game_state);
        use_heal_if_needed(game_state);
    }

    fn on_damage_taken(&self, game_state: &mut GameState, _damage: f32) {
        use_shield_items(game_state);
        use_heal_if_needed(game_state);
    }

    fn on_item_acquired(&self, game_state: &mut GameState) {
        use_grant_barricades(game_state);
        use_heal_if_needed(game_state);
    }
}

fn use_grant_barricades(game_state: &mut GameState) {
    loop {
        let barricade_idx = game_state
            .items
            .iter()
            .position(|item| matches!(item.kind, ItemKind::GrantBarricades));

        let Some(idx) = barricade_idx else {
            break;
        };

        let item = game_state.items.remove(idx);
        game_state.use_item(&item);
    }
}

fn use_heal_if_needed(game_state: &mut GameState) {
    let max_hp = game_state.config.player.max_hp;

    loop {
        let heal_item_idx = game_state.items.iter().position(|item| {
            matches!(item.kind, ItemKind::RiceCake)
                && match &item.effect {
                    Effect::Heal { amount } => {
                        game_state.hp + amount > max_hp || game_state.hp < max_hp * 0.5
                    }
                    _ => false,
                }
        });

        let Some(idx) = heal_item_idx else {
            break;
        };

        let item = game_state.items.remove(idx);
        game_state.use_item(&item);
    }
}

fn use_shield_items(game_state: &mut GameState) {
    loop {
        let shield_idx = game_state
            .items
            .iter()
            .position(|item| matches!(item.kind, ItemKind::Shield));

        let Some(idx) = shield_idx else {
            break;
        };

        let item = game_state.items.remove(idx);
        game_state.use_item(&item);
    }
}
