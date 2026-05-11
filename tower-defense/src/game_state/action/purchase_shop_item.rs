use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;
use crate::game_state::*;
use crate::shop::ShopSlot;
use namui::Instant;

pub(super) fn try_purchase(game_state: &mut GameState, slot_id: crate::shop::ShopSlotId) {
    let shop = match &mut game_state.flow {
        GameFlow::SelectingTower(flow) => &mut flow.shop,
        _ => return,
    };

    let Some(slot_data) = shop.get_slot_by_id_mut(slot_id) else {
        return;
    };

    if slot_data.purchased {
        return;
    }

    match &slot_data.slot {
        ShopSlot::Item { item, cost } => {
            if game_state.gold < *cost {
                return;
            }

            if game_state
                .stage_modifiers
                .is_item_and_upgrade_purchases_disabled()
            {
                return;
            }

            let item_clone = item.clone();
            let cost_value = *cost;

            slot_data.purchased = true;
            slot_data.start_exit_animation(Instant::now());
            game_state.items.push(item_clone.clone());
            game_state.handle_upgrade_trigger(UpgradeTriggerEvent::ItemBought);
            game_state.record_event(
                crate::game_state::play_history::HistoryEventType::ItemPurchased {
                    item: item_clone,
                    cost: cost_value,
                },
            );
            game_state.action(GameStateAction::SpendGold(cost_value));
        }
        ShopSlot::Upgrade { upgrade, cost } => {
            if game_state.gold < *cost {
                return;
            }

            if game_state
                .stage_modifiers
                .is_item_and_upgrade_purchases_disabled()
            {
                return;
            }

            let upgrade_value = *upgrade;
            let cost_value = *cost;

            slot_data.purchased = true;
            slot_data.start_exit_animation(Instant::now());
            game_state.action(GameStateAction::SpendGold(cost_value));
            game_state.action(GameStateAction::Upgrade(upgrade_value, Some(cost_value)));
        }
    }
}
