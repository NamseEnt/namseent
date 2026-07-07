mod shop_slot;

use crate::{
    game_state::{
        GameState, card_service::generate_shop_card_service, flow::GameFlow,
        upgrade::generate_shop_upgrade,
    },
    *,
};
use rand::{Rng, thread_rng};
pub use shop_slot::*;

const BASE_COST_ITEM: f32 = 20.0;
const BASE_COST_TREASURE: f32 = 100.0;
const BASE_COST_CARD_SERVICE: f32 = 50.0;
const SHOP_VALUE_COST_MULTIPLIER: f32 = 0.5;

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
    let (unpurchased_slot_ids, refresh_count) = if let GameFlow::Shopping(flow) = &game_state.flow {
        let ids = flow.shop.get_unpurchased_slot_ids();
        let count = ids.len();
        (ids, count)
    } else {
        return;
    };

    let new_slots: Vec<ShopSlot> = (0..refresh_count)
        .map(|_| generate_shop_slot(game_state))
        .collect();

    let GameFlow::Shopping(flow) = &mut game_state.flow else {
        unreachable!()
    };

    flow.shop.delete_slots(&unpurchased_slot_ids);

    for new_slot in new_slots {
        flow.shop.push(new_slot);
    }
}

pub fn add_shop_slots(game_state: &mut GameState, count: usize) {
    for _ in 0..count {
        let slot = generate_shop_slot(game_state);
        let GameFlow::Shopping(flow) = &mut game_state.flow else {
            return;
        };
        flow.shop.push(slot);
    }
}

fn generate_shop_slot(game_state: &GameState) -> ShopSlot {
    let mut rng = thread_rng();
    let slot_type = thread_rng().gen_range(0..10);
    let free = game_state.stage_modifiers.is_free_shop_this_stage();
    let discount = game_state.upgrade_state.shop_item_price_minus();

    let slot = match slot_type {
        0..=4 => {
            let mut rng = thread_rng();
            let item = crate::game_state::item::generation::generate_item_with_rng(&mut rng);
            ShopSlot::Item { item, cost: 0 }
        }
        5..=7 => {
            let card_service = generate_shop_card_service();
            ShopSlot::CardService {
                card_service,
                cost: 0,
            }
        }
        8..=9 => {
            let upgrade = generate_shop_upgrade(game_state);
            ShopSlot::Upgrade { upgrade, cost: 0 }
        }
        _ => unreachable!(),
    };
    apply_random_cost(&mut rng, slot, free, discount)
}

fn apply_random_cost<R: Rng + ?Sized>(
    rng: &mut R,
    mut slot: ShopSlot,
    free: bool,
    discount: usize,
) -> ShopSlot {
    let base_cost = match slot {
        ShopSlot::Item { .. } => BASE_COST_ITEM,
        ShopSlot::Upgrade { .. } => BASE_COST_TREASURE,
        ShopSlot::CardService { .. } => BASE_COST_CARD_SERVICE,
    };
    let additional_cost = rng.gen_range(0.0..=base_cost * SHOP_VALUE_COST_MULTIPLIER);

    let cost = match slot {
        ShopSlot::Item { ref mut cost, .. } => cost,
        ShopSlot::Upgrade { ref mut cost, .. } => cost,
        ShopSlot::CardService { ref mut cost, .. } => cost,
    };
    *cost = match free {
        true => 0,
        false => (base_cost + additional_cost - discount as f32).max(0.0) as usize,
    };

    slot
}
