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
const BASE_COST_TREASURE: f32 = 95.0;
const BASE_COST_CARD_SERVICE: f32 = 45.0;
const SHOP_VALUE_COST_MULTIPLIER: f32 = 0.5;
const RARE_PRICE_FACTOR: f32 = 1.4;
const EPIC_PRICE_FACTOR: f32 = 1.8;
const LEGENDARY_PRICE_FACTOR: f32 = 2.2;

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

    pub fn push_free_card_service(&mut self) {
        self.push(ShopSlot::CardService {
            card_service: generate_shop_card_service(),
            cost: 0,
        });
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
    game_state.discover_shop();
}

pub fn add_shop_slots(game_state: &mut GameState, count: usize) {
    for _ in 0..count {
        let slot = generate_shop_slot(game_state);
        let GameFlow::Shopping(flow) = &mut game_state.flow else {
            return;
        };
        flow.shop.push(slot);
    }
    game_state.discover_shop();
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
    let additional_cost_ratio = rng.gen_range(0.0..=SHOP_VALUE_COST_MULTIPLIER);
    let rarity = slot.rarity();

    let cost = match slot {
        ShopSlot::Item { ref mut cost, .. } => cost,
        ShopSlot::Upgrade { ref mut cost, .. } => cost,
        ShopSlot::CardService { ref mut cost, .. } => cost,
    };
    *cost = calculate_cost(base_cost, rarity, additional_cost_ratio, free, discount);

    slot
}

fn rarity_price_factor(rarity: Rarity) -> f32 {
    match rarity {
        Rarity::Common => 1.0,
        Rarity::Rare => RARE_PRICE_FACTOR,
        Rarity::Epic => EPIC_PRICE_FACTOR,
        Rarity::Legendary => LEGENDARY_PRICE_FACTOR,
    }
}

fn calculate_cost(
    base_cost: f32,
    rarity: Rarity,
    additional_cost_ratio: f32,
    free: bool,
    discount: usize,
) -> usize {
    if free {
        return 0;
    }

    let rarity_adjusted_base = base_cost * rarity_price_factor(rarity);
    (rarity_adjusted_base * (1.0 + additional_cost_ratio) - discount as f32).max(0.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rarity_price_factors_are_ordered() {
        assert_eq!(rarity_price_factor(Rarity::Common), 1.0);
        assert_eq!(rarity_price_factor(Rarity::Rare), 1.25);
        assert_eq!(rarity_price_factor(Rarity::Epic), 1.5);
        assert_eq!(rarity_price_factor(Rarity::Legendary), 2.0);
    }

    #[test]
    fn calculate_cost_applies_rarity_and_randomness_before_discount() {
        assert_eq!(calculate_cost(20.0, Rarity::Common, 0.0, false, 0), 20);
        assert_eq!(calculate_cost(20.0, Rarity::Rare, 0.0, false, 0), 25);
        assert_eq!(calculate_cost(20.0, Rarity::Epic, 0.5, false, 5), 40);
        assert_eq!(
            calculate_cost(100.0, Rarity::Legendary, 0.5, false, 25),
            275
        );
    }

    #[test]
    fn calculate_cost_keeps_free_and_discount_floor_behavior() {
        assert_eq!(calculate_cost(100.0, Rarity::Legendary, 0.5, true, 0), 0);
        assert_eq!(calculate_cost(20.0, Rarity::Common, 0.0, false, 25), 0);
    }
}
