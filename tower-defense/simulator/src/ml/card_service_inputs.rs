//! Card-service context on an isolated residual input path. These rows never
//! enter legacy typed or effect pooling.
use super::encoding::{EntityRow, EntitySet};
use super::semantic_candidates::PolicyCandidate;
use super::vocabulary::{engraving_key_id, rank_id, suit_id};
use crate::environment::{AgentAction, Observation};

const SERVICE_CONTEXT_ROW: u32 = 3700;
const SERVICE_RESULT_ROW: u32 = 3701;
const SERVICE_EFFECT_ROW: u32 = 3702;

// Stable semantic IDs: 0 = unsupported, 1 = polish delta, 2 = polish delta
// plus suit replacement. These IDs are deliberately independent of names.
fn effect_kind_id(service: &crate::environment::CardServiceObservation) -> u32 {
    match (
        service.polish_delta_raw.is_some(),
        service.suit_change_raw.is_some(),
    ) {
        (false, _) => 0,
        (true, false) => 1,
        (true, true) => 2,
    }
}

fn filter_id(filter: &td_core::CardSelectionFilterState) -> f32 {
    use td_core::CardSelectionFilterState as F;
    match filter {
        F::Any => 1.0,
        F::Face => 2.0,
        F::Number => 3.0,
        F::Rank(_) => 4.0,
        F::Engraved => 5.0,
        F::NotEngraved => 6.0,
        F::And(_) => 7.0,
        F::Or(_) => 8.0,
    }
}

pub fn state_context(observation: &Observation) -> EntitySet {
    let Some(service) = observation.card_service.as_ref() else {
        return EntitySet::default();
    };
    let filter = service.target_filter.as_ref().map_or(0.0, filter_id);
    let suit_change = service.suit_change_raw.map_or(0.0, |raw| raw as f32 + 1.0);
    EntitySet::new(vec![
        EntityRow::new(
            [
                SERVICE_CONTEXT_ROW,
                service.key_id as u32,
                service.current_step as u32 + 1,
                service.step_count as u32,
            ],
            vec![
                service.required_count as f32,
                service.selected_card_indices.len() as f32,
                service.candidate_card_indices.len() as f32,
                service.current_step as f32 / service.step_count.max(1) as f32,
                service
                    .polish_delta_raw
                    .map_or(0.0, |value| value as f32 / 1_000_000.0),
                filter,
            ],
        ),
        EntityRow::new(
            [
                SERVICE_EFFECT_ROW,
                service.key_id as u32,
                effect_kind_id(service),
                filter as u32,
            ],
            vec![
                service
                    .polish_delta_raw
                    .map_or(0.0, |value| value as f32 / 1_000_000.0),
                suit_change,
                (effect_kind_id(service) != 0) as u8 as f32,
                0.0,
                0.0,
                0.0,
            ],
        ),
    ])
}

pub fn candidate_effect(observation: &Observation, candidate: &PolicyCandidate) -> EntitySet {
    let (Some(service), AgentAction::SelectCardServiceCard { card_index }) =
        (observation.card_service.as_ref(), &candidate.action)
    else {
        return EntitySet::default();
    };
    let Some(card) = observation.deck.all_cards.get(*card_index) else {
        return EntitySet::default();
    };
    let delta = service.polish_delta_raw.unwrap_or(0);
    let next_polish = card.polish_pct_raw.saturating_add(delta);
    let next_suit = service
        .suit_change_raw
        .map(|raw| ["spades", "hearts", "diamonds", "clubs"][raw as usize])
        .unwrap_or(card.suit.as_str());
    EntitySet::new(vec![EntityRow::new(
        [
            SERVICE_RESULT_ROW,
            service.key_id as u32,
            suit_id(&card.suit) as u32,
            rank_id(&card.rank) as u32,
        ],
        vec![
            card.polish_pct_raw as f32 / 1_000_000.0,
            next_polish as f32 / 1_000_000.0,
            delta as f32 / 1_000_000.0,
            card.engraving
                .as_deref()
                .map_or(0.0, |value| engraving_key_id(value) as f32),
            suit_id(next_suit) as f32,
            (service.polish_delta_raw.is_some() || service.suit_change_raw.is_some()) as u8 as f32,
        ],
    )])
}
