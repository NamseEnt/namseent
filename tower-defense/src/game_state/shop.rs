use crate::{
    game_state::{GameState, item::generation::generate_item, upgrade::generate_upgrade},
    rarity::Rarity,
    shop::ShopSlot,
};
use namui::OneZero;
use rand::{Rng, thread_rng};

pub fn initialize_shop(game_state: &mut GameState) {
    let items = (0..game_state.max_shop_slot())
        .map(|_| generate_shop_slot(game_state))
        .collect::<Vec<_>>();
    for slot in game_state.shop_slots.iter_mut() {
        *slot = ShopSlot::Locked;
    }
    for (slot, item) in game_state.shop_slots.iter_mut().zip(items.into_iter()) {
        *slot = item;
    }
}

pub fn refresh_shop(game_state: &mut GameState) {
    let items = (0..game_state.max_shop_slot())
        .map(|_| generate_shop_slot(game_state))
        .collect::<Vec<_>>();
    for (slot, item) in game_state.shop_slots.iter_mut().zip(items.into_iter()) {
        let purchased = match slot {
            ShopSlot::Item { purchased, .. } | ShopSlot::Upgrade { purchased, .. } => *purchased,
            _ => false,
        };
        if purchased {
            continue;
        }
        *slot = item;
    }
}

fn generate_shop_slot(game_state: &GameState) -> ShopSlot {
    let is_item = thread_rng().gen_bool(0.3);
    let rarity = game_state.generate_rarity(Default::default());

    match is_item {
        true => {
            let item = generate_item(rarity);
            let cost = item_cost(
                rarity,
                item.value,
                game_state.upgrade_state.shop_item_price_minus,
            );
            ShopSlot::Item {
                item,
                cost,
                purchased: false,
            }
        }
        false => {
            let upgrade = generate_upgrade(game_state, rarity);
            let cost = item_cost(
                rarity,
                upgrade.value,
                game_state.upgrade_state.shop_item_price_minus,
            );
            ShopSlot::Upgrade {
                upgrade,
                cost,
                purchased: false,
            }
        }
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
