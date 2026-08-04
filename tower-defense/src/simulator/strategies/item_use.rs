//! Item use strategies.

use super::ItemUseStrategy;
use crate::game_state::GameState;
use crate::game_state::item::Item;

/// Heuristic item use strategy that immediately uses rubber cone items and preserves heal/shield.
pub struct HeuristicItemUseStrategy;

impl ItemUseStrategy for HeuristicItemUseStrategy {
    fn name(&self) -> &str {
        "smart_item_use"
    }

    fn on_before_defense(&self, game_state: &mut GameState) {
        use_rubber_cone(game_state);
        use_heal_if_needed(game_state);
    }

    fn on_damage_taken(&self, game_state: &mut GameState, _damage: f32) {
        use_shield_items(game_state);
        use_heal_if_needed(game_state);
    }

    fn on_item_acquired(&self, game_state: &mut GameState) {
        use_rubber_cone(game_state);
        use_heal_if_needed(game_state);
    }
}

fn use_rubber_cone(game_state: &mut GameState) {
    loop {
        let rubber_cone_id = game_state.items.iter().find_map(|item| {
            if matches!(item.item, Item::RubberCone(..)) {
                Some(item.id)
            } else {
                None
            }
        });

        let Some(id) = rubber_cone_id else {
            break;
        };

        game_state.action(crate::game_state::GameStateAction::UseInventoryItem(id));
    }
}

fn use_heal_if_needed(game_state: &mut GameState) {
    let max_hp = game_state.config.player.max_hp;

    loop {
        let heal_item_id = game_state.items.iter().find_map(|item| {
            let heal_amount = item_heal_amount(&item.item)?;
            if game_state.hp + heal_amount > max_hp || game_state.hp < max_hp * 0.5 {
                Some(item.id)
            } else {
                None
            }
        });

        let Some(id) = heal_item_id else {
            break;
        };

        game_state.action(crate::game_state::GameStateAction::UseInventoryItem(id));
    }
}

fn item_heal_amount(item: &Item) -> Option<f32> {
    match item {
        Item::Bread(bread) => Some(bread.heal_amount),
        Item::Gimbap(gimbap) => Some(gimbap.heal_amount),
        Item::LunchBox(lunch_box) => Some(lunch_box.heal_amount),
        Item::Candy(candy) => Some(candy.heal_amount),
        Item::Cannoli(cannoli) => Some(cannoli.heal_amount),
        Item::Cookie(cookie) => Some(cookie.heal_amount),
        Item::Donut(donut) => Some(donut.heal_amount),
        Item::RiceBall(rice_ball) => Some(rice_ball.heal_amount),
        Item::Strawberry(strawberry) => Some(strawberry.heal_amount),
        _ => None,
    }
}

fn use_shield_items(game_state: &mut GameState) {
    loop {
        let shield_id = game_state.items.iter().find_map(|item| {
            if matches!(
                item.item,
                Item::Bread(..)
                    | Item::Gimbap(..)
                    | Item::LunchBox(..)
                    | Item::Milk(..)
                    | Item::RiceBall(..)
            ) {
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
