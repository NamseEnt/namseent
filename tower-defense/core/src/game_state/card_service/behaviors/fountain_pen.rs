use super::support::{apply_enhancement_service, no_purchase_blocks, selection_number};
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::game_state::card_service::definition::validate_selection;

pub(crate) const DEFINITION: CardServiceDefinition = CardServiceDefinition {
    kind: crate::CardServiceKind::FountainPen,
    selection_steps: selection_number,
    purchase_block_reasons: no_purchase_blocks,
    validate: validate_selection,
    apply,
};

fn apply(
    state: &mut crate::CoreState,
    selected_card_ids: &[Vec<usize>],
) -> Result<(), crate::CommandError> {
    apply_enhancement_service(state, selected_card_ids, None, 3_000_000)
}
