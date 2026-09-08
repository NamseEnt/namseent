use super::CardServiceBehavior;
use super::support::validate_selection;
use super::support::{no_purchase_blocks, selection_any};

#[derive(Clone, Copy)]
pub(crate) struct Behavior;

impl CardServiceBehavior for Behavior {
    fn kind(&self) -> crate::CardServiceKind {
        crate::CardServiceKind::Eraser
    }

    fn selection_steps(&self) -> Vec<crate::CardServiceSelectionStepState> {
        selection_any()
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
        for card_ids in selected_card_ids {
            for card_id in card_ids {
                state.deck.remove_card(*card_id);
            }
        }
        Ok(())
    }
}
