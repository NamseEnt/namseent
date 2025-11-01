mod shop_slot;

use crate::{
    game_state::{
        GameState, contract::generation::generate_contract, flow::GameFlow,
        item::generation::generate_item, upgrade::generate_upgrade,
    },
    rarity::Rarity,
    *,
};
use namui::OneZero;
use rand::{Rng, thread_rng};
pub use shop_slot::*;

#[derive(Clone, Debug, State)]
pub struct Shop {
    pub slots: Vec<ShopSlotData>,
}

impl Shop {
    pub fn new(game_state: &GameState) -> Self {
        let slots = (0..game_state.max_shop_slot())
            .map(|_| ShopSlotData::new(generate_shop_slot(game_state)))
            .collect();
        Self { slots }
    }

    pub fn get_slot_by_id(&self, id: ShopSlotId) -> Option<&ShopSlotData> {
        self.slots.iter().find(|slot| slot.id == id)
    }

    pub fn get_slot_by_id_mut(&mut self, id: ShopSlotId) -> Option<&mut ShopSlotData> {
        self.slots.iter_mut().find(|slot| slot.id == id)
    }

    pub fn remove_completed_exit_animations(&mut self) {
        let now = Instant::now();
        // 완료된 exit 애니메이션이 있는 슬롯 제거
        self.slots
            .retain(|slot| !slot.is_exit_animation_complete(now));
    }

    pub fn update(&mut self) {
        self.remove_completed_exit_animations();
    }
}

pub fn refresh_shop(game_state: &mut GameState) {
    let new_slots_count = game_state.max_shop_slot();
    let new_slots: Vec<ShopSlot> = (0..new_slots_count)
        .map(|_| generate_shop_slot(game_state))
        .collect();

    let GameFlow::SelectingTower(flow) = &mut game_state.flow else {
        unreachable!()
    };

    // 기존 슬롯 중 구매되지 않은 것만 새로 고침
    let mut new_slot_iter = new_slots.into_iter();
    for slot_data in flow.shop.slots.iter_mut() {
        if !slot_data.purchased
            && let Some(new_slot) = new_slot_iter.next()
        {
            slot_data.slot = new_slot;
        }
    }

    // 슬롯 수가 늘어난 경우 새 슬롯 추가
    while flow.shop.slots.len() < new_slots_count {
        if let Some(new_slot) = new_slot_iter.next() {
            flow.shop.slots.push(ShopSlotData::new(new_slot));
        }
    }

    // 슬롯 수가 줄어든 경우 초과 슬롯 제거 (구매되지 않은 것부터)
    while flow.shop.slots.len() > new_slots_count {
        if let Some(index) = flow.shop.slots.iter().position(|s| !s.purchased) {
            flow.shop.slots.remove(index);
        } else {
            break;
        }
    }
}

fn generate_shop_slot(game_state: &GameState) -> ShopSlot {
    let slot_type = thread_rng().gen_range(0..10);
    let rarity = game_state.generate_rarity(Default::default());

    match slot_type {
        0..=2 => {
            // Item (3/10)
            let item = generate_item(rarity);
            let cost = item_cost(
                rarity,
                item.value,
                game_state.upgrade_state.shop_item_price_minus,
            );
            ShopSlot::Item { item, cost }
        }
        3..=7 => {
            // Upgrade (5/10)
            let upgrade = generate_upgrade(game_state, rarity);
            let cost = item_cost(
                rarity,
                upgrade.value,
                game_state.upgrade_state.shop_item_price_minus,
            );
            ShopSlot::Upgrade { upgrade, cost }
        }
        8..=9 => {
            // Contract (2/10)
            let contract = generate_contract(rarity);
            let cost = item_cost(
                rarity,
                0.5.into(), // 임시로 0.5 사용, contract에 value가 없으므로
                game_state.upgrade_state.shop_item_price_minus,
            );
            ShopSlot::Contract { contract, cost }
        }
        _ => unreachable!(),
    }
}

fn item_cost(rarity: Rarity, value: OneZero, discount: usize) -> usize {
    let base_cost = match rarity {
        crate::rarity::Rarity::Common => 25.0,
        crate::rarity::Rarity::Rare => 50.0,
        crate::rarity::Rarity::Epic => 75.0,
        crate::rarity::Rarity::Legendary => 100.0,
    };
    let additional_cost = value.as_f32() * base_cost * 0.5;
    let cost = base_cost + additional_cost - discount as f32;
    cost.max(0.0) as usize
}
