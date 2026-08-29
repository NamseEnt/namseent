use super::support::{selection_two_unengraved, two_unengraved_purchase_blocks};
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::game_state::card_service::definition::validate_selection;

pub(crate) const DEFINITION: CardServiceDefinition = CardServiceDefinition {
    kind: crate::CardServiceKind::Magnet,
    selection_steps: selection_two_unengraved,
    purchase_block_reasons: two_unengraved_purchase_blocks,
    validate: validate_selection,
    apply,
};

fn apply(
    state: &mut crate::CoreState,
    selected_card_ids: &[Vec<usize>],
) -> Result<(), crate::CommandError> {
    super::support::apply_engraving(&mut state.deck, selected_card_ids, Some(0));
    Ok(())
}
