use crate::{
    game_state::{
        GameState,
        item::{generate_item, generate_items, item_cost},
        upgrade::generate_upgrade,
    },
    shop::ShopSlot,
};
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
    let items = generate_items(game_state, game_state.max_shop_slot());
    for (slot, item) in game_state.shop_slots.iter_mut().zip(items.into_iter()) {
        if let ShopSlot::Item {
            item: item_of_slot,
            cost: cost_of_slot,
            purchased,
        } = slot
        {
            if *purchased {
                continue;
            }
            let cost = item_cost(&item.rarity, game_state.upgrade_state.shop_item_price_minus);
            *cost_of_slot = cost;
            *item_of_slot = item.clone();
        }
    }
}

fn generate_shop_slot(game_state: &GameState) -> ShopSlot {
    let is_item = thread_rng().gen_bool(0.3);
    let rarity = game_state.generate_rarity(Default::default());

    match is_item {
        true => {
            let item = generate_item(rarity);
            let cost = item_cost(&item.rarity, game_state.upgrade_state.shop_item_price_minus);
            ShopSlot::Item {
                item,
                cost,
                purchased: false,
            }
        }
        false => {
            let upgrade = generate_upgrade(game_state, rarity);
            // TODO
            let cost = 1;

            ShopSlot::Upgrade {
                upgrade,
                cost,
                purchased: false,
            }
        }
    }
}
