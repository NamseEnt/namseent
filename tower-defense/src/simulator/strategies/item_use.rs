//! Item use strategies.

use super::ItemUseStrategy;
use crate::game_state::item::Item;
use crate::game_state::{GameState, PlayerCommand};
use crate::{Damage, Health};

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

    fn on_damage_taken(&self, game_state: &mut GameState, _damage: Damage) {
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
        let rubber_cone_index = game_state.items.iter().position(|item| {
            if matches!(item.item, Item::RubberCone(..)) {
                true
            } else {
                false
            }
        });

        let Some(item_index) = rubber_cone_index else {
            break;
        };

        if game_state
            .apply_player_command(PlayerCommand::UseInventoryItem { item_index })
            .is_err()
        {
            break;
        }
    }
}

fn use_heal_if_needed(game_state: &mut GameState) {
    loop {
        let heal_item_index = game_state
            .items
            .iter()
            .enumerate()
            .find_map(|(index, item)| {
                let max_hp = game_state.max_hp();
                let heal_amount = item_heal_amount(&item.item)?;
                if game_state.hp.saturating_add(heal_amount) > max_hp
                    || game_state.hp < max_hp.scaled_by(crate::FixedRatio::from_raw(500_000))
                {
                    Some(index)
                } else {
                    None
                }
            });

        let Some(item_index) = heal_item_index else {
            break;
        };

        if game_state
            .apply_player_command(PlayerCommand::UseInventoryItem { item_index })
            .is_err()
        {
            break;
        }
    }
}

fn item_heal_amount(item: &Item) -> Option<Health> {
    match item {
        Item::Bread(bread) => Some(bread.heal_amount),
        Item::Gimbap(gimbap) => Some(gimbap.heal_amount),
        Item::LunchBox(lunch_box) => Some(lunch_box.heal_amount),
        Item::Candy(candy) => Some(candy.heal_amount),
        Item::Cannoli(cannoli) => Some(cannoli.heal_amount),
        Item::Cookie(cookie) => Some(cookie.heal_amount),
        Item::Donut(donut) => Some(donut.heal_amount),
        Item::RiceBall(rice_ball) => Some(rice_ball.heal_amount),
        _ => None,
    }
}
fn use_shield_items(game_state: &mut GameState) {
    loop {
        let shield_index = game_state.items.iter().position(|item| {
            if matches!(
                item.item,
                Item::Bread(..)
                    | Item::Gimbap(..)
                    | Item::LunchBox(..)
                    | Item::Milk(..)
                    | Item::RiceBall(..)
            ) {
                true
            } else {
                false
            }
        });

        let Some(item_index) = shield_index else {
            break;
        };

        if game_state
            .apply_player_command(PlayerCommand::UseInventoryItem { item_index })
            .is_err()
        {
            break;
        }
    }
}
