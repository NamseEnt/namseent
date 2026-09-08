use super::CardServiceBehavior;
use super::support::validate_selection;
use super::support::{apply_enhancement_service, no_purchase_blocks, selection_low_ranks};

#[derive(Clone, Copy)]
pub(crate) struct Behavior;

impl CardServiceBehavior for Behavior {
    fn kind(&self) -> crate::CardServiceKind {
        crate::CardServiceKind::Tricycle
    }

    fn selection_steps(&self) -> Vec<crate::CardServiceSelectionStepState> {
        selection_low_ranks()
    }

    fn purchase_block_reasons(
        &self,
        deck: &crate::DeckState,
    ) -> Vec<crate::CardServicePurchaseBlockReason> {
        no_purchase_blocks(deck)
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
        apply_enhancement_service(state, selected_card_ids, None, 2_000_000)
    }
}
