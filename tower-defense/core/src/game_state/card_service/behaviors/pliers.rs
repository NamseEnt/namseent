use super::support::{engraved_purchase_blocks, selection_engraved};
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::game_state::card_service::definition::validate_selection;

pub(crate) const DEFINITION: CardServiceDefinition = CardServiceDefinition {
    kind: crate::CardServiceKind::Pliers,
    selection_steps: selection_engraved,
    purchase_block_reasons: engraved_purchase_blocks,
    validate: validate_selection,
    apply,
};

fn apply(
    state: &mut crate::CoreState,
    selected_card_ids: &[Vec<usize>],
) -> Result<(), crate::CommandError> {
    super::support::apply_engraving(&mut state.deck, selected_card_ids, None);
    Ok(())
}
