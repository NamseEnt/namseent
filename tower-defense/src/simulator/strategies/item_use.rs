//! Item use strategies.

use super::ItemUseStrategy;
use crate::game_state::GameState;
use crate::game_state::item::Item;

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
        let barricade_id = game_state.items.iter().find_map(|item| {
            if matches!(item.item, Item::GrantBarricades(..)) {
                Some(item.id)
            } else {
                None
            }
        });

        let Some(id) = barricade_id else {
            break;
        };

        game_state.action(crate::game_state::GameStateAction::UseInventoryItem(id));
    }
}

fn use_heal_if_needed(game_state: &mut GameState) {
    let max_hp = game_state.config.player.max_hp;

    loop {
        let heal_item_id = game_state.items.iter().find_map(|item| match item.item {
            Item::RiceBall(rice_ball) => {
                if game_state.hp + rice_ball.heal_amount > max_hp || game_state.hp < max_hp * 0.5 {
                    Some(item.id)
                } else {
                    None
                }
            }
            _ => None,
        });

        let Some(id) = heal_item_id else {
            break;
        };

        game_state.action(crate::game_state::GameStateAction::UseInventoryItem(id));
    }
}

fn use_shield_items(game_state: &mut GameState) {
    loop {
        let shield_id = game_state.items.iter().find_map(|item| {
            if matches!(item.item, Item::Shield(..)) {
                Some(item.id)
            } else {
                None
            }
        });

        let Some(id) = shield_id else {
            break;
        };

        game_state.action(crate::game_state::GameStateAction::UseInventoryItem(id));
    }
}
