use crate::game_state::{item::Item, upgrade::Upgrade};

#[derive(Debug, Default, Clone)]
pub enum ShopSlot {
    #[default]
    Locked,
    Item {
        item: Item,
        cost: usize,
        purchased: bool,
    },
    Upgrade {
        upgrade: Upgrade,
        cost: usize,
        purchased: bool,
    },
}
