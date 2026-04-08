//! Item use strategies.

use super::ItemUseStrategy;
use crate::game_state::GameState;
use crate::game_state::effect::Effect;
use crate::game_state::item::ItemKind;

/// Default item use strategy with per-item rules:
/// - Heal: Use when damaged AND (max_hp < current_hp + heal_amount)
/// - Shield: Reserved for on_damage_taken (used before damage via pre-check)  
/// - Others: Don't use
pub struct DefaultItemUseStrategy;

impl ItemUseStrategy for DefaultItemUseStrategy {
    fn name(&self) -> &str {
        "default_item_use"
    }

    fn on_before_defense(&self, game_state: &mut GameState) {
        use_heal_if_needed(game_state);
    }

    fn on_damage_taken(&self, game_state: &mut GameState, _damage: f32) {
        // Try to use shield items
        use_shield_items(game_state);
        // Then try to heal
        use_heal_if_needed(game_state);
    }

    fn on_item_acquired(&self, game_state: &mut GameState) {
        use_heal_if_needed(game_state);
    }
}

/// Never uses any items.
pub struct NoItemUseStrategy;

impl ItemUseStrategy for NoItemUseStrategy {
    fn name(&self) -> &str {
        "no_item_use"
    }

    fn on_before_defense(&self, _game_state: &mut GameState) {}
    fn on_damage_taken(&self, _game_state: &mut GameState, _damage: f32) {}
    fn on_item_acquired(&self, _game_state: &mut GameState) {}
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
