use super::behaviors;
use super::{
    CardServicePurchaseBlockReason, CardServiceSelectionState, CardServiceSelectionStepState,
    DeckState,
};

pub(crate) type CardServiceValidate =
    fn(&CardServiceSelectionState, &DeckState, &[Vec<usize>]) -> Result<(), crate::CommandError>;

pub(crate) struct CardServiceDefinition {
    pub(crate) kind: crate::CardServiceKind,
    pub(crate) selection_steps: fn() -> Vec<CardServiceSelectionStepState>,
    pub(crate) purchase_block_reasons: fn(&DeckState) -> Vec<CardServicePurchaseBlockReason>,
    pub(crate) validate: CardServiceValidate,
    pub(crate) apply: fn(&mut crate::CoreState, &[Vec<usize>]) -> Result<(), crate::CommandError>,
}

pub(crate) static CARD_SERVICE_DEFINITIONS: [CardServiceDefinition; crate::CardServiceKind::COUNT] = [
    behaviors::LONG_SWORD,
    behaviors::STAFF,
    behaviors::MACE,
    behaviors::CLUB_SWORD,
    behaviors::BRUSH,
    behaviors::FOUNTAIN_PEN,
    behaviors::TRICYCLE,
    behaviors::ERASER,
    behaviors::MAGIC_WAND,
    behaviors::PLIERS,
    behaviors::SCREWDRIVER,
    behaviors::COPIER,
    behaviors::MAGNET,
    behaviors::CACTUS,
    behaviors::SPINNING_TOP,
    behaviors::BATTERY,
];

pub(crate) fn card_service_definition(
    kind: crate::CardServiceKind,
) -> Option<&'static CardServiceDefinition> {
    CARD_SERVICE_DEFINITIONS
        .get(usize::from(kind.raw()))
        .filter(|definition| definition.kind == kind)
}

pub(crate) fn card_service_definition_raw(raw: u8) -> Option<&'static CardServiceDefinition> {
    card_service_definition(crate::CardServiceKind::from_raw(raw)?)
}

pub(super) fn validate_selection(
    selection: &CardServiceSelectionState,
    deck: &DeckState,
    selected_card_ids: &[Vec<usize>],
) -> Result<(), crate::CommandError> {
    if selected_card_ids.len() != selection.steps.len() {
        return Err(crate::CommandError::InvalidSelection);
    }

    let mut selected_ids = std::collections::HashSet::new();
    for (step, card_ids) in selection.steps.iter().zip(selected_card_ids) {
        if card_ids.len() != step.count {
            return Err(crate::CommandError::InvalidSelection);
        }
        for card_id in card_ids {
            if !selected_ids.insert(*card_id) {
                return Err(crate::CommandError::InvalidSelection);
            }
            let Some(card) = deck.all_cards.iter().find(|card| card.id == *card_id) else {
                return Err(crate::CommandError::InvalidIndex);
            };
            if !step.filter.matches(card) {
                return Err(crate::CommandError::InvalidSelection);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_exactly_one_definition_for_each_card_service_kind() {
        assert_eq!(
            CARD_SERVICE_DEFINITIONS.len(),
            crate::CardServiceKind::ALL.len()
        );
        for &kind in crate::CardServiceKind::ALL {
            let matches = CARD_SERVICE_DEFINITIONS
                .iter()
                .filter(|definition| definition.kind == kind)
                .count();
            assert_eq!(matches, 1, "card service kind {:?}", kind);
            assert_eq!(
                card_service_definition(kind).map(|definition| definition.kind),
                Some(kind)
            );
        }
    }
}
