use crate::shop::{Shop, ShopSlotId};
use namui::*;
use std::collections::HashMap;

/// Precomputed data needed when rendering the shop slots.  Separates active
/// slots from those that are currently playing an exit animation and stores a
/// copy of the position map so the rendering code doesn’t have to recompute it.
pub struct SlotRenderingData<'a> {
    /// Slots that are still visible / interactive.
    pub active_slots: Vec<&'a crate::shop::ShopSlotData>,
    /// Slots that are animating out; still rendered but treated specially.
    pub exiting_slots: Vec<&'a crate::shop::ShopSlotData>,
    /// Current target positions for each slot ID.
    pub slot_positions: HashMap<ShopSlotId, Xy<Px>>,
}

impl<'a> SlotRenderingData<'a> {
    /// Build a `SlotRenderingData` from the given shop and precomputed
    /// positions map.  The map is *moved* in; callers should clone if necessary.
    #[inline]
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

    /// Return the target position for `slot_id` if it exists.
    #[inline]
    pub fn get_position(&self, slot_id: ShopSlotId) -> Option<Xy<Px>> {
        self.slot_positions.get(&slot_id).copied()
    }
}
