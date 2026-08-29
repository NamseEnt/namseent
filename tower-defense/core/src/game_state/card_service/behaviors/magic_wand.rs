use super::support::{engraved_and_unengraved_purchase_blocks, selection_engraved_then_unengraved};
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::game_state::card_service::definition::validate_selection;

pub(crate) const DEFINITION: CardServiceDefinition = CardServiceDefinition {
    kind: crate::CardServiceKind::MagicWand,
    selection_steps: selection_engraved_then_unengraved,
    purchase_block_reasons: engraved_and_unengraved_purchase_blocks,
    validate: validate_selection,
    apply,
};

fn apply(
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
