use super::CardServiceBehavior;
use super::support::validate_selection;
use super::support::{engraved_and_unengraved_purchase_blocks, selection_engraved_then_unengraved};

#[derive(Clone, Copy)]
pub(crate) struct Behavior;

impl CardServiceBehavior for Behavior {
    fn kind(&self) -> crate::CardServiceKind {
        crate::CardServiceKind::MagicWand
    }

    fn selection_steps(&self) -> Vec<crate::CardServiceSelectionStepState> {
        selection_engraved_then_unengraved()
    }

    fn purchase_block_reasons(
        &self,
        deck: &crate::DeckState,
    ) -> Vec<crate::CardServicePurchaseBlockReason> {
        engraved_and_unengraved_purchase_blocks(deck)
    }

    fn validate(
        &self,
        selection: &crate::CardServiceSelectionState,
        deck: &crate::DeckState,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError> {
        validate_selection(selection, deck, selected_card_ids)
    }

    fn apply(
        &self,
        state: &mut crate::CoreState,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError> {
        let source_card_id = selected_card_ids
            .first()
            .and_then(|card_ids| card_ids.first())
            .copied()
            .ok_or(crate::CommandError::InvalidSelection)?;
        let target_card_id = selected_card_ids
            .get(1)
            .and_then(|card_ids| card_ids.first())
            .copied()
            .ok_or(crate::CommandError::InvalidSelection)?;
        let engraving = state
            .deck
            .get_card(source_card_id)
            .and_then(|card| card.engraving)
            .ok_or(crate::CommandError::Rejected)?;
        if state
            .deck
            .get_card(target_card_id)
            .is_none_or(|card| card.engraving.is_some())
        {
            return Err(crate::CommandError::Rejected);
        }
        state.deck.modify_card(source_card_id, |card| {
            card.engraving = None;
        });
        state.deck.modify_card(target_card_id, |card| {
            card.engraving = Some(engraving);
        });
        Ok(())
    }
}
