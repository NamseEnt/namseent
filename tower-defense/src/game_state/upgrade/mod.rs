mod behavior;
mod generation;
mod state;
mod thumbnail;

pub use behavior::*;
pub use generation::*;
pub use state::*;

pub const MAX_GOLD_EARN_PLUS: usize = 16;
pub const MAX_SHOP_SLOT_EXPAND: usize = 2;
pub const MAX_DICE_CHANCE_PLUS: usize = 4;
pub const MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE: usize = 15;
pub const MAX_REMOVE_NUMBER_RANKS: usize = 5;
const TROPHY_DAMAGE_MULTIPLIER: f32 = 2.0;
