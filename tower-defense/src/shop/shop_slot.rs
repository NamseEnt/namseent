use crate::{
    game_state::{contract::Contract, item::Item, upgrade::Upgrade},
    *,
};
use std::sync::atomic::AtomicUsize;

static SHOP_SLOT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, State)]
pub struct ShopSlotId(usize);
impl ShopSlotId {
    pub fn new() -> Self {
        let id = SHOP_SLOT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(id)
    }
}
impl Default for ShopSlotId {
    fn default() -> Self {
        Self::new()
    }
}
impl From<ShopSlotId> for AddKey {
    fn from(val: ShopSlotId) -> Self {
        AddKey::U128(val.0 as u128)
    }
}

#[derive(Clone, Debug, State)]
pub struct ShopSlotData {
    pub id: ShopSlotId,
    pub slot: ShopSlot,
    pub purchased: bool,
}

impl ShopSlotData {
    pub fn new(slot: ShopSlot) -> Self {
        Self {
            id: ShopSlotId::new(),
            slot,
            purchased: false,
        }
    }
}

#[derive(Debug, Default, Clone, State)]
pub enum ShopSlot {
    #[default]
    Locked,
    Item {
        item: Item,
        cost: usize,
    },
    Upgrade {
        upgrade: Upgrade,
        cost: usize,
    },
    Contract {
        contract: Contract,
        cost: usize,
    },
}
