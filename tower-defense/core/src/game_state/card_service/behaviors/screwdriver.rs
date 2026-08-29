use super::support::{no_purchase_blocks, selection_unengraved};
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::game_state::card_service::definition::validate_selection;

pub(crate) const DEFINITION: CardServiceDefinition = CardServiceDefinition {
    kind: crate::CardServiceKind::Screwdriver,
    selection_steps: selection_unengraved,
    purchase_block_reasons: no_purchase_blocks,
    validate: validate_selection,
    apply,
};

fn apply(
    state: &mut crate::CoreState,
    selected_card_ids: &[Vec<usize>],
) -> Result<(), crate::CommandError> {
    for card_ids in selected_card_ids {
        for card_id in card_ids {
            state.deck.modify_card(*card_id, |card| {
                card.rank = (card.rank + 1) % 13;
            });
        }
    }
    Ok(())
}
