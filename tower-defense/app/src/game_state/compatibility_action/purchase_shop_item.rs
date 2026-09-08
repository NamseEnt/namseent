use crate::PresentationInstant;
use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::compatibility_action::spend_gold;
use crate::game_state::*;
use crate::shop::ShopSlot;

pub(crate) fn apply_purchase_presentation_effects(
    game_state: &mut GameState,
    slot_id: crate::shop::ShopSlotId,
    slot: ShopSlot,
    cost_value: usize,
) {
    game_state.discover_shop_slot(&slot);
    if let GameFlow::Shopping(flow) = game_state.presentation_flow_mut()
        && let Some(slot_data) = flow.shop.get_slot_by_id_mut(slot_id)
    {
        slot_data.start_exit_animation(PresentationInstant::capture());
    }

    match slot {
        ShopSlot::Item { item, .. } => {
            game_state.record_event(
                crate::game_state::play_history::HistoryEventType::ItemPurchased {
                    item,
                    cost: cost_value,
                },
            );
            spend_gold::play_spend_sound(game_state, cost_value);
        }
        ShopSlot::Upgrade { upgrade, .. } => {
            spend_gold::play_spend_sound(game_state, cost_value);
            crate::game_state::presentation_effect::apply_upgrade(
                game_state,
                upgrade,
                Some(cost_value),
            );
        }
        ShopSlot::CardService { card_service, .. } => {
            game_state.record_event(
                crate::game_state::play_history::HistoryEventType::CardServicePurchased {
                    service_kind: card_service.key().to_string(),
                    cost: cost_value,
                },
            );
            spend_gold::play_spend_sound(game_state, cost_value);
        }
    }
}

#[cfg(test)]
pub(super) fn try_purchase(game_state: &mut GameState, slot_id: crate::shop::ShopSlotId) -> bool {
    let flow = game_state.presentation_flow_snapshot();
    let slot = match &flow {
        GameFlow::Shopping(flow) => flow
            .shop
            .get_slot_by_id(slot_id)
            .map(|slot| slot.slot.clone()),
        _ => None,
    };
    let Some(slot) = slot else {
        return false;
    };
    let raw_slot_index = match game_state.raw_core.flow() {
        td_core::GameFlowState::Shopping(shop) => {
            shop.slots.iter().position(|slot| slot.id == slot_id.raw())
        }
        _ => None,
    };
    let Some(raw_slot_index) = raw_slot_index else {
        return false;
    };
    let mut raw = game_state.raw_core.state().clone();
    let Ok(purchase) = raw.purchase_shop_item(raw_slot_index) else {
        return false;
    };

    match &purchase.slot {
        td_core::ShopSlotState::Item { item, .. } => {
            raw.grant_inventory_item(item.clone())
                .expect("raw purchased item must be valid");
        }
        td_core::ShopSlotState::Upgrade { upgrade, .. } => {
            let recovery = raw
                .acquire_upgrade(upgrade.clone())
                .expect("shop upgrade payload must contain a valid kind")
                .recovery;
            raw.apply_upgrade_recovery(recovery);
        }
        td_core::ShopSlotState::CardService { .. } => {}
    }
    let cost_value = purchase.cost;
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw shop purchase must be restorable in headed adapter");
    apply_purchase_presentation_effects(game_state, slot_id, slot.clone(), cost_value);
    if let ShopSlot::CardService { card_service, .. } = slot {
        game_state.apply_compatibility_action(CompatibilityAction::UseCardService {
            card_service,
            locale: game_state.locale(),
        });
    }

    true
}

#[cfg(test)]
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

        assert!(
            game_state.apply_compatibility_action(CompatibilityAction::PurchaseShopItem(slot_id))
        );

        let purchased =
            game_state
                .pending_history_events
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

        assert!(
            !game_state.apply_compatibility_action(CompatibilityAction::PurchaseShopItem(slot_id))
        );

        let slot = if let GameFlow::Shopping(flow) = &game_state.flow {
            flow.shop.get_slot_by_id(slot_id).unwrap()
        } else {
            panic!("expected shopping flow");
        };
        assert!(!slot.purchased);
        assert_eq!(game_state.gold, 100);
        assert!(!game_state.pending_history_events.iter().any(|event| {
            matches!(
                event.event_type,
                HistoryEventType::CardServicePurchased { ref service_kind, .. }
                    if service_kind == "magic_wand"
            )
        }));
    }
}
