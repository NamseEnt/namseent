use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;
use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::*;
use crate::shop::ShopSlot;
use namui::Instant;

pub(super) fn try_purchase(game_state: &mut GameState, slot_id: crate::shop::ShopSlotId) {
    if !game_state.shop_purchase_status(slot_id).is_available() {
        return;
    }

    let slot = match &game_state.flow {
        GameFlow::Shopping(flow) => flow
            .shop
            .get_slot_by_id(slot_id)
            .map(|slot| slot.slot.clone()),
        _ => None,
    };
    if let Some(slot) = slot.as_ref() {
        game_state.discover_shop_slot(slot);
    }

    let shop = match &mut game_state.flow {
        GameFlow::Shopping(flow) => &mut flow.shop,
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
            let cost_value = if game_state.stage_modifiers.is_free_shop_this_stage() {
                0
            } else {
                *cost
            };

            if game_state.gold < cost_value {
                return;
            }

            if game_state
                .stage_modifiers
                .is_item_and_upgrade_purchases_disabled()
            {
                return;
            }

            let item = item.clone();
            slot_data.purchased = true;
            slot_data.start_exit_animation(Instant::now());
            game_state.items.push(item.clone().with_unique_id());
            game_state.handle_upgrade_trigger(UpgradeTriggerEvent::ItemBought);
            game_state.record_event(
                crate::game_state::play_history::HistoryEventType::ItemPurchased {
                    item: item.clone(),
                    cost: cost_value,
                },
            );
            game_state.action(GameStateAction::SpendGold(cost_value));
        }
        ShopSlot::Upgrade { upgrade, cost } => {
            let cost_value = if game_state.stage_modifiers.is_free_shop_this_stage() {
                0
            } else {
                *cost
            };

            if game_state.gold < cost_value {
                return;
            }

            if game_state
                .stage_modifiers
                .is_item_and_upgrade_purchases_disabled()
            {
                return;
            }

            let upgrade_value = *upgrade;

            slot_data.purchased = true;
            slot_data.start_exit_animation(Instant::now());
            game_state.action(GameStateAction::SpendGold(cost_value));
            game_state.action(GameStateAction::Upgrade(upgrade_value, Some(cost_value)));
        }
        ShopSlot::CardService { card_service, cost } => {
            let cost_value = if game_state.stage_modifiers.is_free_shop_this_stage() {
                0
            } else {
                *cost
            };

            if game_state.gold < cost_value {
                return;
            }

            if game_state
                .stage_modifiers
                .is_item_and_upgrade_purchases_disabled()
            {
                return;
            }

            let card_service_value = card_service.clone();

            slot_data.purchased = true;
            slot_data.start_exit_animation(Instant::now());
            game_state.record_event(
                crate::game_state::play_history::HistoryEventType::CardServicePurchased {
                    service_kind: card_service_value.key().to_string(),
                    cost: cost_value,
                },
            );
            game_state.action(GameStateAction::SpendGold(cost_value));
            game_state.action(GameStateAction::UseCardService(card_service_value));
        }
    }
}

#[cfg(all(test, feature = "simulator"))]
mod tests {
    use super::*;
    use crate::game_state::card_service::CardServiceDiscriminants;
    use crate::game_state::play_history::HistoryEventType;

    #[test]
    fn purchasing_a_card_service_records_its_kind_and_cost() {
        let mut game_state = crate::game_state::create_initial_game_state();
        game_state.headless = true;
        game_state.gold = 100;

        let slot_id = if let GameFlow::Shopping(flow) = &mut game_state.flow {
            flow.shop.push(ShopSlot::CardService {
                card_service: CardServiceDiscriminants::Eraser.generate(),
                cost: 50,
            });
            flow.shop.slots.last().unwrap().id
        } else {
            panic!("expected shopping flow");
        };

        game_state.action(GameStateAction::PurchaseShopItem(slot_id));

        let purchased =
            game_state
                .play_history
                .events
                .iter()
                .find_map(|event| match &event.event_type {
                    HistoryEventType::CardServicePurchased { service_kind, cost } => {
                        Some((service_kind.clone(), *cost))
                    }
                    _ => None,
                });

        assert_eq!(purchased, Some(("eraser".to_string(), 50)));
        assert_eq!(game_state.gold, 50);
    }
    #[test]
    fn magic_wand_purchase_is_blocked_without_an_engraved_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        game_state.headless = true;
        game_state.gold = 100;

        let slot_id = if let GameFlow::Shopping(flow) = &mut game_state.flow {
            flow.shop.push(ShopSlot::CardService {
                card_service: CardServiceDiscriminants::MagicWand.generate(),
                cost: 50,
            });
            flow.shop.slots.last().unwrap().id
        } else {
            panic!("expected shopping flow");
        };

        game_state.action(GameStateAction::PurchaseShopItem(slot_id));

        let slot = if let GameFlow::Shopping(flow) = &game_state.flow {
            flow.shop.get_slot_by_id(slot_id).unwrap()
        } else {
            panic!("expected shopping flow");
        };
        assert!(!slot.purchased);
        assert_eq!(game_state.gold, 100);
        assert!(!game_state.play_history.events.iter().any(|event| {
            matches!(
                event.event_type,
                HistoryEventType::CardServicePurchased { ref service_kind, .. }
                    if service_kind == "magic_wand"
            )
        }));
    }
}
