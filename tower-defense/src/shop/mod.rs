mod shop_slot;

use crate::{
    game_state::{
        GameState,
        flow::GameFlow,
        upgrade::{generate_tower_damage_upgrade, generate_treasure_upgrade},
    },
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

    pub fn new_treasure(game_state: &GameState) -> Self {
        let slots = (0..3)
            .map(|_| ShopSlotData::new(generate_treasure_slot(game_state)))
            .collect();
        Self { slots }
    }

    pub fn get_slot_by_id(&self, id: ShopSlotId) -> Option<&ShopSlotData> {
        self.slots.iter().find(|slot| slot.id == id)
    }

    pub fn get_slot_by_id_mut(&mut self, id: ShopSlotId) -> Option<&mut ShopSlotData> {
        self.slots.iter_mut().find(|slot| slot.id == id)
    }

    pub fn delete_slots(&mut self, ids: &[ShopSlotId]) {
        let now = Instant::now();
        for slot in self.slots.iter_mut() {
            if ids.contains(&slot.id) {
                slot.start_exit_animation(now);
            }
        }
    }

    pub fn get_unpurchased_slot_ids(&self) -> Vec<ShopSlotId> {
        self.slots
            .iter()
            .filter(|slot| !slot.purchased && slot.exit_animation.is_none())
            .map(|slot| slot.id)
            .collect()
    }

    pub fn push(&mut self, slot: ShopSlot) {
        self.slots.push(ShopSlotData::new(slot));
    }

    pub fn remove_completed_exit_animations(&mut self) {
        let now = Instant::now();
        self.slots
            .retain(|slot| !slot.is_exit_animation_complete(now));
    }

    pub fn update(&mut self) {
        self.remove_completed_exit_animations();
    }
}

pub fn refresh_shop(game_state: &mut GameState) {
    let (unpurchased_slot_ids, refresh_count) =
        if let GameFlow::SelectingTower(flow) = &game_state.flow {
            let ids = flow.shop.get_unpurchased_slot_ids();
            let count = ids.len();
            (ids, count)
        } else {
            return;
        };

    let new_slots: Vec<ShopSlot> = (0..refresh_count)
        .map(|_| generate_shop_slot(game_state))
        .collect();

    let GameFlow::SelectingTower(flow) = &mut game_state.flow else {
        unreachable!()
    };

    flow.shop.delete_slots(&unpurchased_slot_ids);

    for new_slot in new_slots {
        flow.shop.push(new_slot);
    }
}

fn generate_shop_slot(game_state: &GameState) -> ShopSlot {
    let slot_type = thread_rng().gen_range(0..10);

    match slot_type {
        0..=2 => {
            let mut rng = thread_rng();
            let item = crate::game_state::item::generation::generate_item_with_rng(
                &mut rng,
                &game_state.config,
            );
            let cost = item_cost(
                item.value,
                game_state.upgrade_state.shop_item_price_minus,
                &game_state.config,
            );
            ShopSlot::Item { item, cost }
        }
        3..=7 => {
            let upgrade = generate_tower_damage_upgrade(game_state);
            let cost = item_cost(
                upgrade.value,
                game_state.upgrade_state.shop_item_price_minus,
                &game_state.config,
            );
            ShopSlot::Upgrade { upgrade, cost }
        }
        8..=9 => {
            let upgrade = generate_tower_damage_upgrade(game_state);
            let cost = item_cost(
                upgrade.value,
                game_state.upgrade_state.shop_item_price_minus,
                &game_state.config,
            );
            ShopSlot::Upgrade { upgrade, cost }
        }
        _ => unreachable!(),
    }
}

fn generate_treasure_slot(game_state: &GameState) -> ShopSlot {
    let upgrade = generate_treasure_upgrade(game_state);
    ShopSlot::Upgrade { upgrade, cost: 0 }
}

fn item_cost(value: OneZero, discount: usize, config: &crate::config::GameConfig) -> usize {
    let base_cost = config.shop.base_cost;
    let additional_cost = value.as_f32() * base_cost * config.shop.value_cost_multiplier;
    let cost = base_cost + additional_cost - discount as f32;
    cost.max(0.0) as usize
}
