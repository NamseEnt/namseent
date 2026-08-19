mod bag;
mod deterministic;
#[cfg(test)]
mod deterministic_tests;
mod shop_slot;

pub(crate) use deterministic::{ShopGenerationStats, generate_shop_screen_with_stats};
pub(crate) use shop_slot::*;

use crate::{
    game_state::{GameState, flow::GameFlow},
    *,
};

const RARE_PRICE_FACTOR_BASIS_POINTS: u64 = 14_000;
const EPIC_PRICE_FACTOR_BASIS_POINTS: u64 = 18_000;
const LEGENDARY_PRICE_FACTOR_BASIS_POINTS: u64 = 22_000;
const BASIS_POINTS: u64 = 10_000;
const MAX_ADDITIONAL_COST_BASIS_POINTS: u64 = 5_000;

#[derive(Clone, Debug, State)]
pub struct Shop {
    pub slots: Vec<ShopSlotData>,
    pub last_generation_stats: ShopGenerationStats,
}

impl Shop {
    pub fn new(game_state: &mut GameState) -> Self {
        let slot_count = game_state.max_shop_slot();
        let generated = generate_shop_screen_with_stats(game_state, slot_count, &[], None);
        Self {
            slots: generated.slots,
            last_generation_stats: generated.stats,
        }
    }

    pub fn get_slot_by_id(&self, id: ShopSlotId) -> Option<&ShopSlotData> {
        self.slots.iter().find(|slot| slot.id == id)
    }

    pub fn get_slot_by_id_mut(&mut self, id: ShopSlotId) -> Option<&mut ShopSlotData> {
        self.slots.iter_mut().find(|slot| slot.id == id)
    }

    pub fn push(&mut self, slot: ShopSlot) {
        self.slots.push(ShopSlotData::new(slot));
    }

    pub fn push_free_card_service(&mut self, game_state: &mut GameState) {
        let existing_slots = self.slots.clone();
        let generated = generate_shop_screen_with_stats(
            game_state,
            1,
            &existing_slots,
            Some(crate::shop::deterministic::CARD_SERVICE_CATEGORY),
        );
        let mut slots = generated.slots;
        if let Some(slot) = slots.first_mut()
            && let ShopSlot::CardService { cost, .. } = &mut slot.slot
        {
            *cost = 0;
        }
        self.slots.extend(slots);
        self.last_generation_stats = generated.stats;
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

pub fn add_shop_slots(game_state: &mut GameState, count: usize) {
    let existing_slots = match &game_state.flow {
        GameFlow::Shopping(flow) => flow.shop.slots.clone(),
        _ => return,
    };
    let generated = generate_shop_screen_with_stats(game_state, count, &existing_slots, None);
    let GameFlow::Shopping(flow) = &mut game_state.flow else {
        return;
    };
    flow.shop.slots.extend(generated.slots);
    flow.shop.last_generation_stats = generated.stats;
    game_state.discover_shop();
}

fn rarity_price_factor_basis_points(rarity: Rarity) -> u64 {
    match rarity {
        Rarity::Common => BASIS_POINTS,
        Rarity::Rare => RARE_PRICE_FACTOR_BASIS_POINTS,
        Rarity::Epic => EPIC_PRICE_FACTOR_BASIS_POINTS,
        Rarity::Legendary => LEGENDARY_PRICE_FACTOR_BASIS_POINTS,
    }
}

pub(crate) fn calculate_cost_basis_points(
    base_cost: u64,
    rarity: Rarity,
    additional_cost_basis_points: u64,
    free: bool,
    discount: usize,
) -> usize {
    if free {
        return 0;
    }

    let total_basis_points =
        BASIS_POINTS + additional_cost_basis_points.min(MAX_ADDITIONAL_COST_BASIS_POINTS);
    let numerator = u128::from(base_cost)
        * u128::from(rarity_price_factor_basis_points(rarity))
        * u128::from(total_basis_points);
    numerator
        .checked_div(u128::from(BASIS_POINTS) * u128::from(BASIS_POINTS))
        .unwrap_or(u128::MAX)
        .min(usize::MAX as u128)
        .saturating_sub(discount as u128) as usize
}

#[cfg(test)]
fn rarity_price_factor(rarity: Rarity) -> f32 {
    match rarity {
        Rarity::Common => 1.0,
        Rarity::Rare => 1.4,
        Rarity::Epic => 1.8,
        Rarity::Legendary => 2.2,
    }
}

#[cfg(test)]
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
    fn calculate_cost_keeps_free_and_discount_floor_behavior() {
        assert_eq!(calculate_cost(100.0, Rarity::Legendary, 0.5, true, 0), 0);
        assert_eq!(calculate_cost(20.0, Rarity::Common, 0.0, false, 25), 0);
    }

    #[test]
    fn integer_cost_calculation_is_deterministic() {
        assert_eq!(
            calculate_cost_basis_points(20, Rarity::Rare, 5_000, false, 0),
            42
        );
        assert_eq!(
            calculate_cost_basis_points(95, Rarity::Legendary, 0, false, 15),
            194
        );
    }
}
