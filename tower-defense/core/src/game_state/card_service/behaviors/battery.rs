use super::support::{one_unengraved_purchase_blocks, selection_unengraved};
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::game_state::card_service::definition::validate_selection;

pub(crate) const DEFINITION: CardServiceDefinition = CardServiceDefinition {
    kind: crate::CardServiceKind::Battery,
    selection_steps: selection_unengraved,
    purchase_block_reasons: one_unengraved_purchase_blocks,
    validate: validate_selection,
    apply,
};

fn apply(
    state: &mut crate::CoreState,
    selected_card_ids: &[Vec<usize>],
) -> Result<(), crate::CommandError> {
    super::support::apply_engraving(&mut state.deck, selected_card_ids, Some(1));
    Ok(())
}
