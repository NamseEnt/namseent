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

#[derive(Debug, Clone, Copy, State)]
pub struct ExitAnimation {
    pub start_time: Instant,
}

impl ExitAnimation {
    pub fn new(start_time: Instant) -> Self {
        Self { start_time }
    }

    pub fn is_complete(&self, current_time: Instant) -> bool {
        let elapsed = (current_time - self.start_time).as_secs_f32();
        elapsed >= 0.5 // 0.5초 후 완료
    }
}

#[derive(Clone, Debug, State)]
pub struct ShopSlotData {
    pub id: ShopSlotId,
    pub slot: ShopSlot,
    pub purchased: bool,
    pub exit_animation: Option<ExitAnimation>,
}

impl ShopSlotData {
    pub fn new(slot: ShopSlot) -> Self {
        Self {
            id: ShopSlotId::new(),
            slot,
            purchased: false,
            exit_animation: None,
        }
    }

    pub fn start_exit_animation(&mut self, now: Instant) {
        self.exit_animation = Some(ExitAnimation::new(now));
    }

    pub fn is_exit_animation_complete(&self, now: Instant) -> bool {
        if let Some(exit_anim) = self.exit_animation {
            exit_anim.is_complete(now)
        } else {
            false
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
