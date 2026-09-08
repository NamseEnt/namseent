use super::super::{
    CardSelectionFilterState, CardServicePurchaseBlockReason, CardServiceSelectionState,
    CardServiceSelectionStepState, DeckState,
};

pub(super) fn selection_any() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 1,
        filter: CardSelectionFilterState::Any,
    }]
}

pub(super) fn selection_face() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 1,
        filter: CardSelectionFilterState::Face,
    }]
}

pub(super) fn selection_number() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 1,
        filter: CardSelectionFilterState::Number,
    }]
}

pub(super) fn selection_low_ranks() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 1,
        filter: CardSelectionFilterState::Or(vec![
            CardSelectionFilterState::Rank(0),
            CardSelectionFilterState::Rank(1),
            CardSelectionFilterState::Rank(2),
        ]),
    }]
}

pub(super) fn selection_engraved_then_unengraved() -> Vec<CardServiceSelectionStepState> {
    vec![
        CardServiceSelectionStepState {
            count: 1,
            filter: CardSelectionFilterState::Engraved,
        },
        CardServiceSelectionStepState {
            count: 1,
            filter: CardSelectionFilterState::NotEngraved,
        },
    ]
}

pub(super) fn selection_engraved() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 1,
        filter: CardSelectionFilterState::Engraved,
    }]
}

pub(super) fn selection_unengraved() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 1,
        filter: CardSelectionFilterState::NotEngraved,
    }]
}

pub(super) fn selection_two_unengraved() -> Vec<CardServiceSelectionStepState> {
    vec![CardServiceSelectionStepState {
        count: 2,
        filter: CardSelectionFilterState::NotEngraved,
    }]
}

pub(super) fn no_purchase_blocks(_: &DeckState) -> Vec<CardServicePurchaseBlockReason> {
    Vec::new()
}

pub(super) fn engraved_and_unengraved_purchase_blocks(
    deck: &DeckState,
) -> Vec<CardServicePurchaseBlockReason> {
    let (engraved_card_count, unengraved_card_count) = card_counts(deck);
    let mut reasons = Vec::new();
    if engraved_card_count == 0 {
        reasons.push(CardServicePurchaseBlockReason::NoEngravedCard);
    }
    if unengraved_card_count < 1 {
        reasons.push(CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
            required: 1,
            available: unengraved_card_count,
        });
    }
    reasons
}

pub(super) fn engraved_purchase_blocks(deck: &DeckState) -> Vec<CardServicePurchaseBlockReason> {
    let (engraved_card_count, _) = card_counts(deck);
    (engraved_card_count == 0)
        .then_some(CardServicePurchaseBlockReason::NoEngravedCard)
        .into_iter()
        .collect()
}

pub(super) fn one_unengraved_purchase_blocks(
    deck: &DeckState,
) -> Vec<CardServicePurchaseBlockReason> {
    unengraved_purchase_blocks(deck, 1)
}

pub(super) fn two_unengraved_purchase_blocks(
    deck: &DeckState,
) -> Vec<CardServicePurchaseBlockReason> {
    unengraved_purchase_blocks(deck, 2)
}

pub(super) fn unengraved_purchase_blocks(
    deck: &DeckState,
    required: usize,
) -> Vec<CardServicePurchaseBlockReason> {
    let (_, available) = card_counts(deck);
    (available < required)
        .then_some(CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
            required,
            available,
        })
        .into_iter()
        .collect()
}

fn card_counts(deck: &DeckState) -> (usize, usize) {
    let engraved = deck
        .all_cards
        .iter()
        .filter(|card| card.engraving.is_some())
        .count();
    (engraved, deck.all_cards.len().saturating_sub(engraved))
}

pub(super) fn apply_enhancement(
    deck: &mut DeckState,
    selected_card_ids: &[Vec<usize>],
    suit: Option<u8>,
    polish_pct_raw: i64,
) {
    for card_ids in selected_card_ids {
        for card_id in card_ids {
            deck.modify_card(*card_id, |card| {
                if let Some(suit) = suit {
                    card.suit = suit;
                }
                card.polish_pct_raw = card.polish_pct_raw.saturating_add(polish_pct_raw);
            });
        }
    }
}

pub(super) fn apply_enhancement_service(
    state: &mut crate::CoreState,
    selected_card_ids: &[Vec<usize>],
    suit: Option<u8>,
    polish_pct_raw: i64,
) -> Result<(), crate::CommandError> {
    apply_enhancement(&mut state.deck, selected_card_ids, suit, polish_pct_raw);
    Ok(())
}

pub(super) fn apply_engraving(
    deck: &mut DeckState,
    selected_card_ids: &[Vec<usize>],
    engraving: Option<u8>,
) {
    for card_ids in selected_card_ids {
        for card_id in card_ids {
            deck.modify_card(*card_id, |card| {
                if engraving.is_none() || card.engraving.is_none() {
                    card.engraving = engraving;
                }
            });
        }
    }
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
