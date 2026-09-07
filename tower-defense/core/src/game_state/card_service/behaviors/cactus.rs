use super::CardServiceBehavior;
use super::support::validate_selection;
use super::support::{one_unengraved_purchase_blocks, selection_any};

#[derive(Clone, Copy)]
pub(crate) struct Behavior;

impl CardServiceBehavior for Behavior {
    fn kind(&self) -> crate::CardServiceKind {
        crate::CardServiceKind::Cactus
    }

    fn selection_steps(&self) -> Vec<crate::CardServiceSelectionStepState> {
        selection_any()
    }

    fn purchase_block_reasons(
        &self,
        deck: &crate::DeckState,
    ) -> Vec<crate::CardServicePurchaseBlockReason> {
        one_unengraved_purchase_blocks(deck)
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
        super::support::apply_engraving(&mut state.deck, selected_card_ids, Some(2));
        Ok(())
    }
}
