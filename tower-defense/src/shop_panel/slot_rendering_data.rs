use crate::shop::{Shop, ShopSlotId};
use namui::*;
use std::collections::HashMap;

pub struct SlotRenderingData<'a> {
    pub active_slots: Vec<&'a crate::shop::ShopSlotData>,
    pub exiting_slots: Vec<&'a crate::shop::ShopSlotData>,
    pub slot_positions: HashMap<ShopSlotId, Xy<Px>>,
}

impl<'a> SlotRenderingData<'a> {
    pub fn from_shop(shop: &'a Shop, slot_positions: HashMap<ShopSlotId, Xy<Px>>) -> Self {
        let (active_slots, exiting_slots): (Vec<_>, Vec<_>) = shop
            .slots
            .iter()
            .partition(|slot| slot.exit_animation.is_none());

        Self {
            active_slots,
            exiting_slots,
            slot_positions,
        }
    }

    pub fn get_position(&self, slot_id: ShopSlotId) -> Option<Xy<Px>> {
        self.slot_positions.get(&slot_id).copied()
    }
}
