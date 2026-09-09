use crate::game_state::card_service::CardServiceDiscriminants;
use crate::game_state::{GameFlow, GameState};
use crate::shop::{ShopSlot, ShopSlotId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShopPurchaseBlockReason {
    InvalidSlot,
    AlreadyPurchased,
    NotEnoughGold,
    PurchasesDisabled,
    ItemCapacityReached,
    TreasureCapacityReached,
    CardService(td_core::CardServicePurchaseBlockReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShopPurchaseStatus {
    reasons: Vec<ShopPurchaseBlockReason>,
}

impl ShopPurchaseStatus {
    pub(crate) fn blocked(reason: ShopPurchaseBlockReason) -> Self {
        Self {
            reasons: vec![reason],
        }
    }

    pub(crate) fn is_available(&self) -> bool {
        self.reasons.is_empty()
    }

    pub(crate) fn reasons(&self) -> &[ShopPurchaseBlockReason] {
        &self.reasons
    }
}

impl GameState {
    pub(crate) fn shop_purchase_status(&self, slot_id: ShopSlotId) -> ShopPurchaseStatus {
        let flow = self.presentation_flow_snapshot();
        let Some(slot_data) = (match &flow {
            GameFlow::Shopping(flow) => flow.shop.get_slot_by_id(slot_id),
            _ => None,
        }) else {
            return ShopPurchaseStatus::blocked(ShopPurchaseBlockReason::InvalidSlot);
        };

        let mut reasons = Vec::new();
        if slot_data.purchased {
            reasons.push(ShopPurchaseBlockReason::AlreadyPurchased);
        }

        let raw = self.raw_core_state();
        let (cost, card_service_reasons) = match &slot_data.slot {
            ShopSlot::Item { cost, .. } => {
                if raw.items().len() >= raw.item_capacity() {
                    reasons.push(ShopPurchaseBlockReason::ItemCapacityReached);
                }
                (*cost, Vec::new())
            }
            ShopSlot::Upgrade { cost, .. } => {
                if raw.upgrades().len() >= raw.treasure_capacity() {
                    reasons.push(ShopPurchaseBlockReason::TreasureCapacityReached);
                }
                (*cost, Vec::new())
            }
            ShopSlot::CardService {
                card_service, cost, ..
            } => (
                *cost,
                td_core::purchase_block_reasons(
                    CardServiceDiscriminants::from(card_service).to_core_kind(),
                    raw.deck(),
                )
                .into_iter()
                .map(ShopPurchaseBlockReason::CardService)
                .collect(),
            ),
        };

        let effective_cost = if raw.stage_modifiers().free_shop_this_stage {
            0
        } else {
            cost
        };
        if raw.progress().gold < effective_cost {
            reasons.push(ShopPurchaseBlockReason::NotEnoughGold);
        }
        if raw.stage_modifiers().disable_item_and_upgrade_purchases {
            reasons.push(ShopPurchaseBlockReason::PurchasesDisabled);
        }
        reasons.extend(card_service_reasons);

        ShopPurchaseStatus { reasons }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::flow::GameFlow;
    use crate::shop::ShopSlot;

    fn shop_slot_id(state: &GameState, predicate: impl Fn(&ShopSlot) -> bool) -> ShopSlotId {
        let GameFlow::Shopping(flow) = state.presentation_flow_snapshot() else {
            panic!("test state should start in shopping flow");
        };
        flow.shop
            .slots
            .iter()
            .find(|slot| predicate(&slot.slot))
            .map(|slot| slot.id)
            .expect("matching shop slot")
    }

    #[test]
    fn reports_item_capacity_before_purchase() {
        let mut state = crate::game_state::create_game_state_with_seed(7);
        let slot_id = shop_slot_id(&state, |slot| matches!(slot, ShopSlot::Item { .. }));
        state
            .raw_core
            .edit_snapshot(|parts| {
                for _ in 0..2 {
                    let next_id = parts
                        .items
                        .entries()
                        .iter()
                        .map(td_core::ItemEntry::id)
                        .max()
                        .unwrap_or(0)
                        .saturating_add(1);
                    let item = td_core::generated_item(td_core::ItemKind::Bread)
                        .expect("bread item")
                        .with_id(next_id);
                    parts.items.entries_mut().push(item);
                }
            })
            .expect("capacity fixture should be valid");

        let status = state.shop_purchase_status(slot_id);
        assert!(
            status
                .reasons()
                .contains(&ShopPurchaseBlockReason::ItemCapacityReached)
        );
        assert!(!status.is_available());
    }

    #[test]
    fn reports_treasure_capacity_before_purchase() {
        let mut state = crate::game_state::create_game_state_with_seed(7);
        let mut raw = state.raw_core.state().clone();
        raw.edit_snapshot(|parts| {
            let td_core::GameFlowState::Shopping(shop) = &mut parts.flow else {
                return;
            };
            shop.slots.push(td_core::ShopSlotDataState {
                id: 999,
                slot: td_core::ShopSlotState::Upgrade {
                    upgrade: td_core::generated_upgrade(td_core::UpgradeKind::Apple),
                    cost: 0,
                },
                purchased: false,
            });
        })
        .expect("shop fixture should be valid");
        state
            .restore_raw_core_projection(raw)
            .expect("shop fixture should restore");
        let slot_id = shop_slot_id(&state, |slot| matches!(slot, ShopSlot::Upgrade { .. }));
        state
            .raw_core
            .edit_snapshot(|parts| {
                for _ in 0..5 {
                    let next_id = parts
                        .upgrades
                        .entries()
                        .iter()
                        .map(td_core::UpgradeEntry::id)
                        .max()
                        .unwrap_or(0)
                        .saturating_add(1);
                    let upgrade =
                        td_core::generated_upgrade(td_core::UpgradeKind::Apple).with_id(next_id);
                    parts.upgrades.entries_mut().push(upgrade);
                }
            })
            .expect("capacity fixture should be valid");

        let status = state.shop_purchase_status(slot_id);
        assert!(
            status
                .reasons()
                .contains(&ShopPurchaseBlockReason::TreasureCapacityReached)
        );
        assert!(!status.is_available());
    }
}
