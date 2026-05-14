use crate::shop::{Shop, ShopSlotId};
use crate::shop_panel::constants::{PADDING, SHOP_SLOT_WIDTH};
use namui::*;
use std::collections::HashMap;

pub struct SlotLayoutCalculator {
    pub items_area_wh: Wh<Px>,
}

impl SlotLayoutCalculator {
    #[inline]
    pub fn new(items_area_wh: Wh<Px>) -> Self {
        Self { items_area_wh }
    }

    pub fn calculate_positions(&self, shop: &Shop) -> (HashMap<ShopSlotId, Xy<Px>>, Wh<Px>) {
        let active_slots: Vec<_> = shop
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .collect();

        let mut positions = HashMap::new();
        let slot_count = active_slots.len();

        if slot_count == 0 {
            return (positions, Wh::zero());
        }

        let (slot_w, gap, start_x) = self.calculate_layout_params(slot_count);
        let slot_wh = Wh::new(slot_w, self.items_area_wh.height);

        for (active_index, slot_data) in active_slots.iter().enumerate() {
            let x = start_x + (slot_w + gap) * active_index as f32;
            let y = px(0.0);
            positions.insert(slot_data.id, Xy::new(x, y));
        }

        (positions, slot_wh)
    }

    #[inline]
    fn calculate_layout_params(&self, slot_count: usize) -> (Px, Px, Px) {
        let n = slot_count as f32;
        let slot_w = SHOP_SLOT_WIDTH.min(self.items_area_wh.width);
        let default_gap = PADDING;

        let gap = if slot_count > 1 {
            let total_with_default = slot_w * n + default_gap * (n - 1.0);
            if total_with_default > self.items_area_wh.width {
                (self.items_area_wh.width - slot_w * n) / (n - 1.0)
            } else {
                default_gap
            }
        } else {
            px(0.0)
        };

        let total_width = slot_w * n + gap * (n - 1.0);
        let start_x = (self.items_area_wh.width - total_width) / 2.0;

        (slot_w, gap, start_x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::effect::Effect;
    use crate::game_state::item::Item;
    use crate::shop::{Shop, ShopSlot, ShopSlotData};
    use namui::{Wh, px};

    fn make_dummy_slot() -> ShopSlotData {
        let item = Item {
            kind: crate::game_state::item::ItemKind::RiceBall,
            effect: Effect::Heal { amount: 0.0 },
        };
        ShopSlotData::new(ShopSlot::Item { item, cost: 0 })
    }

    #[test]
    fn calculator_handles_various_counts() {
        let calculator = SlotLayoutCalculator::new(Wh::new(px(300.0), px(100.0)));

        let shop = Shop { slots: vec![] };
        let (positions, wh) = calculator.calculate_positions(&shop);
        assert!(positions.is_empty());
        assert_eq!(wh, Wh::zero());

        let slot = make_dummy_slot();
        let mut shop = Shop {
            slots: vec![slot.clone()],
        };
        let (positions, wh) = calculator.calculate_positions(&shop);
        assert_eq!(positions.len(), 1);
        assert!(wh.width <= SHOP_SLOT_WIDTH);

        shop.slots = (0..5).map(|_| make_dummy_slot()).collect();
        let (positions, wh2) = calculator.calculate_positions(&shop);
        assert_eq!(positions.len(), 5);
        assert!(wh2.width <= SHOP_SLOT_WIDTH);
    }
}
