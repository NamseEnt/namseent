mod behavior;
mod generation;
mod state;
#[cfg(test)]
mod tests;
mod thumbnail;
mod tower;

pub use behavior::*;
pub use generation::*;
pub use state::*;
pub use tower::*;

pub const MAX_SHOP_SLOT_EXPAND: usize = 2;
pub const MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE: usize = 15;
