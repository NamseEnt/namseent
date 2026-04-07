//! Shop strategies.

use super::ShopStrategy;
use crate::game_state::GameState;
use crate::game_state::flow::GameFlow;
use rand::RngCore;

/// Does not buy anything from the shop.
pub struct NoBuyStrategy;

impl ShopStrategy for NoBuyStrategy {
    fn name(&self) -> &str {
        "no_buy"
    }

    fn execute_shop(&self, _game_state: &mut GameState, _rng: &mut dyn RngCore) {
        // Intentionally do nothing
    }
}

/// Buys the cheapest affordable item from the shop.
pub struct BuyCheapestStrategy;

impl ShopStrategy for BuyCheapestStrategy {
    fn name(&self) -> &str {
        "buy_cheapest"
    }

    fn execute_shop(&self, game_state: &mut GameState, _rng: &mut dyn RngCore) {
        loop {
            let cheapest_slot_id = {
                let GameFlow::SelectingTower(flow) = &game_state.flow else {
                    return;
                };

                let mut cheapest: Option<(crate::shop::ShopSlotId, usize)> = None;
                for slot in &flow.shop.slots {
                    if slot.purchased || slot.exit_animation.is_some() {
                        continue;
                    }
                    let cost = match &slot.slot {
                        crate::shop::ShopSlot::Item { cost, .. } => *cost,
                        crate::shop::ShopSlot::Upgrade { cost, .. } => *cost,
                    };
                    if cost <= game_state.gold {
                        if cheapest.is_none() || cost < cheapest.unwrap().1 {
                            cheapest = Some((slot.id, cost));
                        }
                    }
                }
                cheapest.map(|(id, _)| id)
            };

            match cheapest_slot_id {
                Some(slot_id) => game_state.purchase_shop_item(slot_id),
                None => return,
            }
        }
    }
}
