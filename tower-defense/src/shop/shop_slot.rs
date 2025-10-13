use crate::{
    game_state::{contract::Contract, item::Item, upgrade::Upgrade},
    *,
};

#[derive(Debug, Default, Clone, State)]
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
    Contract {
        contract: Contract,
        cost: usize,
        purchased: bool,
    },
}
