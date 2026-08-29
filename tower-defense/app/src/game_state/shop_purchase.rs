use crate::game_state::card_service::CardServiceDiscriminants;
use crate::game_state::{GameFlow, GameState};
use crate::shop::{ShopSlot, ShopSlotId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShopPurchaseBlockReason {
    InvalidSlot,
    AlreadyPurchased,
    NotEnoughGold,
    PurchasesDisabled,
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
            ShopSlot::Item { cost, .. } | ShopSlot::Upgrade { cost, .. } => (*cost, Vec::new()),
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
