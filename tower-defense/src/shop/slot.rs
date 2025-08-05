use crate::game_state::item::Item;

#[derive(Default, Clone)]
pub enum ShopSlot {
    #[default]
    Locked,
    Item {
        item: Item,
        cost: usize,
        purchased: bool,
    },
}
