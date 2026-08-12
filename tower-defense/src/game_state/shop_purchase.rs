use crate::game_state::card_service::{
    CardServiceBehavior, CardServicePurchaseBlockReason, CardServicePurchaseContext,
};
use crate::game_state::{GameFlow, GameState};
use crate::shop::{ShopSlot, ShopSlotId};
use namui::*;

#[derive(Debug, Clone, State, PartialEq, Eq)]
pub(crate) enum ShopPurchaseBlockReason {
    InvalidSlot,
    AlreadyPurchased,
    NotEnoughGold,
    PurchasesDisabled,
    CardService(CardServicePurchaseBlockReason),
}

#[derive(Debug, Clone, State, PartialEq, Eq)]
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

impl From<CardServicePurchaseBlockReason> for ShopPurchaseBlockReason {
    fn from(reason: CardServicePurchaseBlockReason) -> Self {
        Self::CardService(reason)
    }
}

impl GameState {
    pub(crate) fn shop_purchase_status(&self, slot_id: ShopSlotId) -> ShopPurchaseStatus {
        let context = CardServicePurchaseContext::from_game_state(self);
        self.shop_purchase_status_with_context(slot_id, &context)
    }

    pub(crate) fn shop_purchase_status_with_context(
        &self,
        slot_id: ShopSlotId,
        context: &CardServicePurchaseContext,
    ) -> ShopPurchaseStatus {
        let Some(slot_data) = (match &self.flow {
            GameFlow::Shopping(flow) => flow.shop.get_slot_by_id(slot_id),
            _ => None,
        }) else {
            return ShopPurchaseStatus::blocked(ShopPurchaseBlockReason::InvalidSlot);
        };

        let mut reasons = Vec::new();
        if slot_data.purchased {
            reasons.push(ShopPurchaseBlockReason::AlreadyPurchased);
        }

        let (cost, card_service_reasons) = match &slot_data.slot {
            ShopSlot::Item { cost, .. } | ShopSlot::Upgrade { cost, .. } => (*cost, Vec::new()),
            ShopSlot::CardService {
                card_service, cost, ..
            } => (
                *cost,
                card_service
                    .purchase_block_reasons(context)
                    .into_iter()
                    .map(ShopPurchaseBlockReason::from)
                    .collect(),
            ),
        };

        let effective_cost = if self.stage_modifiers.is_free_shop_this_stage() {
            0
        } else {
            cost
        };
        if self.gold < effective_cost {
            reasons.push(ShopPurchaseBlockReason::NotEnoughGold);
        }
        if self
            .stage_modifiers
            .is_item_and_upgrade_purchases_disabled()
        {
            reasons.push(ShopPurchaseBlockReason::PurchasesDisabled);
        }
        reasons.extend(card_service_reasons);

        ShopPurchaseStatus { reasons }
    }
}
