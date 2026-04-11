//! Auto-play strategy traits for headless simulation.

pub mod card_reroll;
pub mod item_use;
pub mod shop;
pub mod tower_placement;
pub mod treasure;

pub use card_reroll::{ItemAwareRerollStrategy, SmartRerollStrategy};
pub use item_use::HeuristicItemUseStrategy;
pub use shop::SynergyShopStrategy;
pub use treasure::{SynergyTreasureStrategy, TreasureStrategy};

use crate::game_state::GameState;
use rand::RngCore;

/// Strategy for shop interaction (buy items/upgrades, reroll shop).
pub trait ShopStrategy: Send + Sync {
    fn name(&self) -> &str;
    /// Called once per stage when shop is available.
    /// May purchase items and reroll the shop.
    fn execute_shop(&self, game_state: &mut GameState, rng: &mut dyn RngCore);
}

/// Strategy for card selection and rerolling.
pub trait CardRerollStrategy: Send + Sync {
    fn name(&self) -> &str;
    /// Called when the player has cards in hand and dice to reroll.
    /// Decides which rerolls to make and which tower to select.
    fn execute_card_selection(&self, game_state: &mut GameState, rng: &mut dyn RngCore);
}

/// Strategy for placing towers on the map.
pub trait TowerPlacementStrategy: Send + Sync {
    fn name(&self) -> &str;
    /// Called when entering PlacingTower flow. Places all available towers.
    fn execute_placement(&self, game_state: &mut GameState);
}

/// Strategy for using items during gameplay.
pub trait ItemUseStrategy: Send + Sync {
    fn name(&self) -> &str;
    /// Called at strategic moments (before defense, when damaged, etc.).
    fn on_before_defense(&self, game_state: &mut GameState);
    /// Called when player takes damage.
    fn on_damage_taken(&self, game_state: &mut GameState, damage: f32);
    /// Called when a new item is acquired.
    fn on_item_acquired(&self, game_state: &mut GameState);
}
