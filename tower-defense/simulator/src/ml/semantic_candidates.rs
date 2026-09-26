//! Phase 4 policy candidate set and candidate encoder for the semantic
//! action contract.
//!
//! The candidate set is a superset of both the canonical scripted action and
//! the frozen Phase 3 teacher's S4/1 proposal:
//!
//! - card decision (`Shop`/`CardSelection`): every legal non-build semantic
//!   action (`semantic_non_build_actions`: every `Reroll` subset, shop,
//!   inventory, treasure discard) plus the dense-order top
//!   [`BUILD_TOWER_CANDIDATE_LIMIT`] `BuildTower` pairs from
//!   `DenseBuildTowerScoreTable` (full-map legality, never the legacy
//!   position-limited proposal);
//! - any other decision point: every legal action, with `PlaceTower` limited
//!   to the top [`PLACE_TOWER_CANDIDATE_LIMIT`] in the canonical placement
//!   order.
//!
//! Each candidate is encoded as a small entity set (header row plus
//! kind-specific rows) consumed by `DeepSetsActorCritic`'s existing candidate
//! encoder, so the same network (and therefore the PPO actor) scores it.

use super::encoding::{
    ENTITY_NUMERIC_WIDTH, EntityRow, EntitySet, PaddedEntityBatch, TypedObservation,
};
use super::features::{nearby_tower_occupancy, observation_features, placement_coverage};
use super::vocabulary::{engraving_key_id, rank_id, suit_id, upgrade_key_id};
use crate::environment::{
    ActionKind, AgentAction, CardObservation, DecisionPoint, GameEnvironment, HandItemObservation,
    LegalAction, Observation, TowerTemplateObservation,
};
use crate::joint_action::DenseBuildTowerScoreTable;
use crate::policy_runner::{
    canonical_scripted_semantic_action, canonical_scripted_semantic_action_from_table,
    rank_place_tower_actions,
};
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

pub const POLICY_CANDIDATE_SET_VERSION: u32 = 1;
pub const SEMANTIC_CANDIDATE_ENCODER_VERSION: u32 = 1;
pub const BUILD_TOWER_CANDIDATE_LIMIT: usize = 8;
pub const PLACE_TOWER_CANDIDATE_LIMIT: usize = 8;

const ROW_HEADER: u32 = 3001;
const ROW_TEMPLATE: u32 = 3002;
const ROW_CARD: u32 = 3003;
const ROW_HAND_SUMMARY: u32 = 3004;
const ROW_SHOP: u32 = 3005;
const ROW_INVENTORY: u32 = 3006;
const ROW_TREASURE: u32 = 3007;
const ROW_OWNED_UPGRADE: u32 = 3008;
const KIND_OFFSET: u32 = 3100;
const DECISION_POINT_OFFSET: u32 = 3200;
const MAX_CATEGORICAL_ID: u32 = 4095;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyCandidate {
    pub action: AgentAction,
    pub action_id: String,
    /// Position in the dense-build or canonical placement order, for the
    /// ranked `BuildTower`/`PlaceTower` families only.
    pub family_rank: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct PolicyCandidates {
    pub observation: Observation,
    pub candidates: Vec<PolicyCandidate>,
    pub canonical_action: AgentAction,
}

impl PolicyCandidates {
    pub fn index_of_action_id(&self, action_id: &str) -> Option<usize> {
        self.candidates
            .iter()
            .position(|candidate| candidate.action_id == action_id)
    }

    pub fn canonical_index(&self) -> Option<usize> {
        self.index_of_action_id(&self.canonical_action.action_id())
    }
}

/// Builds the Phase 4 policy candidate set for `environment`'s current
/// decision state together with the canonical scripted action, computing the
/// dense build table at most once.
pub fn policy_candidates(environment: &GameEnvironment) -> Result<PolicyCandidates> {
    let observation = environment.snapshot();
    let mut candidates = Vec::new();
    let canonical_action = if environment.semantic_card_decision_available() {
        let table = DenseBuildTowerScoreTable::compute(environment, &observation);
        for legal in environment.semantic_non_build_actions() {
            push_unique(&mut candidates, legal.action, None);
        }
        for (rank, action) in table
            .top_k_actions(BUILD_TOWER_CANDIDATE_LIMIT)
            .into_iter()
            .enumerate()
        {
            push_unique(&mut candidates, action, Some(rank));
        }
        canonical_scripted_semantic_action_from_table(environment, &observation, &table)?
    } else {
        let legal = environment.semantic_non_build_actions();
        let mut place = Vec::new();
        for legal_action in legal {
            if matches!(legal_action.action, AgentAction::PlaceTower { .. }) {
                place.push(legal_action);
            } else {
                push_unique(&mut candidates, legal_action.action, None);
            }
        }
        for (rank, legal_action) in rank_place_tower_actions(&observation, &place)
            .into_iter()
            .take(PLACE_TOWER_CANDIDATE_LIMIT)
            .enumerate()
        {
            push_unique(&mut candidates, legal_action.action, Some(rank));
        }
        canonical_scripted_semantic_action(environment)?
    };
    if candidates.is_empty() {
        bail!(
            "policy candidate set is empty at a non-terminal decision (state {})",
            environment.state_hash()
        );
    }
    Ok(PolicyCandidates {
        observation,
        candidates,
        canonical_action,
    })
}

fn push_unique(candidates: &mut Vec<PolicyCandidate>, action: AgentAction, rank: Option<usize>) {
    let action_id = action.action_id();
    if candidates
        .iter()
        .any(|candidate| candidate.action_id == action_id)
    {
        return;
    }
    candidates.push(PolicyCandidate {
        action,
        action_id,
        family_rank: rank,
    });
}

pub fn candidates_as_legal_actions(candidates: &[PolicyCandidate]) -> Vec<LegalAction> {
    candidates
        .iter()
        .map(|candidate| LegalAction {
            id: candidate.action_id.clone(),
            action: candidate.action.clone(),
        })
        .collect()
}

pub fn decision_point_index(decision_point: &DecisionPoint) -> usize {
    match decision_point {
        DecisionPoint::Shop => 0,
        DecisionPoint::CardSelection => 1,
        DecisionPoint::CardServiceSelection => 2,
        DecisionPoint::TowerPlacement => 3,
        DecisionPoint::PreDefenseItem => 4,
        DecisionPoint::DamageResponseItem => 5,
        DecisionPoint::TreasureSelection => 6,
        DecisionPoint::Defense => 7,
        DecisionPoint::Terminal => 8,
    }
}

fn categorical(value: u32) -> u32 {
    value.min(MAX_CATEGORICAL_ID)
}

fn row(tag: u32, a: u32, b: u32, c: u32, numeric: [f32; ENTITY_NUMERIC_WIDTH]) -> EntityRow {
    EntityRow::new(
        [
            categorical(tag),
            categorical(a),
            categorical(b),
            categorical(c),
        ],
        numeric
            .map(|value| if value.is_finite() { value } else { 0.0 })
            .to_vec(),
    )
}

fn log_scale(value: f32, unit: f32) -> f32 {
    (1.0 + value.max(0.0) / unit).ln() / 8.0
}

fn nearest_route_norm(observation: &Observation, left: usize, top: usize) -> f32 {
    observation
        .route_coords
        .iter()
        .map(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
        .min()
        .map_or(1.0, |distance| {
            distance as f32 / (observation.map_width + observation.map_height).max(1) as f32
        })
}

fn template_row(template: &TowerTemplateObservation, slot: usize) -> EntityRow {
    row(
        ROW_TEMPLATE,
        template.kind_id as u32 + 1,
        template.suit.as_deref().map_or(0, suit_id) as u32,
        template.rank.as_deref().map_or(0, rank_id) as u32,
        [
            log_scale(template.effective_damage_raw as f32, 1_000.0),
            template.range_raw as f32 / 10_000_000.0,
            template.shoot_interval_ticks as f32 / 120.0,
            template.used_cards.len() as f32 / 5.0,
            (template.on_hit_splashes.len() + template.on_attack_splashes.len()) as f32 / 4.0,
            slot as f32 / 4.0,
        ],
    )
}

fn card_row(card: &CardObservation, selected: bool) -> EntityRow {
    row(
        ROW_CARD,
        suit_id(&card.suit) as u32,
        rank_id(&card.rank) as u32,
        card.engraving.as_deref().map_or(0, engraving_key_id) as u32,
        [
            card.polish_pct_raw as f32 / 1_000.0,
            selected as u8 as f32,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

fn hand_cards(observation: &Observation) -> Vec<&CardObservation> {
    observation
        .hand
        .iter()
        .filter_map(|item| match &item.item {
            HandItemObservation::Card(card) => Some(card),
            HandItemObservation::Tower(_) => None,
        })
        .collect()
}

fn max_multiplicity<'a>(values: impl Iterator<Item = &'a str>) -> usize {
    let mut counts = std::collections::BTreeMap::new();
    for value in values {
        *counts.entry(value).or_insert(0usize) += 1;
    }
    counts.values().copied().max().unwrap_or(0)
}

fn build_template<'a>(
    observation: &'a Observation,
    card_ids: &[usize],
    hand_slot_index: usize,
) -> Option<&'a TowerTemplateObservation> {
    if hand_slot_index == 0 {
        let mut sorted = card_ids.to_vec();
        sorted.sort_unstable();
        observation
            .build_tower_candidates
            .iter()
            .find(|candidate| {
                let mut ids = candidate.card_ids.clone();
                ids.sort_unstable();
                ids == sorted
            })
            .map(|candidate| &candidate.template)
    } else {
        observation
            .extra_tower_card_templates
            .get(hand_slot_index - 1)
    }
}

fn hand_tower(
    observation: &Observation,
    hand_slot_index: usize,
) -> Option<&TowerTemplateObservation> {
    observation.hand.iter().find_map(|item| {
        (item.index == hand_slot_index)
            .then_some(match &item.item {
                HandItemObservation::Tower(tower) => Some(tower),
                HandItemObservation::Card(_) => None,
            })
            .flatten()
    })
}

fn placement_numeric(
    observation: &Observation,
    left: usize,
    top: usize,
    template: Option<&TowerTemplateObservation>,
    rank: Option<usize>,
    limit: usize,
) -> [f32; ENTITY_NUMERIC_WIDTH] {
    let rank_score = rank.map_or(0.0, |rank| 1.0 - rank as f32 / limit as f32);
    [
        rank_score,
        template.map_or(0.0, |template| {
            placement_coverage(observation, left, top, template.range_raw)
        }),
        nearest_route_norm(observation, left, top),
        nearby_tower_occupancy(observation, left, top),
        left as f32 / observation.map_width.max(1) as f32,
        top as f32 / observation.map_height.max(1) as f32,
    ]
}

/// Encodes one candidate as an entity set: a header row (action kind,
/// decision point) and kind-specific rows exposing the information the
/// candidate's effect depends on (shop kind/key/cost, item/treasure keys,
/// rerolled cards and hand strength, tower template and placement geometry).
pub fn encode_candidate(observation: &Observation, candidate: &PolicyCandidate) -> EntitySet {
    let action = &candidate.action;
    let kind = action.kind();
    let decision_point = decision_point_index(&observation.decision_point) as u32;
    let mut header_numeric = [0.0; ENTITY_NUMERIC_WIDTH];
    header_numeric[5] = 1.0;
    let mut rows = Vec::new();
    match action {
        AgentAction::BuildTower {
            card_ids,
            hand_slot_index,
            left,
            top,
        } => {
            let template = build_template(observation, card_ids, *hand_slot_index);
            header_numeric = placement_numeric(
                observation,
                *left,
                *top,
                template,
                candidate.family_rank,
                BUILD_TOWER_CANDIDATE_LIMIT,
            );
            if let Some(template) = template {
                rows.push(template_row(template, *hand_slot_index));
            }
        }
        AgentAction::PlaceTower {
            hand_slot_index,
            left,
            top,
        } => {
            let template = hand_tower(observation, *hand_slot_index);
            header_numeric = placement_numeric(
                observation,
                *left,
                *top,
                template,
                candidate.family_rank,
                PLACE_TOWER_CANDIDATE_LIMIT,
            );
            if let Some(template) = template {
                rows.push(template_row(template, *hand_slot_index));
            }
        }
        AgentAction::Reroll { card_ids } => {
            let cards = hand_cards(observation);
            let rerolled = cards
                .iter()
                .filter(|card| card_ids.contains(&card.id))
                .collect::<Vec<_>>();
            for card in &rerolled {
                rows.push(card_row(card, true));
            }
            rows.push(row(
                ROW_HAND_SUMMARY,
                0,
                0,
                0,
                [
                    rerolled.len() as f32 / cards.len().max(1) as f32,
                    cards.len() as f32 / 10.0,
                    max_multiplicity(cards.iter().map(|card| card.rank.as_str())) as f32 / 5.0,
                    max_multiplicity(cards.iter().map(|card| card.suit.as_str())) as f32 / 5.0,
                    observation.rerolled_count as f32 / 5.0,
                    observation.left_dice as f32 / 20.0,
                ],
            ));
        }
        AgentAction::PurchaseShopItem { slot_index } => {
            if let Some(slot) = observation
                .shop
                .iter()
                .find(|slot| slot.index == *slot_index)
            {
                let cheapest_of_kind = observation
                    .shop
                    .iter()
                    .filter(|other| !other.purchased && other.kind == slot.kind)
                    .map(|other| other.cost)
                    .min()
                    .is_some_and(|cost| cost == slot.cost);
                rows.push(row(
                    ROW_SHOP,
                    slot.kind_id as u32 + 1,
                    slot.key_id as u32 + 1,
                    0,
                    [
                        log_scale(slot.cost as f32, 10.0),
                        slot.cost as f32 / (observation.gold.max(1)) as f32,
                        cheapest_of_kind as u8 as f32,
                        slot.purchased as u8 as f32,
                        log_scale(observation.gold as f32, 10.0),
                        1.0,
                    ],
                ));
            }
        }
        AgentAction::UseInventoryItem { item_index } => {
            if let Some(item) = observation.inventory.get(*item_index) {
                rows.push(row(
                    ROW_INVENTORY,
                    item.key_id as u32 + 1,
                    0,
                    0,
                    [
                        observation.inventory.len() as f32
                            / observation.item_capacity.max(1) as f32,
                        observation.hp_raw.max(0) as f32 / observation.max_hp_raw.max(1) as f32,
                        observation.queued_monster_count as f32 / 100.0,
                        observation.active_monster_count as f32 / 100.0,
                        0.0,
                        1.0,
                    ],
                ));
            }
        }
        AgentAction::SelectTreasure { option_index } => {
            if let Some(key) = observation.treasure_options.get(*option_index) {
                rows.push(row(
                    ROW_TREASURE,
                    upgrade_key_id(key) as u32 + 1,
                    0,
                    0,
                    [
                        observation.owned_upgrades.len() as f32
                            / observation.treasure_capacity.max(1) as f32,
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        1.0,
                    ],
                ));
            }
        }
        AgentAction::DiscardTreasure { upgrade_id } => {
            if let Some(owned) = observation
                .owned_upgrades
                .iter()
                .find(|owned| owned.id == *upgrade_id)
            {
                rows.push(row(
                    ROW_OWNED_UPGRADE,
                    owned.key_id as u32 + 1,
                    0,
                    0,
                    [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                ));
            }
        }
        AgentAction::SelectCardServiceCard { card_index } => {
            if let Some(card) = observation.deck.all_cards.get(*card_index) {
                let selected = observation
                    .card_service
                    .as_ref()
                    .is_some_and(|service| service.selected_card_indices.contains(card_index));
                rows.push(card_row(card, selected));
            }
        }
        AgentAction::RemoveTower { tower_id } => {
            if let Some(tower) = observation
                .towers
                .iter()
                .find(|tower| tower.id == *tower_id)
            {
                header_numeric = placement_numeric(
                    observation,
                    tower.left,
                    tower.top,
                    Some(&tower.template),
                    None,
                    1,
                );
                rows.push(template_row(&tower.template, 0));
            }
        }
        _ => {}
    }
    rows.insert(
        0,
        row(
            ROW_HEADER,
            KIND_OFFSET + kind.index() as u32,
            DECISION_POINT_OFFSET + decision_point,
            candidate.family_rank.map_or(0, |rank| rank as u32 + 1),
            header_numeric,
        ),
    );
    EntitySet::new(rows)
}

/// Policy input for one decision state: global features, typed entity sets
/// and one encoded entity set per candidate, aligned with `legal_mask`.
#[derive(Clone, Debug, PartialEq)]
pub struct EncodedDecision {
    pub global_features: Vec<f32>,
    pub typed: TypedObservation,
    pub candidates: Vec<EntitySet>,
    pub legal_mask: Vec<bool>,
}

pub fn encode_decision(
    observation: &Observation,
    candidates: &[PolicyCandidate],
    legal_mask: Vec<bool>,
) -> EncodedDecision {
    assert_eq!(candidates.len(), legal_mask.len());
    EncodedDecision {
        global_features: observation_features(observation),
        typed: TypedObservation::from_observation(observation),
        candidates: candidates
            .iter()
            .map(|candidate| encode_candidate(observation, candidate))
            .collect(),
        legal_mask,
    }
}

pub fn candidate_batch(decisions: &[&EncodedDecision]) -> PaddedEntityBatch {
    PaddedEntityBatch::from_sets(
        &decisions
            .iter()
            .flat_map(|decision| decision.candidates.iter().cloned())
            .collect::<Vec<_>>(),
    )
}

pub fn action_kind_name(action: &AgentAction) -> &'static str {
    action.kind().wire_name()
}

pub fn is_ranked_family(kind: ActionKind) -> bool {
    matches!(kind, ActionKind::BuildTower | ActionKind::PlaceTower)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use crate::teacher::settle_forced_actions;
    use std::sync::Arc;

    #[test]
    fn candidate_set_contains_canonical_action_and_is_legal_along_canonical_episodes() {
        let config = Arc::new(GameConfig::default_config());
        let mut checked = 0usize;
        for seed in [0u64, 1] {
            let mut environment = GameEnvironment::new(Arc::clone(&config), seed);
            let mut decisions = 0usize;
            while !matches!(environment.decision_point(), DecisionPoint::Terminal) && decisions < 40
            {
                let set = policy_candidates(&environment).expect("candidates");
                let canonical =
                    canonical_scripted_semantic_action(&environment).expect("canonical");
                assert_eq!(set.canonical_action, canonical);
                assert!(set.canonical_index().is_some(), "canonical action missing");
                let mut ids = std::collections::HashSet::new();
                for candidate in &set.candidates {
                    assert!(
                        ids.insert(candidate.action_id.clone()),
                        "duplicate candidate"
                    );
                    assert!(environment.semantic_action_is_legal(&candidate.action));
                    let encoded = encode_candidate(&set.observation, candidate);
                    assert!(!encoded.rows.is_empty());
                    for row in &encoded.rows {
                        assert_eq!(row.numeric.len(), ENTITY_NUMERIC_WIDTH);
                        assert!(row.categorical.iter().all(|value| *value < 4096));
                    }
                }
                checked += 1;
                let mut outcome = environment.semantic_step(canonical).expect("step");
                settle_forced_actions(&mut environment, &mut outcome).expect("settle");
                decisions += 1;
                if outcome.terminated {
                    break;
                }
            }
        }
        assert!(checked > 20);
    }

    #[test]
    fn reroll_candidates_expose_rerolled_card_identity() {
        let config = Arc::new(GameConfig::default_config());
        let environment = (0..32u64)
            .map(|seed| GameEnvironment::new(Arc::clone(&config), seed))
            .find(|environment| environment.semantic_card_decision_available())
            .expect("card decision");
        let set = policy_candidates(&environment).expect("candidates");
        let rerolls = set
            .candidates
            .iter()
            .filter(|candidate| matches!(candidate.action, AgentAction::Reroll { .. }))
            .map(|candidate| encode_candidate(&set.observation, candidate))
            .collect::<Vec<_>>();
        assert!(rerolls.len() >= 2);
        for (index, left) in rerolls.iter().enumerate() {
            for right in &rerolls[index + 1..] {
                assert_ne!(left, right);
            }
        }
    }
}
