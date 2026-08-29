//! Shared explicit-seed execution for environment policies.

use crate::config::GameConfig;
use anyhow::{Result, bail};
use rand::RngCore;
use rayon::prelude::*;
use std::{collections::BTreeMap, sync::Arc};
use td_core::{domain, rng_for, uniform_index};

use super::environment::{
    AgentAction, DecisionPoint, EnvironmentError, EnvironmentMetrics, GameEnvironment, LegalAction,
    Observation, RewardConfig, StepOutcome,
};
use serde::{Deserialize, Serialize};

pub const DEFAULT_MAX_DECISIONS_PER_EPISODE: usize = 10_000;
pub const NO_PROGRESS_CYCLE_LIMIT: usize = 32;
const PROGRESS_WINDOW_SIZE: usize = NO_PROGRESS_CYCLE_LIMIT * 2;

#[derive(Default)]
struct ProgressTracker {
    recent: Vec<String>,
}

impl ProgressTracker {
    fn observe(&mut self, fingerprint: String) -> bool {
        self.recent.push(fingerprint);
        if self.recent.len() > PROGRESS_WINDOW_SIZE {
            self.recent.remove(0);
        }
        for period in 1..=NO_PROGRESS_CYCLE_LIMIT {
            if self.recent.len() < period * 2 {
                continue;
            }
            let split = self.recent.len() - period;
            if self.recent[split..] == self.recent[split - period..split] {
                return true;
            }
        }
        false
    }
}

#[derive(Default)]
struct ActionHistoryGuard {
    previous: Option<(String, AgentAction)>,
}

impl ActionHistoryGuard {
    fn effective_actions(
        &self,
        fingerprint: &str,
        legal_actions: &[LegalAction],
    ) -> Vec<LegalAction> {
        let Some((previous_fingerprint, previous_action)) = &self.previous else {
            return legal_actions.to_vec();
        };
        let mut filtered = legal_actions
            .iter()
            .filter(|legal| {
                !is_immediate_inverse(previous_action, &legal.action)
                    && !(previous_fingerprint == fingerprint && previous_action == &legal.action)
            })
            .cloned()
            .collect::<Vec<_>>();
        if filtered.is_empty() {
            filtered = legal_actions.to_vec();
        }
        filtered
    }

    fn observe(&mut self, pre_fingerprint: String, action: AgentAction) {
        self.previous = Some((pre_fingerprint, action));
    }
}

fn is_immediate_inverse(previous: &AgentAction, current: &AgentAction) -> bool {
    match (previous, current) {
        (
            AgentAction::BeginRerollSelection | AgentAction::BeginTowerSelection,
            AgentAction::CancelCardSelection,
        ) => true,
        (
            AgentAction::SelectHandCard { hand_slot_index },
            AgentAction::DeselectHandCard {
                hand_slot_index: current_slot,
            },
        )
        | (
            AgentAction::DeselectHandCard { hand_slot_index },
            AgentAction::SelectHandCard {
                hand_slot_index: current_slot,
            },
        ) => hand_slot_index == current_slot,
        _ => false,
    }
}

fn finish_cycle_outcome(outcome: &mut StepOutcome, is_cycle: bool, reward_config: &RewardConfig) {
    if !is_cycle || outcome.terminated {
        return;
    }
    outcome.info.no_progress_cycle = true;
    outcome.reward.shaping.insert(
        "no_progress_cycle_penalty".to_string(),
        reward_config.no_progress_cycle_penalty,
    );
}

pub trait EnvironmentPolicy: Send {
    fn choose_action(
        &mut self,
        observation: &Observation,
        legal_actions: &[LegalAction],
    ) -> Result<AgentAction>;
}

fn should_scripted_reroll(observation: &Observation) -> bool {
    scripted_reroll_indices(observation).len() >= 3
}

fn scripted_reroll_indices(observation: &Observation) -> Vec<usize> {
    let mut rank_counts = BTreeMap::new();
    let mut suit_counts = BTreeMap::new();
    let mut card_indices = Vec::new();
    for item in &observation.hand {
        let super::environment::HandItemObservation::Card(card) = &item.item else {
            continue;
        };
        *rank_counts.entry(card.rank.clone()).or_insert(0usize) += 1;
        *suit_counts.entry(card.suit.clone()).or_insert(0usize) += 1;
        card_indices.push(item.index);
    }
    let has_pair = rank_counts.values().any(|count| *count >= 2);
    let has_flush_potential = suit_counts.values().any(|count| *count >= 3);
    if has_pair || has_flush_potential {
        Vec::new()
    } else {
        card_indices
    }
}

impl<F> EnvironmentPolicy for F
where
    F: FnMut(&Observation, &[LegalAction]) -> Result<AgentAction> + Send,
{
    fn choose_action(
        &mut self,
        observation: &Observation,
        legal_actions: &[LegalAction],
    ) -> Result<AgentAction> {
        self(observation, legal_actions)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyRunnerConfig {
    pub max_decisions_per_episode: usize,
    pub record_steps: bool,
    pub max_stage: Option<usize>,
    pub reward_config: RewardConfig,
}

impl Default for PolicyRunnerConfig {
    fn default() -> Self {
        Self {
            max_decisions_per_episode: DEFAULT_MAX_DECISIONS_PER_EPISODE,
            record_steps: false,
            max_stage: None,
            reward_config: RewardConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyStep {
    pub observation: Observation,
    pub legal_actions: Vec<LegalAction>,
    pub action: AgentAction,
    pub outcome: StepOutcome,
    pub pre_progress_fingerprint: String,
    pub post_progress_fingerprint: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ForcedActionStats {
    pub total: usize,
    pub by_decision_point: std::collections::BTreeMap<String, usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptedOracleStep {
    pub decision_point: DecisionPoint,
    pub legal_action_ids: Vec<String>,
    pub selected_index: usize,
    pub selected_action_id: String,
    pub pre_progress_fingerprint: String,
    pub post_progress_fingerprint: String,
    pub state_hash: String,
    pub reward: super::environment::RewardComponents,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptedOracleTrace {
    pub seed: u64,
    pub steps: Vec<ScriptedOracleStep>,
    pub decision_count: usize,
    pub terminated: bool,
    pub truncated: bool,
    pub final_state_hash: String,
    pub final_stage: usize,
    pub total_towers_placed: usize,
    pub final_sim_tick: u64,
}

pub fn run_scripted_oracle(game_config: Arc<GameConfig>, seed: u64) -> Result<ScriptedOracleTrace> {
    run_scripted_oracle_with_stage_limit(game_config, seed, Some(1))
}

pub fn run_scripted_oracle_with_stage_limit(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_stage: Option<usize>,
) -> Result<ScriptedOracleTrace> {
    let episode = run_episode(
        game_config,
        seed,
        &PolicyRunnerConfig {
            max_decisions_per_episode: 256,
            record_steps: true,
            max_stage,
            reward_config: RewardConfig::default(),
        },
        scripted_oracle_action,
    )?;
    let policy_steps = episode
        .steps
        .as_ref()
        .expect("scripted oracle records steps");
    let steps = policy_steps
        .iter()
        .map(|step| {
            let selected_index = step
                .legal_actions
                .iter()
                .position(|legal| legal.action == step.action)
                .expect("scripted action must be legal");
            ScriptedOracleStep {
                decision_point: step.observation.decision_point.clone(),
                legal_action_ids: step
                    .legal_actions
                    .iter()
                    .map(|legal| legal.id.clone())
                    .collect(),
                selected_index,
                selected_action_id: step.action.action_id(),
                pre_progress_fingerprint: step.pre_progress_fingerprint.clone(),
                post_progress_fingerprint: step.post_progress_fingerprint.clone(),
                state_hash: step.outcome.state_hash.clone(),
                reward: step.outcome.reward.clone(),
            }
        })
        .collect();
    Ok(ScriptedOracleTrace {
        seed: episode.seed,
        steps,
        decision_count: episode.decision_count,
        terminated: episode.terminated,
        truncated: episode.truncated,
        final_state_hash: episode.final_state_hash,
        final_stage: episode.final_observation.stage,
        total_towers_placed: episode.metrics.total_towers_placed,
        final_sim_tick: episode.final_observation.sim_tick,
    })
}

pub fn run_scripted_oracle_trajectory(
    game_config: Arc<GameConfig>,
    seed: u64,
) -> Result<crate::trajectory::Trajectory> {
    let episode = run_episode(
        Arc::clone(&game_config),
        seed,
        &PolicyRunnerConfig {
            max_decisions_per_episode: 256,
            record_steps: true,
            max_stage: None,
            reward_config: RewardConfig::default(),
        },
        scripted_oracle_action,
    )?;
    let steps = episode
        .steps
        .as_ref()
        .expect("scripted oracle records steps");
    let mut trajectory =
        crate::trajectory::Trajectory::from_policy_steps(game_config.as_ref(), seed, steps);
    trajectory.set_outcome(crate::trajectory::TrajectoryOutcome {
        victory: episode.victory,
        clear_rate: episode.clear_rate,
        terminated: episode.terminated,
        truncated: episode.truncated,
        termination_reason: episode.termination_reason,
        final_stage: episode.final_observation.stage,
        episode_return: episode.episode_return,
    });
    Ok(trajectory)
}

pub fn run_scripted_expert_trajectory(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_decisions_per_episode: usize,
) -> Result<crate::trajectory::Trajectory> {
    run_expert_trajectory(
        game_config,
        seed,
        max_decisions_per_episode,
        scripted_expert_action,
    )
}

fn run_expert_trajectory<P>(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_decisions_per_episode: usize,
    policy: P,
) -> Result<crate::trajectory::Trajectory>
where
    P: FnMut(&Observation, &[LegalAction]) -> Result<AgentAction> + Send,
{
    let episode = run_episode(
        Arc::clone(&game_config),
        seed,
        &PolicyRunnerConfig {
            max_decisions_per_episode,
            record_steps: true,
            max_stage: None,
            reward_config: RewardConfig::default(),
        },
        policy,
    )?;
    let steps = episode
        .steps
        .as_ref()
        .expect("scripted expert records steps");
    let mut trajectory =
        crate::trajectory::Trajectory::from_policy_steps(game_config.as_ref(), seed, steps);
    trajectory.set_outcome(crate::trajectory::TrajectoryOutcome {
        victory: episode.victory,
        clear_rate: episode.clear_rate,
        terminated: episode.terminated,
        truncated: episode.truncated,
        termination_reason: episode.termination_reason,
        final_stage: episode.final_observation.stage,
        episode_return: episode.episode_return,
    });
    Ok(trajectory)
}

pub fn run_spiral_expert_trajectory(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_decisions_per_episode: usize,
) -> Result<crate::trajectory::Trajectory> {
    run_expert_trajectory(
        game_config,
        seed,
        max_decisions_per_episode,
        spiral_expert_action,
    )
}

pub fn run_monte_carlo_expert_trajectory(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_decisions_per_episode: usize,
) -> Result<crate::trajectory::Trajectory> {
    let mut rng = rng_for(seed, domain::ML_TOWER_EXPERT, &[seed]);
    run_expert_trajectory(
        game_config,
        seed,
        max_decisions_per_episode,
        |observation, legal_actions| {
            monte_carlo_expert_action(observation, legal_actions, &mut rng)
        },
    )
}

pub fn run_item_expert_trajectory(
    game_config: Arc<GameConfig>,
    seed: u64,
    max_decisions_per_episode: usize,
) -> Result<crate::trajectory::Trajectory> {
    run_expert_trajectory(
        game_config,
        seed,
        max_decisions_per_episode,
        item_expert_action,
    )
}

const SPIRAL_POSITIONS: &[(usize, usize)] = &[
    (18, 18),
    (18, 16),
    (20, 17),
    (16, 17),
    (14, 17),
    (12, 17),
    (10, 17),
    (8, 17),
    (6, 17),
    (4, 15),
    (2, 17),
    (0, 17),
    (15, 20),
    (17, 21),
    (19, 21),
    (21, 20),
    (23, 19),
    (23, 17),
    (23, 15),
    (21, 14),
    (19, 13),
    (17, 13),
    (15, 14),
    (12, 19),
    (12, 21),
    (18, 11),
    (18, 9),
    (18, 7),
    (19, 5),
    (18, 3),
    (18, 1),
    (20, 0),
    (14, 23),
    (16, 24),
    (18, 24),
    (20, 24),
    (22, 23),
    (24, 22),
    (26, 20),
    (26, 18),
    (26, 16),
    (26, 14),
    (24, 12),
    (22, 11),
    (12, 15),
    (12, 13),
    (14, 11),
    (3, 19),
    (5, 20),
    (7, 20),
    (9, 20),
    (9, 22),
    (11, 24),
    (13, 26),
    (15, 27),
    (16, 29),
    (16, 31),
    (18, 32),
    (19, 30),
    (19, 28),
    (21, 27),
    (23, 26),
    (25, 25),
    (27, 23),
    (29, 21),
    (29, 19),
    (29, 17),
    (29, 15),
    (31, 15),
    (32, 17),
    (34, 17),
];

fn spiral_expert_action(
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Result<AgentAction> {
    if observation.decision_point == DecisionPoint::TowerPlacement {
        if let Some(action) = legal_actions.iter().find_map(|legal| {
            let AgentAction::RemoveTower { tower_id } = legal.action else {
                return None;
            };
            observation
                .towers
                .iter()
                .any(|tower| tower.id == tower_id && tower.left == 0 && tower.top == 17)
                .then(|| legal.action.clone())
        }) {
            return Ok(action);
        }
        for &(left, top) in SPIRAL_POSITIONS {
            if let Some(action) = legal_actions.iter().find_map(|legal| {
                matches!(
                    legal.action,
                    AgentAction::PlaceTower {
                        left: action_left,
                        top: action_top,
                        ..
                    } if action_left == left && action_top == top
                )
                .then(|| legal.action.clone())
            }) {
                return Ok(action);
            }
        }
    }
    scripted_expert_action(observation, legal_actions)
}

fn monte_carlo_expert_action(
    observation: &Observation,
    legal_actions: &[LegalAction],
    rng: &mut impl RngCore,
) -> Result<AgentAction> {
    if observation.decision_point == DecisionPoint::CardSelection {
        if observation.card_selection_purpose.is_none() {
            if observation.rerolled_count == 0
                && monte_carlo_reroll_indices(observation).is_some()
                && let Some(action) = legal_actions
                    .iter()
                    .find(|legal| matches!(legal.action, AgentAction::BeginRerollSelection))
            {
                return Ok(action.action.clone());
            }
        } else if observation.card_selection_purpose.as_deref() == Some("reroll") {
            let discard_indices = monte_carlo_reroll_indices(observation).unwrap_or_default();
            if let Some(action) = legal_actions.iter().find_map(|legal| {
                let AgentAction::SelectHandCard { hand_slot_index } = legal.action else {
                    return None;
                };
                (!observation
                    .selected_hand_slot_indices
                    .contains(&hand_slot_index)
                    && discard_indices.contains(&hand_slot_index))
                .then(|| legal.action.clone())
            }) {
                return Ok(action);
            }
            if let Some(action) = legal_actions
                .iter()
                .find(|legal| matches!(legal.action, AgentAction::ConfirmCardSelection))
            {
                return Ok(action.action.clone());
            }
        }
    }
    if observation.decision_point != DecisionPoint::TowerPlacement {
        return scripted_expert_action(observation, legal_actions);
    }
    let placement_actions = legal_actions
        .iter()
        .filter(|legal| matches!(legal.action, AgentAction::PlaceTower { .. }))
        .collect::<Vec<_>>();
    if placement_actions.is_empty() {
        return scripted_expert_action(observation, legal_actions);
    }
    let sample_count = placement_actions.len().min(32);
    let mut best: Option<(AgentAction, f32)> = None;
    for _ in 0..sample_count {
        let legal = placement_actions[uniform_index(rng, placement_actions.len())];
        let AgentAction::PlaceTower {
            hand_slot_index,
            left,
            top,
        } = legal.action
        else {
            continue;
        };
        let Some(tower) = observation
            .hand
            .get(hand_slot_index)
            .and_then(|item| match &item.item {
                super::environment::HandItemObservation::Tower(tower) => Some(tower),
                super::environment::HandItemObservation::Card(_) => None,
            })
        else {
            continue;
        };
        let coverage = crate::ml::features::placement_coverage(observation, left, top, &tower.kind);
        let score = coverage * 1_000.0 + tower.damage_raw as f32 / 10_000.0;
        if best
            .as_ref()
            .is_none_or(|(_, best_score)| score > *best_score)
        {
            best = Some((legal.action.clone(), score));
        }
    }
    best.map(|(action, _)| action)
        .or_else(|| placement_actions.first().map(|legal| legal.action.clone()))
        .ok_or_else(|| anyhow::anyhow!("monte carlo expert found no placement"))
}

fn item_expert_action(
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Result<AgentAction> {
    if observation.decision_point == DecisionPoint::PreDefenseItem
        && let Some(action) = legal_actions
            .iter()
            .filter_map(|legal| {
                let AgentAction::UseInventoryItem { item_index } = legal.action else {
                    return None;
                };
                let item = observation.inventory.get(item_index)?;
                Some((
                    item_priority(&item.key),
                    item.key.clone(),
                    legal.action.clone(),
                ))
            })
            .max_by_key(|(priority, key, action)| (*priority, key.clone(), action.action_id()))
    {
        return Ok(action.2);
    }
    scripted_expert_action(observation, legal_actions)
}

fn item_priority(key: &str) -> usize {
    match key {
        "rice_ball" | "lunch_box" | "gimbap" | "bread" => 5,
        "milk" => 4,
        "candy" | "cookie" | "donut" => 3,
        "lump_sugar" => 2,
        _ => 1,
    }
}

fn monte_carlo_reroll_indices(observation: &Observation) -> Option<Vec<usize>> {
    let cards = observation
        .hand
        .iter()
        .filter_map(|item| match &item.item {
            super::environment::HandItemObservation::Card(card) => Some((item.index, card)),
            super::environment::HandItemObservation::Tower(_) => None,
        })
        .collect::<Vec<_>>();
    if cards.len() < 2 || cards.len() > 12 || observation.deck.draw_cards.is_empty() {
        return None;
    }
    let current_score = card_dps_proxy(
        &cards
            .iter()
            .map(|(_, card)| (*card).clone())
            .collect::<Vec<_>>(),
    );
    let mut rng = rng_for(
        observation.sim_tick,
        domain::ML_TOWER_EXPERT,
        &[cards.len() as u64],
    );
    let mut best_score = current_score;
    let mut best_mask = 0usize;
    for mask in 1usize..(1usize << cards.len()) {
        let discard_count = mask.count_ones() as usize;
        if discard_count > observation.deck.draw_cards.len() {
            continue;
        }
        let mut expected_score = 0.0;
        for _ in 0..8 {
            let mut candidate = cards
                .iter()
                .enumerate()
                .filter(|(index, _)| (mask & (1 << index)) == 0)
                .map(|(_, (_, card))| (*card).clone())
                .collect::<Vec<_>>();
            let mut draw_indices = (0..observation.deck.draw_cards.len()).collect::<Vec<_>>();
            for _ in 0..discard_count {
                let index = uniform_index(&mut rng, draw_indices.len());
                let draw_index = draw_indices.swap_remove(index);
                candidate.push(observation.deck.draw_cards[draw_index].clone());
            }
            expected_score += card_dps_proxy(&candidate);
        }
        expected_score /= 8.0;
        if expected_score > best_score + 0.05 {
            best_score = expected_score;
            best_mask = mask;
        }
    }
    (best_mask != 0).then(|| {
        cards
            .iter()
            .enumerate()
            .filter_map(|(index, (slot, _))| ((best_mask & (1 << index)) != 0).then_some(*slot))
            .collect()
    })
}

fn card_dps_proxy(cards: &[crate::environment::CardObservation]) -> f32 {
    let mut ranks = BTreeMap::new();
    let mut suits = BTreeMap::new();
    let mut score = 0.0;
    for card in cards {
        *ranks.entry(card.rank.clone()).or_insert(0usize) += 1;
        *suits.entry(card.suit.clone()).or_insert(0usize) += 1;
        score += rank_value(&card.rank) as f32;
        score += card.polish_pct_raw as f32 / 1_000.0;
        score += match card.engraving.as_deref() {
            Some("overcharge") => 5.0,
            Some("magnet") => 4.0,
            Some("spinning_top") => 3.0,
            Some("cactus") => 2.0,
            _ => 0.0,
        };
    }
    score += ranks
        .values()
        .map(|count| (*count * *count) as f32 * 2.0)
        .sum::<f32>();
    score += suits
        .values()
        .map(|count| *count as f32 * *count as f32)
        .fold(0.0, f32::max);
    score
}

fn rank_value(rank: &str) -> usize {
    match rank {
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "jack" => 11,
        "queen" => 12,
        "king" => 13,
        "ace" => 14,
        _ => 0,
    }
}

fn scripted_expert_action(
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Result<AgentAction> {
    let first = || {
        legal_actions
            .first()
            .map(|legal| legal.action.clone())
            .ok_or_else(|| anyhow::anyhow!("scripted expert found no legal action"))
    };
    match observation.decision_point {
        DecisionPoint::Shop => legal_actions
            .iter()
            .filter_map(|legal| {
                let AgentAction::PurchaseShopItem { slot_index } = legal.action else {
                    return None;
                };
                let slot = observation
                    .shop
                    .iter()
                    .find(|slot| slot.index == slot_index)?;
                let priority = match slot.kind.as_str() {
                    "card_service" => 0,
                    "upgrade" => 1,
                    "item" => 2,
                    _ => 3,
                };
                Some((priority, slot.cost, legal.action.clone()))
            })
            .min_by_key(|(priority, cost, action)| (*priority, *cost, action.action_id()))
            .map(|(_, _, action)| action)
            .or_else(|| {
                legal_actions
                    .iter()
                    .find(|legal| matches!(legal.action, AgentAction::StartSelectingTower))
                    .map(|legal| legal.action.clone())
            })
            .ok_or_else(|| anyhow::anyhow!("scripted expert found no shop action")),
        DecisionPoint::CardSelection => {
            if observation.card_selection_purpose.is_none() {
                if observation.rerolled_count == 0
                    && should_scripted_reroll(observation)
                    && let Some(action) = legal_actions
                        .iter()
                        .find(|legal| matches!(legal.action, AgentAction::BeginRerollSelection))
                {
                    return Ok(action.action.clone());
                }
                return Ok(AgentAction::BeginTowerSelection);
            }
            if observation.card_selection_purpose.as_deref() == Some("reroll") {
                let discard_indices = scripted_reroll_indices(observation);
                if let Some(action) = legal_actions.iter().find_map(|legal| {
                    let AgentAction::SelectHandCard { hand_slot_index } = legal.action else {
                        return None;
                    };
                    discard_indices
                        .contains(&hand_slot_index)
                        .then(|| legal.action.clone())
                }) {
                    return Ok(action);
                }
                return legal_actions
                    .iter()
                    .find(|legal| matches!(legal.action, AgentAction::ConfirmCardSelection))
                    .map(|legal| legal.action.clone())
                    .ok_or_else(|| {
                        anyhow::anyhow!("scripted expert found no reroll confirmation")
                    });
            }
            if let Some(action) = legal_actions.iter().find_map(|legal| {
                let AgentAction::SelectHandCard { hand_slot_index } = legal.action else {
                    return None;
                };
                (!observation
                    .selected_hand_slot_indices
                    .contains(&hand_slot_index))
                .then(|| legal.action.clone())
            }) {
                return Ok(action);
            }
            legal_actions
                .iter()
                .find(|legal| matches!(legal.action, AgentAction::ConfirmCardSelection))
                .map(|legal| legal.action.clone())
                .or_else(|| legal_actions.first().map(|legal| legal.action.clone()))
                .ok_or_else(|| anyhow::anyhow!("scripted expert found no card action"))
        }
        DecisionPoint::TowerPlacement => {
            let route = &observation.route_coords;
            legal_actions
                .iter()
                .filter_map(|legal| {
                    let AgentAction::PlaceTower {
                        hand_slot_index,
                        left,
                        top,
                    } = legal.action
                    else {
                        return None;
                    };
                    let tower = observation.hand.iter().find_map(|item| {
                        (item.index == hand_slot_index).then_some(match &item.item {
                            super::environment::HandItemObservation::Tower(tower) => tower,
                            super::environment::HandItemObservation::Card(_) => return None,
                        })
                    })?;
                    let range_raw = tower_range_raw(&tower.kind);
                    let covered_route = route
                        .iter()
                        .filter(|coord| {
                            let dx = (coord.x as i64 - left as i64)
                                .saturating_mul(1_000_000)
                                .saturating_sub(500_000);
                            let dy = (coord.y as i64 - top as i64)
                                .saturating_mul(1_000_000)
                                .saturating_sub(500_000);
                            dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy))
                                <= range_raw.saturating_mul(range_raw)
                        })
                        .count();
                    let nearest_route = route
                        .iter()
                        .map(|coord| coord.x.abs_diff(left) + coord.y.abs_diff(top))
                        .min()
                        .unwrap_or(usize::MAX);
                    Some((
                        covered_route,
                        nearest_route,
                        tower.damage_raw,
                        legal.action.clone(),
                    ))
                })
                .max_by_key(|(covered_route, nearest_route, damage, action)| {
                    (
                        *covered_route,
                        std::cmp::Reverse(*nearest_route),
                        *damage,
                        action.action_id(),
                    )
                })
                .map(|(_, _, _, action)| action)
                .or_else(|| {
                    legal_actions
                        .iter()
                        .find(|legal| matches!(legal.action, AgentAction::StartDefense))
                        .map(|legal| legal.action.clone())
                })
                .ok_or_else(|| anyhow::anyhow!("scripted expert found no placement"))
        }
        DecisionPoint::CardServiceSelection => {
            let selected = observation
                .card_service
                .as_ref()
                .map(|service| service.selected_card_indices.as_slice())
                .unwrap_or(&[]);
            if observation
                .card_service
                .as_ref()
                .is_some_and(|service| selected.len() >= service.required_count)
            {
                return legal_actions
                    .iter()
                    .find(|legal| matches!(legal.action, AgentAction::ConfirmCardServiceSelection))
                    .map(|legal| legal.action.clone())
                    .ok_or_else(|| {
                        anyhow::anyhow!("scripted expert found no card service confirmation")
                    });
            }
            legal_actions
                .iter()
                .find(|legal| matches!(legal.action, AgentAction::SelectCardServiceCard { card_index } if !selected.contains(&card_index)))
                .or_else(|| legal_actions.iter().find(|legal| matches!(legal.action, AgentAction::ConfirmCardServiceSelection)))
                .map(|legal| legal.action.clone())
                .ok_or_else(|| anyhow::anyhow!("scripted expert found no card service action"))
        }
        DecisionPoint::TreasureSelection => legal_actions
            .iter()
            .filter_map(|legal| {
                let AgentAction::SelectTreasure { option_index } = legal.action else {
                    return None;
                };
                let key = observation.treasure_options.get(option_index)?;
                let priority = match key.as_str() {
                    "perfect_pottery" | "resolution" | "ice_cream" | "popcorn" => 0,
                    "dice_bundle" | "energy_drink" | "cat" | "gift_box" => 1,
                    "carrot" | "strawberry" | "tape" | "mirror" => 2,
                    _ => 3,
                };
                Some((priority, key.clone(), legal.action.clone()))
            })
            .min_by_key(|(priority, key, action)| (*priority, key.clone(), action.action_id()))
            .map(|(_, _, action)| action)
            .ok_or_else(|| anyhow::anyhow!("scripted expert found no treasure action"))
            .or_else(|_| first()),
        DecisionPoint::DamageResponseItem => legal_actions
            .iter()
            .filter_map(|legal| {
                let AgentAction::UseInventoryItem { item_index } = legal.action else {
                    return None;
                };
                let item = observation.inventory.get(item_index)?;
                let priority = match item.key.as_str() {
                    "rice_ball" | "lunch_box" | "gimbap" | "bread" => 0,
                    "milk" => 1,
                    "candy" | "cookie" | "donut" => 2,
                    "lump_sugar" => 3,
                    _ => 4,
                };
                Some((priority, item.key.clone(), legal.action.clone()))
            })
            .min_by_key(|(priority, key, action)| (*priority, key.clone(), action.action_id()))
            .map(|(_, _, action)| action)
            .or_else(|| {
                legal_actions
                    .iter()
                    .find(|legal| matches!(legal.action, AgentAction::Continue))
                    .map(|legal| legal.action.clone())
            })
            .ok_or_else(|| anyhow::anyhow!("scripted expert found no item action")),
        DecisionPoint::PreDefenseItem => legal_actions
            .iter()
            .find(|legal| matches!(legal.action, AgentAction::Continue))
            .map(|legal| legal.action.clone())
            .or_else(|| legal_actions.first().map(|legal| legal.action.clone()))
            .ok_or_else(|| anyhow::anyhow!("scripted expert found no pre-defense action")),
        DecisionPoint::Defense => Ok(AgentAction::Continue),
        DecisionPoint::Terminal => first(),
    }
}

fn tower_range_raw(kind: &str) -> i64 {
    match kind {
        "rubber_cone" | "high" => 4_000_000,
        "one_pair" => 5_000_000,
        "two_pair" => 6_000_000,
        "three_of_a_kind" => 7_000_000,
        "straight" | "flush" => 9_000_000,
        "full_house" | "four_of_a_kind" => 11_000_000,
        "straight_flush" => 14_000_000,
        "royal_flush" => 15_000_000,
        _ => 4_000_000,
    }
}

fn scripted_oracle_action(
    observation: &Observation,
    legal_actions: &[LegalAction],
) -> Result<AgentAction> {
    let action = match observation.decision_point {
        DecisionPoint::Shop => AgentAction::StartSelectingTower,
        DecisionPoint::CardSelection => {
            if observation.card_selection_purpose.is_none() {
                AgentAction::BeginTowerSelection
            } else if observation.selected_hand_slot_indices.is_empty() {
                legal_actions
                    .iter()
                    .find_map(|legal| {
                        matches!(legal.action, AgentAction::SelectHandCard { .. })
                            .then(|| legal.action.clone())
                    })
                    .ok_or_else(|| anyhow::anyhow!("tower oracle found no card selection"))?
            } else {
                AgentAction::ConfirmCardSelection
            }
        }
        DecisionPoint::TowerPlacement => legal_actions
            .iter()
            .find_map(|legal| {
                matches!(legal.action, AgentAction::PlaceTower { .. }).then(|| legal.action.clone())
            })
            .or_else(|| {
                legal_actions.iter().find_map(|legal| {
                    matches!(legal.action, AgentAction::StartDefense).then(|| legal.action.clone())
                })
            })
            .ok_or_else(|| anyhow::anyhow!("tower oracle found no placement"))?,
        DecisionPoint::PreDefenseItem
        | DecisionPoint::DamageResponseItem
        | DecisionPoint::Defense => AgentAction::Continue,
        _ => legal_actions
            .first()
            .map(|legal| legal.action.clone())
            .ok_or_else(|| anyhow::anyhow!("scripted oracle found no legal action"))?,
    };
    Ok(action)
}

#[derive(Clone, Debug, PartialEq)]
pub struct EpisodeResult {
    pub seed: u64,
    pub decision_count: usize,
    pub forced_actions: ForcedActionStats,
    pub terminated: bool,
    pub truncated: bool,
    pub victory: bool,
    pub clear_rate: f32,
    pub final_observation: Observation,
    pub final_state_hash: String,
    pub metrics: EnvironmentMetrics,
    pub episode_return: f32,
    pub termination_reason: super::environment::StepReason,
    pub steps: Option<Vec<PolicyStep>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BatchResult {
    pub seeds: Vec<u64>,
    pub episodes: Vec<EpisodeResult>,
}

pub fn run_batch<P, F>(
    game_config: Arc<GameConfig>,
    seeds: &[u64],
    runner_config: &PolicyRunnerConfig,
    policy_factory: F,
) -> Result<BatchResult>
where
    P: EnvironmentPolicy,
    F: Fn(u64) -> P + Sync,
{
    let mut episodes = seeds
        .par_iter()
        .map(|&seed| {
            run_episode(
                Arc::clone(&game_config),
                seed,
                runner_config,
                policy_factory(seed),
            )
        })
        .collect::<Result<Vec<_>>>()?;
    episodes.sort_by_key(|episode| episode.seed);
    Ok(BatchResult {
        seeds: seeds.to_vec(),
        episodes,
    })
}

pub fn run_episode<P>(
    game_config: Arc<GameConfig>,
    seed: u64,
    runner_config: &PolicyRunnerConfig,
    mut policy: P,
) -> Result<EpisodeResult>
where
    P: EnvironmentPolicy,
{
    let max_decisions = runner_config.max_decisions_per_episode;
    if max_decisions == 0 {
        bail!("max decisions per episode must be positive");
    }
    runner_config
        .reward_config
        .validate()
        .map_err(anyhow::Error::msg)?;

    let mut environment = match runner_config.max_stage {
        Some(max_stage) => GameEnvironment::new_with_stage_limit(
            game_config,
            seed,
            runner_config.reward_config.clone(),
            max_stage,
        ),
        None => GameEnvironment::new_with_reward_config(
            game_config,
            seed,
            runner_config.reward_config.clone(),
        ),
    };
    environment.set_max_advance_ticks(
        (max_decisions as u64).saturating_mul(super::environment::DEFAULT_MAX_ADVANCE_TICKS),
    );
    let mut steps = runner_config.record_steps.then(Vec::new);
    let mut terminated = false;
    let mut truncated = false;
    let mut decision_count = 0;
    let mut episode_return = 0.0;
    let mut termination_reason = super::environment::StepReason::Terminal;
    let mut progress_tracker = ProgressTracker::default();
    let mut action_history_guard = ActionHistoryGuard::default();
    let mut forced_actions = ForcedActionStats::default();
    progress_tracker.observe(environment.progress_fingerprint());

    while decision_count < max_decisions {
        if matches!(environment.decision_point(), DecisionPoint::Terminal) {
            terminated = true;
            break;
        }

        let observation = environment.snapshot();
        let pre_progress_fingerprint = environment.progress_fingerprint();
        let canonical_legal_actions = environment.legal_actions();
        if canonical_legal_actions.is_empty() {
            bail!(
                "environment reached a non-terminal state without legal actions at seed {} (state {})",
                seed,
                environment.state_hash()
            );
        }
        let legal_actions = action_history_guard
            .effective_actions(&pre_progress_fingerprint, &canonical_legal_actions);
        let action = policy.choose_action(&observation, &legal_actions)?;
        action_history_guard.observe(pre_progress_fingerprint.clone(), action.clone());
        let mut outcome = environment
            .step(action.clone())
            .map_err(|error| runner_environment_error(seed, error))?;
        while !outcome.terminated && !outcome.truncated {
            let Some(forced_action) = environment.forced_action() else {
                break;
            };
            let forced_point = environment.decision_point();
            let forced_outcome = environment
                .step(forced_action)
                .map_err(|error| runner_environment_error(seed, error))?;
            forced_actions.total += 1;
            *forced_actions
                .by_decision_point
                .entry(format!("{forced_point:?}"))
                .or_insert(0) += 1;
            outcome.reward.terminal += forced_outcome.reward.terminal;
            for (key, value) in forced_outcome.reward.shaping {
                *outcome.reward.shaping.entry(key).or_insert(0.0) += value;
            }
            outcome.observation = forced_outcome.observation;
            outcome.terminated = forced_outcome.terminated;
            outcome.truncated = forced_outcome.truncated;
            outcome.info.ticks_advanced += forced_outcome.info.ticks_advanced;
            outcome.info.reason = forced_outcome.info.reason;
            outcome.state_hash = forced_outcome.state_hash;
        }
        let post_progress_fingerprint = environment.progress_fingerprint();
        let is_cycle =
            !outcome.terminated && progress_tracker.observe(environment.progress_fingerprint());
        finish_cycle_outcome(&mut outcome, is_cycle, &runner_config.reward_config);
        if is_cycle {
            progress_tracker = ProgressTracker::default();
            progress_tracker.observe(environment.progress_fingerprint());
        }
        if !outcome.terminated && !outcome.truncated && decision_count + 1 == max_decisions {
            outcome.truncated = true;
            outcome.info.reason = super::environment::StepReason::MaxDecisions;
        }
        terminated = outcome.terminated;
        truncated = outcome.truncated;
        episode_return += outcome.reward.total();
        if terminated || truncated {
            termination_reason = outcome.info.reason.clone();
        }
        if let Some(steps) = &mut steps {
            steps.push(PolicyStep {
                observation,
                legal_actions,
                action,
                outcome,
                pre_progress_fingerprint,
                post_progress_fingerprint,
            });
        }
        decision_count += 1;
        if terminated || truncated {
            break;
        }
    }

    if !terminated && !truncated && decision_count == max_decisions {
        truncated = true;
        termination_reason = super::environment::StepReason::MaxDecisions;
    }

    Ok(EpisodeResult {
        seed,
        decision_count,
        forced_actions,
        terminated,
        truncated,
        victory: terminated && environment.clear_rate() >= 100.0,
        clear_rate: environment.clear_rate(),
        final_observation: environment.snapshot(),
        final_state_hash: environment.state_hash(),
        metrics: environment.metrics(),
        episode_return,
        termination_reason,
        steps,
    })
}

pub(crate) fn run_episode_with_step_callback<P, F>(
    game_config: Arc<GameConfig>,
    seed: u64,
    runner_config: &PolicyRunnerConfig,
    mut policy: P,
    mut on_step: F,
) -> Result<EpisodeResult>
where
    P: FnMut(&Observation, &[LegalAction]) -> Result<AgentAction>,
    F: FnMut(&Observation, &[LegalAction], &AgentAction, &StepOutcome),
{
    let max_decisions = runner_config.max_decisions_per_episode;
    if max_decisions == 0 {
        bail!("max decisions per episode must be positive");
    }
    runner_config
        .reward_config
        .validate()
        .map_err(anyhow::Error::msg)?;

    let mut environment = match runner_config.max_stage {
        Some(max_stage) => GameEnvironment::new_with_stage_limit(
            game_config,
            seed,
            runner_config.reward_config.clone(),
            max_stage,
        ),
        None => GameEnvironment::new_with_reward_config(
            game_config,
            seed,
            runner_config.reward_config.clone(),
        ),
    };
    environment.set_max_advance_ticks(
        (max_decisions as u64).saturating_mul(super::environment::DEFAULT_MAX_ADVANCE_TICKS),
    );
    let mut terminated = false;
    let mut truncated = false;
    let mut decision_count = 0;
    let mut episode_return = 0.0;
    let mut termination_reason = super::environment::StepReason::Terminal;
    let mut progress_tracker = ProgressTracker::default();
    let mut action_history_guard = ActionHistoryGuard::default();
    let mut forced_actions = ForcedActionStats::default();
    progress_tracker.observe(environment.progress_fingerprint());

    while decision_count < max_decisions {
        if matches!(environment.decision_point(), DecisionPoint::Terminal) {
            terminated = true;
            break;
        }
        let observation = environment.snapshot();
        let pre_progress_fingerprint = environment.progress_fingerprint();
        let canonical_legal_actions = environment.legal_actions();
        if canonical_legal_actions.is_empty() {
            bail!("environment reached a non-terminal state without legal actions at seed {seed}");
        }
        let legal_actions = action_history_guard
            .effective_actions(&pre_progress_fingerprint, &canonical_legal_actions);
        let action = policy(&observation, &legal_actions)?;
        action_history_guard.observe(pre_progress_fingerprint, action.clone());
        let mut outcome = environment
            .step(action.clone())
            .map_err(|error| runner_environment_error(seed, error))?;
        while !outcome.terminated && !outcome.truncated {
            let Some(forced_action) = environment.forced_action() else {
                break;
            };
            let forced_point = environment.decision_point();
            let forced_outcome = environment
                .step(forced_action)
                .map_err(|error| runner_environment_error(seed, error))?;
            forced_actions.total += 1;
            *forced_actions
                .by_decision_point
                .entry(format!("{forced_point:?}"))
                .or_insert(0) += 1;
            outcome.reward.terminal += forced_outcome.reward.terminal;
            for (key, value) in forced_outcome.reward.shaping {
                *outcome.reward.shaping.entry(key).or_insert(0.0) += value;
            }
            outcome.observation = forced_outcome.observation;
            outcome.terminated = forced_outcome.terminated;
            outcome.truncated = forced_outcome.truncated;
            outcome.info.ticks_advanced += forced_outcome.info.ticks_advanced;
            outcome.info.reason = forced_outcome.info.reason;
            outcome.state_hash = forced_outcome.state_hash;
        }
        let is_cycle =
            !outcome.terminated && progress_tracker.observe(environment.progress_fingerprint());
        finish_cycle_outcome(&mut outcome, is_cycle, &runner_config.reward_config);
        if is_cycle {
            progress_tracker = ProgressTracker::default();
            progress_tracker.observe(environment.progress_fingerprint());
        }
        if !outcome.terminated && !outcome.truncated && decision_count + 1 == max_decisions {
            outcome.truncated = true;
            outcome.info.reason = super::environment::StepReason::MaxDecisions;
        }
        terminated = outcome.terminated;
        truncated = outcome.truncated;
        episode_return += outcome.reward.total();
        if terminated || truncated {
            termination_reason = outcome.info.reason.clone();
        }
        on_step(&observation, &legal_actions, &action, &outcome);
        decision_count += 1;
        if terminated || truncated {
            break;
        }
    }
    if !terminated && !truncated && decision_count == max_decisions {
        truncated = true;
        termination_reason = super::environment::StepReason::MaxDecisions;
    }

    Ok(EpisodeResult {
        seed,
        decision_count,
        forced_actions,
        terminated,
        truncated,
        victory: terminated && environment.clear_rate() >= 100.0,
        clear_rate: environment.clear_rate(),
        final_observation: environment.snapshot(),
        final_state_hash: environment.state_hash(),
        metrics: environment.metrics(),
        episode_return,
        termination_reason,
        steps: None,
    })
}

fn runner_environment_error(seed: u64, error: EnvironmentError) -> anyhow::Error {
    anyhow::anyhow!("environment action failed for seed {seed}: {error:?}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;

    fn first_legal_action(
        _observation: &Observation,
        legal_actions: &[LegalAction],
    ) -> Result<AgentAction> {
        legal_actions
            .first()
            .map(|legal| legal.action.clone())
            .ok_or_else(|| anyhow::anyhow!("expected a legal action"))
    }

    #[test]
    fn explicit_seed_batch_preserves_seed_identity_and_order() {
        let config = Arc::new(GameConfig::default_config());
        let seeds = vec![19, 3, 11];
        let result = run_batch::<_, _>(
            config,
            &seeds,
            &PolicyRunnerConfig {
                max_decisions_per_episode: 1,
                record_steps: true,
                max_stage: None,
                reward_config: RewardConfig::default(),
            },
            |_| first_legal_action,
        )
        .expect("explicit seed batch should run");

        assert_eq!(result.seeds, seeds);
        assert_eq!(
            result
                .episodes
                .iter()
                .map(|episode| episode.seed)
                .collect::<Vec<_>>(),
            vec![3, 11, 19]
        );
        assert!(
            result
                .episodes
                .iter()
                .all(|episode| episode.steps.is_some())
        );
        assert!(
            result
                .episodes
                .iter()
                .all(|episode| episode.steps.as_ref().unwrap()[0].outcome.truncated)
        );
    }

    #[test]
    fn batch_results_are_identical_across_thread_counts() {
        let config = Arc::new(GameConfig::default_config());
        let seeds = vec![0, 1, 2, 3];
        let runner_config = PolicyRunnerConfig {
            max_decisions_per_episode: 2,
            record_steps: true,
            ..PolicyRunnerConfig::default()
        };
        let run = |threads| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .expect("thread pool should build")
                .install(|| {
                    run_batch::<_, _>(Arc::clone(&config), &seeds, &runner_config, |_| {
                        first_legal_action
                    })
                })
                .expect("batch should run")
        };
        assert_eq!(run(1), run(4));
    }

    #[test]
    fn zero_decision_limit_is_rejected() {
        let error = run_episode(
            Arc::new(GameConfig::default_config()),
            0,
            &PolicyRunnerConfig {
                max_decisions_per_episode: 0,
                record_steps: false,
                max_stage: None,
                reward_config: RewardConfig::default(),
            },
            first_legal_action,
        )
        .expect_err("zero decision limit should fail");
        assert!(error.to_string().contains("max decisions"));
    }

    #[test]
    fn progress_tracker_detects_periodic_cycle_only() {
        let mut tracker = ProgressTracker::default();
        assert!(!tracker.observe("a".to_string()));
        assert!(!tracker.observe("b".to_string()));
        assert!(!tracker.observe("a".to_string()));
        assert!(tracker.observe("b".to_string()));
    }

    #[test]
    fn action_history_guard_filters_inverse_actions_but_preserves_cancel_fallback() {
        let mut guard = ActionHistoryGuard::default();
        let legal_actions = vec![
            LegalAction {
                id: AgentAction::CancelCardSelection.action_id(),
                action: AgentAction::CancelCardSelection,
            },
            LegalAction {
                id: AgentAction::SelectHandCard { hand_slot_index: 0 }.action_id(),
                action: AgentAction::SelectHandCard { hand_slot_index: 0 },
            },
        ];
        guard.observe("before".to_string(), AgentAction::BeginTowerSelection);
        let filtered = guard.effective_actions("after", &legal_actions);
        assert_eq!(filtered.len(), 1);
        assert_eq!(
            filtered[0].action,
            AgentAction::SelectHandCard { hand_slot_index: 0 }
        );

        guard.observe("before".to_string(), AgentAction::BeginTowerSelection);
        let fallback = guard.effective_actions("after", &[legal_actions[0].clone()]);
        assert_eq!(fallback, vec![legal_actions[0].clone()]);
    }

    #[test]
    fn cycle_event_does_not_truncate_episode() {
        let mut environment = GameEnvironment::new_with_reward_config(
            Arc::new(GameConfig::default_config()),
            0,
            RewardConfig::default(),
        );
        let action = environment
            .legal_actions()
            .into_iter()
            .next()
            .expect("initial environment should have an action")
            .action;
        let mut outcome = environment
            .step(action)
            .expect("initial action should be legal");
        finish_cycle_outcome(&mut outcome, true, &RewardConfig::default());
        assert!(outcome.info.no_progress_cycle);
        assert!(!outcome.truncated);
        assert!(!outcome.terminated);
    }

    #[test]
    fn scripted_expert_selects_all_available_cards() {
        let config = Arc::new(GameConfig::default_config());
        let episode = run_episode(
            config,
            17,
            &PolicyRunnerConfig {
                max_decisions_per_episode: 20,
                record_steps: true,
                ..PolicyRunnerConfig::default()
            },
            scripted_expert_action,
        )
        .expect("expert episode should run");
        let steps = episode.steps.expect("steps should be recorded");
        let selected = steps
            .iter()
            .filter(|step| matches!(step.action, AgentAction::SelectHandCard { .. }))
            .map(|step| step.action.action_id())
            .collect::<Vec<_>>();
        assert!(selected.starts_with(&[
            "select_hand_card:0".to_string(),
            "select_hand_card:1".to_string(),
            "select_hand_card:2".to_string(),
            "select_hand_card:3".to_string(),
            "select_hand_card:4".to_string(),
        ]));
    }

    #[test]
    fn scripted_expert_emits_reroll_branch_for_a_poor_hand() {
        let config = Arc::new(GameConfig::default_config());
        let mut found_reroll = false;
        for seed in 0..64 {
            let episode = run_episode(
                Arc::clone(&config),
                seed,
                &PolicyRunnerConfig {
                    max_decisions_per_episode: 40,
                    record_steps: true,
                    ..PolicyRunnerConfig::default()
                },
                scripted_expert_action,
            )
            .expect("expert episode should run");
            let steps = episode.steps.expect("steps should be recorded");
            if steps
                .iter()
                .any(|step| matches!(step.action, AgentAction::BeginRerollSelection))
            {
                found_reroll = true;
                break;
            }
        }
        assert!(
            found_reroll,
            "expert should expose a reroll training sample"
        );
    }

    #[test]
    fn callback_recorded_rewards_and_episode_return_match() {
        let config = Arc::new(GameConfig::default_config());
        let runner_config = PolicyRunnerConfig {
            max_decisions_per_episode: 4,
            record_steps: true,
            ..PolicyRunnerConfig::default()
        };
        let recorded = run_episode(Arc::clone(&config), 17, &runner_config, first_legal_action)
            .expect("recorded episode should run");
        let callback_rewards = std::cell::RefCell::new(Vec::new());
        let callback = run_episode_with_step_callback(
            config,
            17,
            &runner_config,
            first_legal_action,
            |_observation, _legal_actions, _action, outcome| {
                callback_rewards.borrow_mut().push(outcome.reward.total());
            },
        )
        .expect("callback episode should run");
        let recorded_rewards = recorded
            .steps
            .as_ref()
            .expect("recorded steps should exist")
            .iter()
            .map(|step| step.outcome.reward.total())
            .collect::<Vec<_>>();
        assert_eq!(*callback_rewards.borrow(), recorded_rewards);
        assert_eq!(callback.episode_return, recorded.episode_return);
        assert_eq!(
            callback_rewards.borrow().iter().sum::<f32>(),
            callback.episode_return
        );
    }

    #[test]
    fn scripted_oracle_is_repeatable_and_reaches_placement_and_defense() {
        let config = Arc::new(GameConfig::default_config());
        let first =
            run_scripted_oracle(Arc::clone(&config), 19).expect("scripted oracle should run");
        let second = run_scripted_oracle(config, 19).expect("scripted oracle should repeat");

        assert_eq!(first, second);
        assert!(first.total_towers_placed >= 1);
        assert!(first.final_sim_tick > 0 || first.final_stage > 0 || first.terminated);
        assert!(first.steps.iter().any(|step| {
            step.decision_point == DecisionPoint::TowerPlacement
                && step.selected_action_id.starts_with("place_tower:")
        }));
        assert!(first.steps.iter().any(|step| {
            step.decision_point == DecisionPoint::PreDefenseItem
                || step.decision_point == DecisionPoint::Defense
        }));
        assert!(first.steps.iter().all(|step| {
            step.legal_action_ids
                .iter()
                .filter(|id| **id == step.selected_action_id)
                .count()
                == 1
        }));
    }

    #[test]
    fn scripted_oracle_stage_limit_is_explicit() {
        let config = Arc::new(GameConfig::default_config());
        let limited = run_scripted_oracle_with_stage_limit(Arc::clone(&config), 19, Some(1))
            .expect("limited oracle should run");
        let unrestricted = run_scripted_oracle_with_stage_limit(config, 19, None)
            .expect("unrestricted oracle should run");

        assert_eq!(limited.final_stage, 1);
        assert_eq!(limited.steps[0].selected_action_id, "start_selecting_tower");
        assert_eq!(
            unrestricted.steps[0].selected_action_id,
            "start_selecting_tower"
        );
    }
}
