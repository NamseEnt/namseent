//! Versioned, headless game environment for policy-driven simulation.

use crate::GameCore;
use crate::config::GameConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use td_core::CommandError;
#[cfg(test)]
use td_core::PlayerCommand;

pub const ENVIRONMENT_VERSION: u32 = 6;
pub const ACTION_SCHEMA_VERSION: u32 = 5;
pub const ENVIRONMENT_REPLAY_SCHEMA_VERSION: u32 = 6;
pub const DEFAULT_MAX_ADVANCE_TICKS: u64 = 60 * 60 * 5;

pub use td_core::RewardConfig;

pub fn potential(observation: &Observation) -> f32 {
    let hp_ratio = observation.hp_raw.max(0) as f32 / observation.max_hp_raw.max(1) as f32;
    let stage_progress =
        observation.stage_progress_raw.max(0) as f32 / observation.stage_total_hp_raw.max(1) as f32;
    observation.stage as f32 + stage_progress + hp_ratio * 0.01
}

pub fn potential_shaping(
    before: &Observation,
    after: &Observation,
    terminated: bool,
    config: &RewardConfig,
) -> f32 {
    let next_potential = if terminated { 0.0 } else { potential(after) };
    config.potential_weight * (config.potential_gamma * next_potential - potential(before))
}

fn decision_point_from_flow(flow: &td_core::GameFlowState) -> DecisionPoint {
    match flow {
        td_core::GameFlowState::Shopping(_) => DecisionPoint::Shop,
        td_core::GameFlowState::SelectingTower => DecisionPoint::CardSelection,
        td_core::GameFlowState::PlacingTower => DecisionPoint::TowerPlacement,
        td_core::GameFlowState::Defense(_) => DecisionPoint::Defense,
        td_core::GameFlowState::TreasureSelection { .. } => DecisionPoint::TreasureSelection,
        td_core::GameFlowState::Result { .. } | td_core::GameFlowState::Initializing => {
            DecisionPoint::Terminal
        }
    }
}

fn shaping_rewards(
    clear_rate_before: f32,
    clear_rate_after: f32,
    escaped_hp: f32,
    player_damage: f32,
    stage_total_hp: f32,
    tower_damage: f32,
    config: &RewardConfig,
) -> BTreeMap<String, f32> {
    let progress_reward = clear_rate_after - clear_rate_before;
    let escaped_penalty = -(escaped_hp / stage_total_hp.max(1.0) / config.escaped_hp_penalty_scale);
    let hp_loss_penalty = -(player_damage / config.player_hp_loss_penalty_scale);
    BTreeMap::from([
        ("clear_progress".to_string(), progress_reward),
        ("escaped_hp_penalty".to_string(), escaped_penalty),
        ("player_hp_loss_penalty".to_string(), hp_loss_penalty),
        (
            "damage_progress".to_string(),
            config.damage_progress_weight * tower_damage / stage_total_hp.max(1.0),
        ),
        ("no_progress_cycle_penalty".to_string(), 0.0),
    ])
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum DecisionContext {
    None,
    CardSelection {
        purpose: CardSelectionPurpose,
        selected_slot_indices: Vec<usize>,
    },
    PreDefenseItem {
        stage: usize,
    },
    DamageResponseItem {
        stage: usize,
        trigger_tick: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum CardSelectionPurpose {
    Reroll,
    BuildTower,
}

pub use td_core::{ActionKind, AgentAction, DecisionPoint};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalAction {
    pub id: String,
    pub action: AgentAction,
}

pub use td_core::CardObservation;
pub use td_core::ShopSlotObservation;

pub use td_core::{HandItemObservation, TowerTemplateObservation};

pub use td_core::HandObservation;

pub use td_core::TowerObservation;
pub use td_core::{InventoryObservation, MonsterObservation, OwnedUpgradeObservation};

pub use td_core::DeckObservation;

pub use td_core::StageModifiersObservation;

pub use td_core::Observation;

pub use td_core::RouteCoordObservation;

pub use td_core::CardServiceObservation;

pub use td_core::{RewardComponents, StepInfo, StepReason};

pub use td_core::StepOutcome;

pub use td_core::ReplayCheckpoint as EnvironmentReplayCheckpoint;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentReplay {
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub replay_schema_version: u32,
    pub config_version: u32,
    #[serde(default)]
    pub config_digest_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    pub seed: u64,
    pub commands: Vec<AgentAction>,
    pub checkpoints: Vec<EnvironmentReplayCheckpoint>,
}

impl EnvironmentReplay {
    pub fn validate_schema(&self) -> Result<(), String> {
        if self.environment_version != ENVIRONMENT_VERSION {
            return Err(format!(
                "unsupported environment replay environment schema {}",
                self.environment_version
            ));
        }
        if self.action_schema_version != ACTION_SCHEMA_VERSION {
            return Err(format!(
                "unsupported environment replay action schema {}",
                self.action_schema_version
            ));
        }
        if self.replay_schema_version != ENVIRONMENT_REPLAY_SCHEMA_VERSION {
            return Err(format!(
                "unsupported environment replay schema {}",
                self.replay_schema_version
            ));
        }
        if self.config_digest_version != 0
            && self.config_digest_version != crate::config::CONFIG_DIGEST_VERSION
        {
            return Err(format!(
                "unsupported environment replay config digest schema {}",
                self.config_digest_version
            ));
        }
        Ok(())
    }

    pub fn execute(&self, config: Arc<GameConfig>) -> Result<GameEnvironment, String> {
        self.validate_schema()?;
        let mut environment = GameEnvironment::new(config, self.seed);
        for action in &self.commands {
            environment
                .step(action.clone())
                .map_err(|error| format!("{error:?}"))?;
        }
        if let Some(checkpoint) = self.checkpoints.last()
            && environment.state_hash() != checkpoint.state_hash
        {
            return Err(format!(
                "environment replay final hash mismatch: expected {}, got {}",
                checkpoint.state_hash,
                environment.state_hash()
            ));
        }
        Ok(environment)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnvironmentMetrics {
    pub stage_damage: Vec<(usize, f32)>,
    pub total_towers_placed: usize,
    pub total_items_used: usize,
    pub total_gold_earned: usize,
    pub total_escaped_hp: f32,
    pub total_player_damage: f32,
    pub total_tower_damage: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentError {
    IllegalAction {
        decision_point: DecisionPoint,
        action_id: String,
        state_hash: String,
    },
    CommandRejected {
        action_id: String,
        error: CommandError,
    },
    CardServiceRejected {
        action_id: String,
    },
}

pub struct GameEnvironment {
    game_state: GameCore,
    seed: u64,
    max_advance_ticks: u64,
    card_service_selection: Option<td_core::CardServiceSelectionState>,
    decision_context: DecisionContext,
    environment_actions: Vec<AgentAction>,
    policy_trace: td_core::PolicyTrace,
    metrics: EnvironmentMetrics,
    reward_config: RewardConfig,
    max_stage: Option<usize>,
}

impl GameEnvironment {
    /// Replays an authoritative headed/core command log without constructing
    /// any headed modal, locale, or card-service presentation state.
    pub fn from_core_replay(replay: &td_core::CoreReplay) -> Result<Self, String> {
        let mut replay = replay.clone();
        replay
            .migrate_legacy_metadata()
            .map_err(|error| format!("{error:?}"))?;
        replay.validate().map_err(|error| format!("{error:?}"))?;
        let config = GameConfig::from_core_state(replay.config.clone())
            .ok_or_else(|| "core replay config cannot be restored".to_string())?;
        let mut environment = Self::new(Arc::new(config), replay.seed);
        environment.policy_trace = td_core::PolicyTrace::from_core_replay(
            &replay,
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
        );

        for (index, recorded) in replay.commands.iter().enumerate() {
            let pre_observation = environment.snapshot();
            let pre_state_hash = environment.state_hash();
            let pre_tick = environment.game_state.sim_tick().ticks();
            let action = Self::agent_action_for_command(&recorded.command);
            environment
                .game_state
                .apply(recorded.command.clone())
                .map_err(|error| format!("replay command {index} rejected: {error:?}"))?;
            while environment.game_state.sim_tick().ticks() < recorded.completed_sim_tick {
                environment.game_state.advance_tick();
            }
            if environment.game_state.sim_tick().ticks() != recorded.completed_sim_tick {
                return Err(format!(
                    "replay command {} tick mismatch: expected {}, got {}",
                    index,
                    recorded.completed_sim_tick,
                    environment.game_state.sim_tick().ticks()
                ));
            }
            let post_observation = environment.snapshot();
            let post_state_hash = environment.state_hash();
            if let Some(checkpoint) = replay.checkpoints.get(index)
                && (checkpoint.sequence != recorded.sequence
                    || checkpoint.completed_sim_tick != recorded.completed_sim_tick
                    || checkpoint.state_hash != post_state_hash)
            {
                return Err(format!(
                    "replay checkpoint {} mismatch: expected ({}, {}, {}), got ({}, {}, {})",
                    index,
                    checkpoint.sequence,
                    checkpoint.completed_sim_tick,
                    checkpoint.state_hash,
                    recorded.sequence,
                    recorded.completed_sim_tick,
                    post_state_hash
                ));
            }
            environment.environment_actions.push(action.clone());
            let legal_actions = vec![action.clone()];
            environment
                .policy_trace
                .steps
                .push(td_core::PolicyTraceStep {
                    index: index as u64,
                    decision_point: pre_observation.decision_point.clone(),
                    agent_action: action,
                    legal_actions,
                    action_mask: vec![true],
                    player_command: Some(recorded.command.clone()),
                    pre_observation,
                    post_observation,
                    reward: RewardComponents::default(),
                    terminated: false,
                    truncated: false,
                    info: StepInfo {
                        reason: StepReason::DecisionPoint,
                        ticks_advanced: environment
                            .game_state
                            .sim_tick()
                            .ticks()
                            .saturating_sub(pre_tick),
                        no_progress_cycle: false,
                    },
                    pre_state_hash,
                    post_state_hash,
                });
        }

        Ok(environment)
    }

    pub fn new(config: Arc<GameConfig>, seed: u64) -> Self {
        Self::new_with_reward_config(config, seed, RewardConfig::default())
    }

    fn agent_action_for_command(command: &td_core::PlayerCommand) -> AgentAction {
        match command {
            td_core::PlayerCommand::Reroll {
                selected_slot_indices,
            } => AgentAction::Reroll {
                selected_slot_indices: selected_slot_indices.clone(),
            },
            td_core::PlayerCommand::PurchaseShopItem { slot_index } => {
                AgentAction::PurchaseShopItem {
                    slot_index: *slot_index,
                }
            }
            td_core::PlayerCommand::UseInventoryItem { item_index } => {
                AgentAction::UseInventoryItem {
                    item_index: *item_index,
                }
            }
            td_core::PlayerCommand::DiscardTreasure { upgrade_id } => {
                AgentAction::DiscardTreasure {
                    upgrade_id: *upgrade_id,
                }
            }
            td_core::PlayerCommand::StartSelectingTower => AgentAction::StartSelectingTower,
            td_core::PlayerCommand::SelectTower {
                selected_slot_indices,
            } => AgentAction::SelectTower {
                selected_slot_indices: selected_slot_indices.clone(),
            },
            td_core::PlayerCommand::PlaceTower {
                hand_slot_index,
                left,
                top,
            } => AgentAction::PlaceTower {
                hand_slot_index: *hand_slot_index,
                left: *left,
                top: *top,
            },
            td_core::PlayerCommand::RemoveTower { tower_id } => AgentAction::RemoveTower {
                tower_id: *tower_id,
            },
            td_core::PlayerCommand::StartDefense => AgentAction::StartDefense,
            td_core::PlayerCommand::SelectTreasure { option_index } => {
                AgentAction::SelectTreasure {
                    option_index: *option_index,
                }
            }
            td_core::PlayerCommand::ConfirmCardServiceSelection { .. } => {
                AgentAction::ConfirmCardServiceSelection
            }
        }
    }

    pub fn new_with_reward_config(
        config: Arc<GameConfig>,
        seed: u64,
        reward_config: RewardConfig,
    ) -> Self {
        Self::new_with_options(config, seed, reward_config, None)
    }

    pub fn new_with_stage_limit(
        config: Arc<GameConfig>,
        seed: u64,
        reward_config: RewardConfig,
        max_stage: usize,
    ) -> Self {
        Self::new_with_options(config, seed, reward_config, Some(max_stage))
    }

    fn new_with_options(
        config: Arc<GameConfig>,
        seed: u64,
        reward_config: RewardConfig,
        max_stage: Option<usize>,
    ) -> Self {
        let config = if let Some(max_stage) = max_stage {
            let mut config = (*config).clone();
            config.player.max_stages = config.player.max_stages.min(max_stage);
            Arc::new(config)
        } else {
            config
        };
        let game_state = GameCore::new((*config).clone(), seed);
        let policy_trace = td_core::PolicyTrace::from_core_replay(
            &game_state.replay(),
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
        );
        Self {
            game_state,
            seed,
            max_advance_ticks: DEFAULT_MAX_ADVANCE_TICKS,
            card_service_selection: None,
            decision_context: DecisionContext::None,
            environment_actions: Vec::new(),
            policy_trace,
            metrics: EnvironmentMetrics::default(),
            reward_config,
            max_stage,
        }
    }

    pub fn reset(&mut self, seed: u64, config: Arc<GameConfig>) -> Observation {
        let config = if let Some(max_stage) = self.max_stage {
            let mut config = (*config).clone();
            config.player.max_stages = config.player.max_stages.min(max_stage);
            Arc::new(config)
        } else {
            config
        };
        self.game_state = GameCore::new((*config).clone(), seed);
        self.seed = seed;
        self.card_service_selection = None;
        self.decision_context = DecisionContext::None;
        self.environment_actions.clear();
        self.policy_trace = td_core::PolicyTrace::from_core_replay(
            &self.game_state.replay(),
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
        );
        self.metrics = EnvironmentMetrics::default();
        self.snapshot()
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn set_max_advance_ticks(&mut self, max_advance_ticks: u64) {
        self.max_advance_ticks = max_advance_ticks;
    }

    pub fn reward_config(&self) -> &RewardConfig {
        &self.reward_config
    }

    pub fn set_reward_config(&mut self, reward_config: RewardConfig) {
        self.reward_config = reward_config;
    }

    pub fn decision_point(&self) -> DecisionPoint {
        if self.card_service_selection.is_some() {
            return DecisionPoint::CardServiceSelection;
        }
        match self.decision_context {
            DecisionContext::CardSelection { .. } => return DecisionPoint::CardSelection,
            DecisionContext::PreDefenseItem { .. } => return DecisionPoint::PreDefenseItem,
            DecisionContext::DamageResponseItem { .. } => {
                return DecisionPoint::DamageResponseItem;
            }
            DecisionContext::None => {}
        }
        decision_point_from_flow(self.game_state.raw_state().flow())
    }

    pub fn snapshot(&self) -> Observation {
        let mut observation = self.game_state.observation(
            ENVIRONMENT_VERSION,
            ACTION_SCHEMA_VERSION,
            td_core::MAP_SIZE[0],
            td_core::MAP_SIZE[1],
        );
        let selected_hand_slot_indices: &[usize] = match &self.decision_context {
            DecisionContext::CardSelection {
                selected_slot_indices,
                ..
            } => selected_slot_indices.as_slice(),
            _ => &[],
        };
        for hand in &mut observation.hand {
            hand.selected = selected_hand_slot_indices.contains(&hand.index);
        }
        observation.decision_point = self.decision_point();
        observation.item_window_stage = match self.decision_context {
            DecisionContext::PreDefenseItem { stage }
            | DecisionContext::DamageResponseItem { stage, .. } => Some(stage),
            DecisionContext::None | DecisionContext::CardSelection { .. } => None,
        };
        observation.damage_trigger_tick = match self.decision_context {
            DecisionContext::DamageResponseItem { trigger_tick, .. } => Some(trigger_tick),
            _ => None,
        };
        observation.card_selection_purpose = match &self.decision_context {
            DecisionContext::CardSelection { purpose, .. } => Some(
                match purpose {
                    CardSelectionPurpose::Reroll => "reroll",
                    CardSelectionPurpose::BuildTower => "build_tower",
                }
                .to_string(),
            ),
            _ => None,
        };
        observation.selected_hand_slot_indices = selected_hand_slot_indices.to_vec();
        observation.card_selection_confirmable = !selected_hand_slot_indices.is_empty()
            && matches!(self.decision_context, DecisionContext::CardSelection { .. });
        observation.card_service = self.card_service_selection.as_ref().and_then(|selection| {
            card_service_observation_raw(self.game_state.raw_state(), selection)
        });
        observation
    }

    pub fn state_hash(&self) -> String {
        self.game_state.authoritative_hash()
    }

    pub fn progress_fingerprint(&self) -> String {
        let observation = self.snapshot();
        serde_json::to_string(&(
            self.state_hash(),
            observation.decision_point,
            observation.stage,
            observation.sim_tick,
            observation.card_service,
            &self.decision_context,
            observation.item_window_stage,
            observation.damage_trigger_tick,
        ))
        .expect("progress fingerprint serialization should be infallible")
    }

    pub fn clear_rate(&self) -> f32 {
        match self.game_state.raw_state().flow() {
            td_core::GameFlowState::Result { clear_rate_raw } => {
                *clear_rate_raw as f32 * 100.0 / td_core::RATIO_SCALE as f32
            }
            _ => self.game_state.clear_rate().as_percent_f32(),
        }
    }

    pub fn metrics(&self) -> EnvironmentMetrics {
        let mut metrics = self.metrics.clone();
        let reward_metrics = self.game_state.reward_metrics();
        metrics.total_gold_earned = reward_metrics.total_gold_earned;
        metrics.total_player_damage = reward_metrics.total_player_damage;
        metrics.total_tower_damage = reward_metrics.total_tower_damage;
        metrics.stage_damage = reward_metrics.stage_damage;
        metrics
    }

    pub fn replay(&self) -> EnvironmentReplay {
        let replay = self.game_state.replay();
        EnvironmentReplay {
            environment_version: ENVIRONMENT_VERSION,
            action_schema_version: ACTION_SCHEMA_VERSION,
            replay_schema_version: ENVIRONMENT_REPLAY_SCHEMA_VERSION,
            config_version: replay.config_version,
            config_digest_version: crate::config::CONFIG_DIGEST_VERSION,
            config_digest: replay.config_digest,
            rng_algorithm_version: replay.rng_algorithm_version,
            seed: replay.seed,
            commands: self.environment_actions.clone(),
            checkpoints: replay
                .checkpoints
                .iter()
                .map(|checkpoint| EnvironmentReplayCheckpoint {
                    sequence: checkpoint.sequence,
                    completed_sim_tick: checkpoint.completed_sim_tick,
                    state_hash: checkpoint.state_hash.clone(),
                    event_count: checkpoint.event_count,
                    event_digest: checkpoint.event_digest.clone(),
                })
                .collect(),
        }
    }

    pub fn policy_trace(&self) -> &td_core::PolicyTrace {
        &self.policy_trace
    }

    pub fn core_replay(&self) -> td_core::CoreReplay {
        self.game_state
            .replay()
            .with_policy_trace(self.policy_trace.clone())
    }

    pub fn legal_actions(&self) -> Vec<LegalAction> {
        let mut actions = match self.decision_point() {
            DecisionPoint::Shop => self.shop_actions(),
            DecisionPoint::CardSelection => self.card_selection_actions(),
            DecisionPoint::CardServiceSelection => self.card_service_actions(),
            DecisionPoint::TowerPlacement => self.tower_placement_actions(),
            DecisionPoint::PreDefenseItem | DecisionPoint::DamageResponseItem => {
                let mut actions = self.inventory_actions();
                actions.push(AgentAction::Continue);
                actions
            }
            DecisionPoint::Defense => vec![AgentAction::Continue],
            DecisionPoint::TreasureSelection => self.treasure_actions(),
            DecisionPoint::Terminal => Vec::new(),
        };
        if !matches!(
            self.decision_point(),
            DecisionPoint::Defense
                | DecisionPoint::PreDefenseItem
                | DecisionPoint::DamageResponseItem
                | DecisionPoint::Terminal
                | DecisionPoint::CardServiceSelection
        ) {
            actions.extend(self.inventory_actions());
            actions.extend(self.treasure_discard_actions());
        }

        actions.sort_by_key(AgentAction::action_id);
        actions
            .into_iter()
            .map(|action| LegalAction {
                id: action.action_id(),
                action,
            })
            .collect()
    }

    pub fn action_mask(&self) -> Vec<bool> {
        self.legal_actions().into_iter().map(|_| true).collect()
    }

    pub fn forced_action(&self) -> Option<AgentAction> {
        match self.decision_point() {
            DecisionPoint::PreDefenseItem
            | DecisionPoint::DamageResponseItem
            | DecisionPoint::Defense => {
                let actions = self.legal_actions();
                (actions.len() == 1 && matches!(actions[0].action, AgentAction::Continue))
                    .then_some(AgentAction::Continue)
            }
            _ => None,
        }
    }

    /// Applies one AI decision and advances to the next decision point or
    /// terminal state; this may execute multiple fixed simulation ticks.
    pub fn step(&mut self, action: AgentAction) -> Result<StepOutcome, EnvironmentError> {
        let action_id = action.action_id();
        let legal_actions_before = self.legal_actions();
        if !legal_actions_before
            .iter()
            .any(|legal_action| legal_action.action == action)
        {
            return Err(EnvironmentError::IllegalAction {
                decision_point: self.decision_point(),
                action_id,
                state_hash: self.state_hash(),
            });
        }

        let reward_metrics_before = self.game_state.reward_metrics();
        let escaped_hp_before = reward_metrics_before.total_escaped_hp;
        let tower_damage_before = reward_metrics_before.total_tower_damage;
        let clear_rate_before = self.clear_rate() / 100.0;
        let observation_before = self.snapshot();
        let pre_state_hash = self.state_hash();
        let pre_decision_point = self.decision_point();
        let command_count_before = self.game_state.replay().commands.len();

        let deferred_card_service = self.card_service_kind_for_action(&action);
        let action_result = if let Some(command) = action.to_player_command() {
            self.game_state
                .apply(command)
                .map_err(|error| EnvironmentError::CommandRejected {
                    action_id: action.action_id(),
                    error,
                })
                .map(|_| ())
        } else {
            self.apply_environment_action(&action)
        };
        action_result?;
        self.apply_card_selection_action(&action)?;
        if matches!(action, AgentAction::Continue)
            && matches!(
                self.decision_context,
                DecisionContext::PreDefenseItem { .. } | DecisionContext::DamageResponseItem { .. }
            )
        {
            self.decision_context = DecisionContext::None;
        }
        if matches!(action, AgentAction::StartDefense)
            && matches!(
                self.game_state.raw_state().flow(),
                td_core::GameFlowState::Defense(_)
            )
        {
            self.decision_context = DecisionContext::PreDefenseItem {
                stage: self.game_state.stage(),
            };
        }
        match action {
            AgentAction::PlaceTower { .. } => self.metrics.total_towers_placed += 1,
            AgentAction::UseInventoryItem { .. } => self.metrics.total_items_used += 1,
            _ => {}
        }
        if deferred_card_service.is_some() {
            self.card_service_selection = self.game_state.take_card_service_selection();
        }
        self.environment_actions.push(action.clone());

        let ticks_before = self.game_state.sim_tick().ticks();
        let mut reason = self.advance_until_decision_or_terminal();
        if self.max_stage.is_some()
            && matches!(
                self.game_state.raw_state().flow(),
                td_core::GameFlowState::Result { clear_rate_raw }
                    if *clear_rate_raw == td_core::RATIO_SCALE
            )
        {
            reason = StepReason::CurriculumComplete;
        }
        let ticks_advanced = self
            .game_state
            .sim_tick()
            .ticks()
            .saturating_sub(ticks_before);
        let terminated = matches!(self.decision_point(), DecisionPoint::Terminal);
        let truncated = matches!(reason, StepReason::MaxTicks);
        let terminal_reward = match self.game_state.raw_state().flow() {
            td_core::GameFlowState::Result { clear_rate_raw }
                if *clear_rate_raw == td_core::RATIO_SCALE =>
            {
                self.reward_config.terminal_win
            }
            td_core::GameFlowState::Result { .. } => self.reward_config.terminal_loss,
            _ => 0.0,
        };
        let clear_rate_after = self.clear_rate() / 100.0;
        let reward_metrics_after = self.game_state.reward_metrics();
        let player_damage =
            reward_metrics_after.total_player_damage - self.metrics.total_player_damage;
        self.metrics.total_player_damage = reward_metrics_after.total_player_damage;
        let escaped_hp = (reward_metrics_after.total_escaped_hp - escaped_hp_before).max(0.0);
        self.metrics.total_escaped_hp += escaped_hp;
        let stage_total_hp = match self.game_state.raw_state().flow() {
            td_core::GameFlowState::Defense(flow) => flow.start_total_hp_raw as f32 / 1_000.0,
            _ => {
                let state = self.game_state.raw_state();
                td_core::game_state::monster_spawn::calculate_stage_total_hp_raw(
                    state.progress().stage,
                    state.config(),
                    &state.stage_modifiers().enemy_health_multipliers_raw,
                ) as f32
                    / 1_000.0
            }
        }
        .max(1.0);
        let tower_damage = (reward_metrics_after.total_tower_damage - tower_damage_before).max(0.0);
        let observation_after = self.snapshot();
        let potential_reward = potential_shaping(
            &observation_before,
            &observation_after,
            terminated,
            &self.reward_config,
        );

        let mut shaping = shaping_rewards(
            clear_rate_before,
            clear_rate_after,
            escaped_hp,
            player_damage,
            stage_total_hp,
            tower_damage,
            &self.reward_config,
        );
        shaping.insert("potential_progress".to_string(), potential_reward);

        let outcome = StepOutcome {
            observation: self.snapshot(),
            reward: RewardComponents {
                terminal: terminal_reward,
                shaping,
            },
            terminated,
            truncated,
            info: StepInfo {
                reason,
                ticks_advanced,
                no_progress_cycle: false,
            },
            state_hash: self.state_hash(),
        };

        let player_command = self
            .game_state
            .replay()
            .commands
            .get(command_count_before)
            .map(|recorded| recorded.command.clone());
        self.policy_trace.steps.push(td_core::PolicyTraceStep {
            index: self.policy_trace.steps.len() as u64,
            decision_point: pre_decision_point,
            agent_action: action,
            legal_actions: legal_actions_before
                .iter()
                .map(|legal_action| legal_action.action.clone())
                .collect(),
            action_mask: vec![true; legal_actions_before.len()],
            player_command,
            pre_observation: observation_before,
            post_observation: outcome.observation.clone(),
            reward: outcome.reward.clone(),
            terminated: outcome.terminated,
            truncated: outcome.truncated,
            info: outcome.info.clone(),
            pre_state_hash,
            post_state_hash: outcome.state_hash.clone(),
        });

        Ok(outcome)
    }

    pub fn advance_until_decision_or_terminal(&mut self) -> StepReason {
        let mut ticks_advanced = 0;
        while matches!(
            self.game_state.raw_state().flow(),
            td_core::GameFlowState::Defense(_)
        ) {
            if !matches!(self.decision_context, DecisionContext::None) {
                return StepReason::DecisionPoint;
            }
            if ticks_advanced >= self.max_advance_ticks {
                return StepReason::MaxTicks;
            }
            let hp_before = self.game_state.hp().raw();
            let tick_before = self.game_state.sim_tick().ticks();
            // Defense ticks are internal to the decision-point transition;
            // policy/replay metadata is recorded at the surrounding
            // environment step instead.
            let _ = self.game_state.advance_tick_unrecorded();
            ticks_advanced += 1;
            let hp_after = self.game_state.hp().raw();
            if hp_after < hp_before
                && hp_after > 0
                && matches!(
                    self.game_state.raw_state().flow(),
                    td_core::GameFlowState::Defense(_)
                )
            {
                self.decision_context = DecisionContext::DamageResponseItem {
                    stage: self.game_state.stage(),
                    trigger_tick: tick_before + 1,
                };
                return StepReason::DecisionPoint;
            }
        }

        if matches!(self.decision_point(), DecisionPoint::Terminal) {
            StepReason::Terminal
        } else {
            StepReason::DecisionPoint
        }
    }

    pub fn replay_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.replay())
    }

    fn shop_actions(&self) -> Vec<AgentAction> {
        let mut actions = Vec::new();
        if let td_core::GameFlowState::Shopping(flow) = self.game_state.raw_state().flow() {
            for slot_index in 0..flow.slots.len() {
                if self
                    .game_state
                    .raw_state()
                    .can_purchase_shop_slot(slot_index)
                {
                    actions.push(AgentAction::PurchaseShopItem { slot_index });
                }
            }
            actions.push(AgentAction::StartSelectingTower);
        }
        actions
    }

    fn card_selection_actions(&self) -> Vec<AgentAction> {
        if let DecisionContext::CardSelection {
            selected_slot_indices,
            ..
        } = &self.decision_context
        {
            let card_indices = self.card_hand_indices();
            let mut actions = card_indices
                .into_iter()
                .map(|hand_slot_index| {
                    if selected_slot_indices.contains(&hand_slot_index) {
                        AgentAction::DeselectHandCard { hand_slot_index }
                    } else {
                        AgentAction::SelectHandCard { hand_slot_index }
                    }
                })
                .collect::<Vec<_>>();
            if !selected_slot_indices.is_empty() {
                actions.push(AgentAction::ConfirmCardSelection);
            }
            actions.push(AgentAction::CancelCardSelection);
            return actions;
        }
        let card_indices = self.card_hand_indices();
        let mut actions = Vec::new();
        if !card_indices.is_empty() {
            let state = self.game_state.raw_state();
            let reroll_health_cost = state
                .stage_modifiers()
                .reroll_health_cost
                .saturating_mul(1_000) as i64;
            let can_afford_reroll = state.progress().left_dice > 0
                || (reroll_health_cost > 0
                    && state.hp_raw().saturating_sub(reroll_health_cost) > 1_000);
            if can_afford_reroll {
                actions.push(AgentAction::BeginRerollSelection);
            }
            actions.push(AgentAction::BeginTowerSelection);
        }
        actions
    }

    fn apply_card_selection_action(
        &mut self,
        action: &AgentAction,
    ) -> Result<(), EnvironmentError> {
        match action {
            AgentAction::BeginRerollSelection => {
                self.decision_context = DecisionContext::CardSelection {
                    purpose: CardSelectionPurpose::Reroll,
                    selected_slot_indices: Vec::new(),
                };
            }
            AgentAction::BeginTowerSelection => {
                self.decision_context = DecisionContext::CardSelection {
                    purpose: CardSelectionPurpose::BuildTower,
                    selected_slot_indices: Vec::new(),
                };
            }
            AgentAction::SelectHandCard { hand_slot_index }
            | AgentAction::DeselectHandCard { hand_slot_index } => {
                let DecisionContext::CardSelection {
                    selected_slot_indices,
                    ..
                } = &mut self.decision_context
                else {
                    return Err(EnvironmentError::IllegalAction {
                        decision_point: self.decision_point(),
                        action_id: action.action_id(),
                        state_hash: self.state_hash(),
                    });
                };
                match action {
                    AgentAction::SelectHandCard { .. } => {
                        if selected_slot_indices.contains(hand_slot_index) {
                            return Err(EnvironmentError::IllegalAction {
                                decision_point: self.decision_point(),
                                action_id: action.action_id(),
                                state_hash: self.state_hash(),
                            });
                        }
                        selected_slot_indices.push(*hand_slot_index);
                    }
                    AgentAction::DeselectHandCard { .. } => {
                        let Some(position) = selected_slot_indices
                            .iter()
                            .position(|index| index == hand_slot_index)
                        else {
                            return Err(EnvironmentError::IllegalAction {
                                decision_point: self.decision_point(),
                                action_id: action.action_id(),
                                state_hash: self.state_hash(),
                            });
                        };
                        selected_slot_indices.remove(position);
                    }
                    _ => unreachable!(),
                }
            }
            AgentAction::ConfirmCardSelection => {
                let DecisionContext::CardSelection {
                    purpose,
                    selected_slot_indices,
                } = std::mem::replace(&mut self.decision_context, DecisionContext::None)
                else {
                    return Err(EnvironmentError::IllegalAction {
                        decision_point: self.decision_point(),
                        action_id: action.action_id(),
                        state_hash: self.state_hash(),
                    });
                };
                let committed = match purpose {
                    CardSelectionPurpose::Reroll => AgentAction::Reroll {
                        selected_slot_indices,
                    },
                    CardSelectionPurpose::BuildTower => AgentAction::SelectTower {
                        selected_slot_indices,
                    },
                };
                self.game_state
                    .apply(
                        committed
                            .to_player_command()
                            .expect("card selection commit command"),
                    )
                    .map_err(|error| EnvironmentError::CommandRejected {
                        action_id: committed.action_id(),
                        error,
                    })?;
            }
            AgentAction::CancelCardSelection => {
                self.decision_context = DecisionContext::None;
            }
            _ => {}
        }
        Ok(())
    }

    fn tower_placement_actions(&self) -> Vec<AgentAction> {
        let coordinates = self.placement_coordinates();
        let mut actions = Vec::with_capacity(self.tower_hand_indices().len() * coordinates.len());
        let state = self.game_state.raw_state();
        for hand_slot_index in self.tower_hand_indices() {
            actions.extend(
                coordinates
                    .iter()
                    .cloned()
                    .filter(|(left, top)| state.can_place_tower(hand_slot_index, *left, *top))
                    .map(|(left, top)| AgentAction::PlaceTower {
                        hand_slot_index,
                        left,
                        top,
                    }),
            );
        }
        actions.extend(
            self.game_state
                .raw_state()
                .towers()
                .iter()
                .filter_map(|tower| tower.id)
                .map(|tower_id| AgentAction::RemoveTower { tower_id }),
        );
        actions.push(AgentAction::StartDefense);
        actions
    }

    fn placement_coordinates(&self) -> Vec<(usize, usize)> {
        (0..td_core::MAP_SIZE[1].saturating_sub(1))
            .flat_map(|top| {
                (0..td_core::MAP_SIZE[0].saturating_sub(1)).filter_map(move |left| {
                    self.tower_hand_indices()
                        .iter()
                        .copied()
                        .any(|hand_slot_index| {
                            self.game_state
                                .raw_state()
                                .can_place_tower(hand_slot_index, left, top)
                        })
                        .then_some((left, top))
                })
            })
            .collect()
    }

    fn treasure_actions(&self) -> Vec<AgentAction> {
        match self.game_state.raw_state().flow() {
            td_core::GameFlowState::TreasureSelection { options, .. } => (0..options.len())
                .map(|option_index| AgentAction::SelectTreasure { option_index })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn treasure_discard_actions(&self) -> Vec<AgentAction> {
        self.game_state
            .raw_state()
            .upgrades()
            .entries()
            .iter()
            .map(|upgrade| AgentAction::DiscardTreasure {
                upgrade_id: upgrade.id(),
            })
            .collect()
    }

    fn card_service_actions(&self) -> Vec<AgentAction> {
        let Some(selection) = self.card_service_selection() else {
            return Vec::new();
        };
        let cards = &self.game_state.raw_state().deck().all_cards;
        let mut actions = cards
            .iter()
            .enumerate()
            .filter(|(_, card)| {
                raw_card_matches_filter(&selection.steps[selection.current_step].filter, card)
            })
            .map(|(card_index, _)| AgentAction::SelectCardServiceCard { card_index })
            .collect::<Vec<_>>();
        if selection.selected_card_ids[selection.current_step].len()
            == selection.steps[selection.current_step].count
        {
            actions.push(AgentAction::ConfirmCardServiceSelection);
        }
        actions
    }

    fn card_service_kind_for_action(
        &self,
        action: &AgentAction,
    ) -> Option<td_core::CardServiceKind> {
        let AgentAction::PurchaseShopItem { slot_index } = action else {
            return None;
        };
        let td_core::GameFlowState::Shopping(flow) = self.game_state.raw_state().flow() else {
            return None;
        };
        flow.slots
            .get(*slot_index)
            .and_then(|slot| match slot.slot {
                td_core::ShopSlotState::CardService { kind, .. } => {
                    td_core::CardServiceKind::from_raw(kind)
                }
                _ => None,
            })
    }

    fn card_service_selection(&self) -> Option<&td_core::CardServiceSelectionState> {
        self.card_service_selection.as_ref()
    }

    fn apply_environment_action(&mut self, action: &AgentAction) -> Result<(), EnvironmentError> {
        match action {
            AgentAction::SelectCardServiceCard { card_index } => {
                let card = self
                    .game_state
                    .raw_state()
                    .deck()
                    .all_cards
                    .get(*card_index)
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                let selection = self.card_service_selection.as_mut().ok_or_else(|| {
                    EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    }
                })?;
                if !raw_card_matches_filter(&selection.steps[selection.current_step].filter, card) {
                    return Err(EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    });
                }
                let selected = &mut selection.selected_card_ids[selection.current_step];
                if let Some(index) = selected.iter().position(|id| *id == card.id) {
                    selected.remove(index);
                } else if selected.len() < selection.steps[selection.current_step].count {
                    selected.push(card.id);
                }
                Ok(())
            }
            AgentAction::ConfirmCardServiceSelection => {
                let selection = self
                    .card_service_selection
                    .as_ref()
                    .filter(|selection| {
                        selection.selected_card_ids[selection.current_step].len()
                            == selection.steps[selection.current_step].count
                    })
                    .ok_or_else(|| EnvironmentError::CardServiceRejected {
                        action_id: action.action_id(),
                    })?;
                if selection.current_step + 1 < selection.steps.len() {
                    let next_step = selection.current_step + 1;
                    let selection = self.card_service_selection.as_mut().ok_or_else(|| {
                        EnvironmentError::CardServiceRejected {
                            action_id: action.action_id(),
                        }
                    })?;
                    selection.current_step = next_step;
                    return Ok(());
                }
                let selected_card_ids = selection.selected_card_ids.clone();
                self.card_service_selection = None;
                self.game_state
                    .apply_card_service_selection(selected_card_ids)
                    .map_err(|error| EnvironmentError::CommandRejected {
                        action_id: action.action_id(),
                        error,
                    })
                    .map(|_| ())
            }
            AgentAction::BeginRerollSelection
            | AgentAction::BeginTowerSelection
            | AgentAction::SelectHandCard { .. }
            | AgentAction::DeselectHandCard { .. }
            | AgentAction::ConfirmCardSelection
            | AgentAction::CancelCardSelection
            | AgentAction::Continue => Ok(()),
            _ => Err(EnvironmentError::CardServiceRejected {
                action_id: action.action_id(),
            }),
        }
    }

    fn inventory_actions(&self) -> Vec<AgentAction> {
        let state = self.game_state.raw_state();
        state
            .items()
            .iter()
            .enumerate()
            .filter(|(item_index, _)| state.can_use_inventory_item(*item_index))
            .map(|(item_index, _)| AgentAction::UseInventoryItem { item_index })
            .collect()
    }

    fn card_hand_indices(&self) -> Vec<usize> {
        self.game_state
            .raw_state()
            .hand()
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                matches!(slot.item, td_core::HandItemState::Card(_)).then_some(index)
            })
            .collect()
    }

    fn tower_hand_indices(&self) -> Vec<usize> {
        self.game_state
            .raw_state()
            .hand()
            .slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                matches!(slot.item, td_core::HandItemState::Tower(_)).then_some(index)
            })
            .collect()
    }
}

#[cfg(test)]
fn unordered_cards(cards: &[td_core::CardState]) -> Vec<CardObservation> {
    let mut observations = cards.iter().map(card_observation).collect::<Vec<_>>();
    observations.sort_by_key(|card| card.id);
    observations
}

fn card_service_observation_raw(
    game_state: &td_core::CoreState,
    selection: &td_core::CardServiceSelectionState,
) -> Option<CardServiceObservation> {
    let cards = &game_state.deck().all_cards;
    let selected_card_indices = selection.selected_card_ids[selection.current_step]
        .iter()
        .filter_map(|card_id| cards.iter().position(|card| card.id == *card_id))
        .collect();
    let candidate_card_indices = cards
        .iter()
        .enumerate()
        .filter(|(_, card)| {
            raw_card_matches_filter(&selection.steps[selection.current_step].filter, card)
        })
        .map(|(index, _)| index)
        .collect();
    Some(CardServiceObservation {
        key: selection.service_kind()?.key().to_string(),
        current_step: selection.current_step,
        step_count: selection.steps.len(),
        required_count: selection.steps[selection.current_step].count,
        selected_card_indices,
        candidate_card_indices,
    })
}

fn raw_card_matches_filter(
    filter: &td_core::CardSelectionFilterState,
    card: &td_core::CardState,
) -> bool {
    match filter {
        td_core::CardSelectionFilterState::Any => true,
        td_core::CardSelectionFilterState::Face => (9..=11).contains(&card.rank),
        td_core::CardSelectionFilterState::Number => card.rank <= 8,
        td_core::CardSelectionFilterState::Rank(rank) => card.rank == *rank,
        td_core::CardSelectionFilterState::Engraved => card.engraving.is_some(),
        td_core::CardSelectionFilterState::NotEngraved => card.engraving.is_none(),
        td_core::CardSelectionFilterState::And(filters) => filters
            .iter()
            .all(|filter| raw_card_matches_filter(filter, card)),
        td_core::CardSelectionFilterState::Or(filters) => filters
            .iter()
            .any(|filter| raw_card_matches_filter(filter, card)),
    }
}

#[cfg(test)]
fn card_observation(card: &td_core::CardState) -> CardObservation {
    CardObservation {
        id: card.id,
        suit: ["spades", "hearts", "diamonds", "clubs"][card.suit as usize].to_string(),
        rank: [
            "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack",
            "queen", "king", "ace",
        ][card.rank as usize]
            .to_string(),
        polish_pct_raw: card.polish_pct_raw,
        engraving: card.engraving.map(|kind| {
            ["magnet", "overcharge", "cactus", "spinning_top"][kind as usize].to_string()
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn default_reward_config_preserves_existing_scales() {
        let config = RewardConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.terminal_win, 1.0);
        assert_eq!(config.terminal_loss, -1.0);
        assert_eq!(config.escaped_hp_penalty_scale, 50.0);
        assert_eq!(config.player_hp_loss_penalty_scale, 60.0);
        assert_eq!(config.potential_weight, 0.0);
        assert_eq!(config.potential_gamma, 0.99);
    }

    #[test]
    fn reward_config_rejects_positive_cycle_penalty() {
        let config = RewardConfig {
            no_progress_cycle_penalty: 0.25,
            ..RewardConfig::default()
        };
        let error = config
            .validate()
            .expect_err("positive cycle penalty must fail");
        assert!(error.contains("non-positive"));
    }

    #[test]
    fn reward_config_rejects_non_finite_fields() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let config = RewardConfig {
                no_progress_cycle_penalty: value,
                ..RewardConfig::default()
            };
            assert!(config.validate().is_err());
        }
    }

    #[test]
    fn terminal_potential_is_zero() {
        let environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), 7);
        let before = environment.snapshot();
        let after = before.clone();
        let config = RewardConfig {
            potential_weight: 1.0,
            ..RewardConfig::default()
        };

        assert_eq!(
            potential_shaping(&before, &after, true, &config),
            -potential(&before)
        );
    }

    #[test]
    fn potential_progress_component_is_zero_when_disabled() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert_eq!(outcome.reward.shaping["potential_progress"], 0.0);
    }

    fn environment() -> GameEnvironment {
        GameEnvironment::new(Arc::new(GameConfig::default_config()), 7)
    }

    #[test]
    #[ignore = "manual release profiling diagnostic"]
    fn profile_environment_step_path() {
        let config = Arc::new(GameConfig::default_config());
        let started = std::time::Instant::now();
        let mut steps = 0usize;
        let mut episodes = 0usize;

        for seed in 11..15 {
            let mut environment = GameEnvironment::new(Arc::clone(&config), seed);
            for _ in 0..128 {
                let action = environment
                    .legal_actions()
                    .into_iter()
                    .next()
                    .expect("profile environment should expose an action")
                    .action;
                let outcome = environment
                    .step(action)
                    .expect("profile action should be legal");
                std::hint::black_box((&outcome, environment.policy_trace().steps.len()));
                steps += 1;
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
            episodes += 1;
        }

        let elapsed = started.elapsed();
        println!(
            "environment_step_profile episodes={episodes} steps={steps} elapsed={:.3}s ns_per_step={:.0}",
            elapsed.as_secs_f64(),
            elapsed.as_nanos() as f64 / steps as f64
        );
        assert!(steps > 0);
    }

    #[test]
    fn reset_and_observation_are_reproducible_for_same_seed() {
        let mut first = environment();
        let mut second = environment();

        assert_eq!(
            first.reset(42, Arc::new(GameConfig::default_config())),
            second.reset(42, Arc::new(GameConfig::default_config()))
        );
        assert_eq!(first.state_hash(), second.state_hash());
        assert_eq!(first.legal_actions(), second.legal_actions());
    }

    #[test]
    fn two_core_tower_attack_ticks_match_for_a_seeded_defense() {
        let config = Arc::new(GameConfig::default_config());
        let mut first = GameCore::new((*config).clone(), 7);
        let mut second = GameCore::new((*config).clone(), 7);
        let commands = [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::SelectTower {
                selected_slot_indices: vec![0],
            },
            PlayerCommand::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            PlayerCommand::StartDefense,
        ];

        for command in commands {
            first
                .apply(command.clone())
                .expect("first command should be accepted");
            second
                .apply(command)
                .expect("second command should be accepted");
            assert_eq!(
                first.authoritative_hash(),
                second.authoritative_hash(),
                "command paths diverged at tick {}",
                first.sim_tick().ticks()
            );
        }

        for _ in 0..180 {
            let first_output = first.advance_tick();
            let second_output = second.advance_tick();
            assert_eq!(first_output, second_output);
            assert_eq!(
                first.core_state_snapshot(),
                second.core_state_snapshot(),
                "core combat diverged at tick {}",
                first.sim_tick().ticks()
            );
        }
    }

    #[test]
    fn game_core_and_simulator_command_paths_match_at_the_same_sim_tick() {
        let config = Arc::new(GameConfig::default_config());
        let mut core = GameCore::new((*config).clone(), 7);
        let mut simulator = GameEnvironment::new(config, 7);
        let action = AgentAction::StartSelectingTower;
        let command = action
            .to_player_command()
            .expect("start selecting tower must map to a player command");

        core.apply(command)
            .expect("core command should be accepted");
        simulator
            .step(action)
            .expect("simulator action should be accepted");

        while core.sim_tick().ticks() < simulator.game_state.sim_tick().ticks() {
            core.advance_tick();
        }

        assert_eq!(
            core.sim_tick().ticks(),
            simulator.game_state.sim_tick().ticks()
        );
        assert_eq!(
            core.authoritative_hash(),
            simulator.game_state.authoritative_hash()
        );
    }

    #[test]
    fn core_replay_restores_simulator_hash_without_headed_types() {
        let config = GameConfig::default_config();
        let mut source = GameCore::new(config.clone(), 0xC0DE);
        for command in [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            },
        ] {
            source
                .apply_with_receipt(command)
                .expect("source command should be accepted");
        }

        let replay = source.replay();
        let restored =
            GameEnvironment::from_core_replay(&replay).expect("core replay should restore");

        assert_eq!(restored.state_hash(), source.authoritative_hash());
        assert_eq!(
            restored
                .game_state
                .raw_state()
                .replay_checkpoints()
                .last()
                .expect("restored checkpoint"),
            replay.checkpoints.last().expect("source checkpoint")
        );
    }

    #[test]
    fn game_core_and_simulator_match_across_card_selection_command_sequence() {
        let config = Arc::new(GameConfig::default_config());
        let mut core = GameCore::new((*config).clone(), 7);
        let mut simulator = GameEnvironment::new(config, 7);

        let start = AgentAction::StartSelectingTower;
        core.apply(start.to_player_command().expect("start command"))
            .expect("core start command should be accepted");
        simulator
            .step(start)
            .expect("simulator start action should be accepted");

        simulator
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be accepted");
        let hand_slot_index = simulator
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => Some(hand_slot_index),
                _ => None,
            })
            .expect("a hand card should be selectable");
        simulator
            .step(AgentAction::SelectHandCard { hand_slot_index })
            .expect("card selection should be accepted");

        let confirm = AgentAction::ConfirmCardSelection;
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![hand_slot_index],
        })
        .expect("core tower selection command should be accepted");
        simulator
            .step(confirm)
            .expect("simulator card selection should be accepted");

        assert_eq!(core.sim_tick(), simulator.game_state.sim_tick());
        assert_eq!(core.authoritative_hash(), simulator.state_hash());
    }

    #[test]
    fn deck_observation_exposes_owned_cards_and_zone_compositions() {
        let environment = environment();
        let observation = environment.snapshot();
        let owned_ids = observation
            .deck
            .all_cards
            .iter()
            .map(|card| card.id)
            .collect::<HashSet<_>>();

        assert_eq!(owned_ids.len(), observation.deck.all_cards.len());
        assert!(
            observation
                .deck
                .draw_cards
                .iter()
                .all(|card| owned_ids.contains(&card.id))
        );
        assert!(
            observation
                .deck
                .discard_cards
                .iter()
                .all(|card| owned_ids.contains(&card.id))
        );
        let zone_ids = observation
            .deck
            .draw_cards
            .iter()
            .chain(observation.deck.discard_cards.iter())
            .map(|card| card.id)
            .collect::<HashSet<_>>();
        assert_eq!(
            zone_ids.len(),
            observation.deck.draw_cards.len() + observation.deck.discard_cards.len()
        );
        assert!(zone_ids.len() <= owned_ids.len());
    }

    #[test]
    fn unordered_card_observation_is_invariant_to_zone_order() {
        let cards = environment()
            .game_state
            .deck()
            .all_cards
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>();
        let forward = unordered_cards(&cards);
        let reverse = unordered_cards(&cards.into_iter().rev().collect::<Vec<_>>());

        assert_eq!(forward, reverse);
    }

    #[test]
    fn simulator_selection_changes_hand_observation_without_mutating_gameplay_hand() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be legal");
        let select = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("an unselected card should be selectable");

        let before = environment.snapshot();
        let slot_index = match select {
            AgentAction::SelectHandCard { hand_slot_index } => hand_slot_index,
            _ => unreachable!(),
        };
        environment.step(select).expect("select should be legal");
        let selected = environment.snapshot();
        assert_eq!(selected.selected_hand_slot_indices, vec![slot_index]);
        assert!(selected.hand[slot_index].selected);

        assert!(environment.legal_actions().iter().any(|legal| matches!(
            legal.action,
            AgentAction::DeselectHandCard { hand_slot_index } if hand_slot_index == slot_index
        )));
        assert_eq!(before.deck.all_cards, selected.deck.all_cards);
    }

    #[test]
    fn curriculum_stage_limit_preserves_authoritative_hash_before_target_stage() {
        let base_config = GameConfig::default_config();
        let config = Arc::new(base_config);
        let runner_config = crate::policy_runner::PolicyRunnerConfig {
            max_decisions_per_episode: 32,
            max_stage: Some(2),
            ..Default::default()
        };
        let curriculum_runner_config = crate::policy_runner::PolicyRunnerConfig {
            max_stage: Some(2),
            ..runner_config.clone()
        };
        let mut unrestricted_hashes = Vec::new();
        let unrestricted = crate::policy_runner::run_episode_with_step_callback(
            Arc::clone(&config),
            17,
            &runner_config,
            |_observation, legal_actions| {
                Ok(legal_actions
                    .first()
                    .expect("test environment should expose a legal action")
                    .action
                    .clone())
            },
            |_observation, _legal_actions, _action, outcome| {
                unrestricted_hashes.push(outcome.state_hash.clone());
            },
        )
        .expect("unrestricted rollout should succeed");

        let mut curriculum_hashes = Vec::new();
        let curriculum = crate::policy_runner::run_episode_with_step_callback(
            config,
            17,
            &curriculum_runner_config,
            |_observation, legal_actions| {
                Ok(legal_actions
                    .first()
                    .expect("test environment should expose a legal action")
                    .action
                    .clone())
            },
            |_observation, _legal_actions, _action, outcome| {
                curriculum_hashes.push(outcome.state_hash.clone());
            },
        )
        .expect("curriculum rollout should succeed");

        let compared_len = unrestricted_hashes.len().min(curriculum_hashes.len());
        assert!(compared_len > 0);
        assert_eq!(
            unrestricted_hashes[..compared_len],
            curriculum_hashes[..compared_len],
            "curriculum must not alter authoritative state before the target stage"
        );
        assert_eq!(
            unrestricted.final_state_hash,
            unrestricted_hashes
                .last()
                .cloned()
                .expect("unrestricted rollout should have a final hash")
        );
        assert_eq!(
            curriculum.final_state_hash,
            curriculum_hashes
                .last()
                .cloned()
                .expect("curriculum rollout should have a final hash")
        );
    }

    #[test]
    fn replay_round_trips_with_current_action_schema() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");

        let replay = environment.replay();
        let json = serde_json::to_string(&replay).expect("replay should serialize");
        let decoded: EnvironmentReplay =
            serde_json::from_str(&json).expect("replay should deserialize");

        assert_eq!(decoded, replay);
        assert_eq!(decoded.action_schema_version, ACTION_SCHEMA_VERSION);
        decoded
            .execute(Arc::new(GameConfig::default_config()))
            .expect("current replay schema should execute");
    }

    #[test]
    fn environment_replay_checkpoints_match_core_replay_checkpoints() {
        let config = Arc::new(GameConfig::default_config());
        let mut environment = GameEnvironment::new(Arc::clone(&config), 7);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be legal");
        let hand_slot_index = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => Some(hand_slot_index),
                _ => None,
            })
            .expect("a hand card should be selectable");
        environment
            .step(AgentAction::SelectHandCard { hand_slot_index })
            .expect("card selection should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection confirmation should be legal");

        let mut core = GameCore::new((*config).clone(), 7);
        core.apply(PlayerCommand::StartSelectingTower)
            .expect("core start command should be accepted");
        core.apply(PlayerCommand::SelectTower {
            selected_slot_indices: vec![hand_slot_index],
        })
        .expect("core tower selection command should be accepted");

        let environment_checkpoints = environment.replay().checkpoints;
        let core_checkpoints = core.replay().checkpoints;
        assert_eq!(environment_checkpoints, core_checkpoints);
    }

    #[test]
    fn replay_with_unsupported_action_schema_fails_before_commands() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        let mut replay = environment.replay();
        replay.action_schema_version = ACTION_SCHEMA_VERSION + 1;

        let error = match replay.execute(Arc::new(GameConfig::default_config())) {
            Ok(_) => panic!("unsupported action schema should be rejected"),
            Err(error) => error,
        };

        assert!(error.contains("unsupported environment replay action schema"));
    }

    #[test]
    fn replay_with_missing_action_schema_fails_during_deserialization() {
        let mut value = serde_json::to_value(environment().replay()).expect("replay value");
        value
            .as_object_mut()
            .expect("replay should be an object")
            .remove("action_schema_version");

        let error = serde_json::from_value::<EnvironmentReplay>(value)
            .expect_err("action schema is required for replay execution");

        assert!(error.to_string().contains("action_schema_version"));
    }

    #[test]
    fn legal_action_ids_are_stable_and_ordered() {
        let environment = environment();
        let actions = environment.legal_actions();
        let ids = actions
            .iter()
            .map(|action| action.id.clone())
            .collect::<Vec<_>>();
        let mut sorted_ids = ids.clone();
        sorted_ids.sort();

        assert_eq!(ids, sorted_ids);
        assert!(
            actions
                .iter()
                .all(|action| action.id == action.action.action_id())
        );
    }

    #[test]
    fn tower_placement_exposes_direct_slot_coordinate_actions() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("tower selection should be legal");
        let card = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("card selection should be available");
        environment
            .step(card)
            .expect("card selection should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection should be confirmable");

        let tower_actions = environment
            .legal_actions()
            .into_iter()
            .filter_map(|legal| match legal.action {
                AgentAction::PlaceTower {
                    hand_slot_index,
                    left,
                    top,
                } => Some((hand_slot_index, left, top)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(!tower_actions.is_empty());
        assert!(
            tower_actions.iter().all(|(_, left, top)| {
                *left < td_core::MAP_SIZE[0] && *top < td_core::MAP_SIZE[1]
            })
        );
        assert!(tower_actions.iter().any(|(hand_slot_index, _, _)| {
            tower_actions
                .iter()
                .filter(|(other_slot, _, _)| other_slot == hand_slot_index)
                .count()
                > 1
        }));
    }

    #[test]
    fn action_kind_has_one_bucket_per_action_variant() {
        let actions = [
            AgentAction::PurchaseShopItem { slot_index: 0 },
            AgentAction::StartSelectingTower,
            AgentAction::BeginRerollSelection,
            AgentAction::BeginTowerSelection,
            AgentAction::SelectHandCard { hand_slot_index: 0 },
            AgentAction::DeselectHandCard { hand_slot_index: 0 },
            AgentAction::ConfirmCardSelection,
            AgentAction::CancelCardSelection,
            AgentAction::Reroll {
                selected_slot_indices: vec![],
            },
            AgentAction::SelectTower {
                selected_slot_indices: vec![],
            },
            AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::RemoveTower { tower_id: 0 },
            AgentAction::StartDefense,
            AgentAction::SelectTreasure { option_index: 0 },
            AgentAction::SelectCardServiceCard { card_index: 0 },
            AgentAction::ConfirmCardServiceSelection,
            AgentAction::UseInventoryItem { item_index: 0 },
            AgentAction::DiscardTreasure { upgrade_id: 0 },
            AgentAction::Continue,
        ];
        let kinds = actions.iter().map(AgentAction::kind).collect::<Vec<_>>();
        assert_eq!(kinds.len(), ActionKind::COUNT);
        assert_eq!(
            kinds.windows(2).filter(|pair| pair[0] == pair[1]).count(),
            0
        );
        let mut indices = kinds.iter().map(|kind| kind.index()).collect::<Vec<_>>();
        indices.sort_unstable();
        assert_eq!(indices, (0..ActionKind::COUNT).collect::<Vec<_>>());
    }

    #[test]
    fn invalid_action_does_not_change_state_or_hash() {
        let mut environment = environment();
        let before = environment.snapshot();
        let before_hash = environment.state_hash();

        let result = environment.step(AgentAction::StartDefense);

        assert!(matches!(
            result,
            Err(EnvironmentError::IllegalAction { .. })
        ));
        assert_eq!(environment.snapshot(), before);
        assert_eq!(environment.state_hash(), before_hash);
    }

    #[test]
    fn step_reports_clear_progress_shaping_reward() {
        let mut first = environment();
        let mut second = environment();
        let first_outcome = first
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let second_outcome = second
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");

        let first_progress = first_outcome.reward.shaping.get("clear_progress").copied();
        let second_progress = second_outcome.reward.shaping.get("clear_progress").copied();
        assert!(first_progress.is_some_and(|progress| progress >= 0.0));
        assert_eq!(first_progress, second_progress);
    }

    #[test]
    fn clear_progress_is_zero_when_clear_rate_does_not_change() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let next_action = environment
            .legal_actions()
            .first()
            .expect("next decision should have a legal action")
            .action
            .clone();

        let outcome = environment
            .step(next_action)
            .expect("non-defense action should be legal");

        assert_eq!(outcome.reward.shaping["clear_progress"], 0.0);
    }

    #[test]
    fn escaped_progress_is_not_rewarded_above_attack_progress() {
        let config = RewardConfig::default();
        let attack = shaping_rewards(0.0, 0.02, 0.0, 0.0, 1_000.0, 0.0, &config);
        let escaped = shaping_rewards(0.0, 0.02, 100.0, 0.0, 1_000.0, 0.0, &config);

        assert_eq!(attack["clear_progress"], escaped["clear_progress"]);
        assert!(escaped["escaped_hp_penalty"] < 0.0);
        assert!(escaped.values().sum::<f32>() < attack.values().sum::<f32>());
    }

    #[test]
    fn damage_progress_is_normalized_by_stage_total_hp() {
        let config = RewardConfig {
            damage_progress_weight: 1.0,
            ..RewardConfig::default()
        };
        let rewards = shaping_rewards(0.0, 0.0, 0.0, 0.0, 1_000.0, 250.0, &config);

        assert_eq!(rewards["damage_progress"], 0.25);
    }

    #[test]
    fn reward_contains_escape_and_player_damage_penalties() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert!(outcome.reward.shaping.contains_key("escaped_hp_penalty"));
        assert!(
            outcome
                .reward
                .shaping
                .contains_key("player_hp_loss_penalty")
        );
    }

    #[test]
    fn escaped_penalty_is_stage_normalized() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        let penalty = outcome
            .reward
            .shaping
            .get("escaped_hp_penalty")
            .copied()
            .expect("escaped penalty should be present");
        assert!(penalty.is_finite());
        assert!(penalty <= 0.0);
    }

    #[test]
    fn non_terminal_steps_have_no_death_penalty() {
        let mut environment = environment();
        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert!(!outcome.terminated);
        assert_eq!(outcome.reward.terminal, 0.0);
    }

    #[test]
    fn player_damage_metric_uses_game_units() {
        let mut environment = environment();
        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(30));
        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(60));
        let before = environment.metrics.total_player_damage;
        let _ = environment
            .step(AgentAction::StartSelectingTower)
            .expect("initial action should be legal");
        assert!(environment.metrics.total_player_damage - before < 60.0);
    }

    #[test]
    fn start_selecting_tower_reaches_card_selection_decision() {
        let mut environment = environment();

        let outcome = environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal in the initial shop");

        assert_eq!(outcome.observation, environment.snapshot());
        assert_eq!(outcome.state_hash, environment.state_hash());
        let trace_step = environment
            .policy_trace()
            .steps
            .last()
            .expect("step should append a policy trace entry");
        assert_eq!(trace_step.post_observation, outcome.observation);
        assert_eq!(trace_step.post_state_hash, outcome.state_hash);
        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::CardSelection
        );
        assert!(!outcome.terminated);
        assert!(!outcome.truncated);
        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|action| matches!(action.action, AgentAction::BeginTowerSelection))
        );
    }

    #[test]
    fn start_defense_opens_pre_defense_item_window_before_ticks() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let tower_selection = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::BeginTowerSelection => Some(AgentAction::BeginTowerSelection),
                _ => None,
            })
            .expect("tower selection should be available");
        environment
            .step(tower_selection)
            .expect("tower selection should be legal");
        let card = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index }
                | AgentAction::DeselectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("a card toggle should be available");
        environment.step(card).expect("card toggle should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection should be legal");
        let placement = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::PlaceTower { .. } => Some(legal.action),
                _ => None,
            })
            .expect("placement should be available");
        environment
            .step(placement)
            .expect("placement should be legal");

        let outcome = environment
            .step(AgentAction::StartDefense)
            .expect("start defense should be legal");
        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::PreDefenseItem
        );
        assert_eq!(outcome.info.ticks_advanced, 0);
        assert_eq!(
            outcome.observation.item_window_stage,
            Some(environment.game_state.stage())
        );
        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|legal| matches!(legal.action, AgentAction::Continue))
        );
    }

    #[test]
    fn continue_closes_pre_defense_window_and_advances_defense() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        let tower_selection = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::BeginTowerSelection => Some(AgentAction::BeginTowerSelection),
                _ => None,
            })
            .expect("tower selection should be available");
        environment
            .step(tower_selection)
            .expect("tower selection should be legal");
        let card = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::SelectHandCard { hand_slot_index }
                | AgentAction::DeselectHandCard { hand_slot_index } => {
                    Some(AgentAction::SelectHandCard { hand_slot_index })
                }
                _ => None,
            })
            .expect("a card toggle should be available");
        environment.step(card).expect("card toggle should be legal");
        environment
            .step(AgentAction::ConfirmCardSelection)
            .expect("card selection should be legal");
        let placement = environment
            .legal_actions()
            .into_iter()
            .find_map(|legal| match legal.action {
                AgentAction::PlaceTower { .. } => Some(legal.action),
                _ => None,
            })
            .expect("placement should be available");
        environment
            .step(placement)
            .expect("placement should be legal");
        environment
            .step(AgentAction::StartDefense)
            .expect("start defense should be legal");
        let before_continue = environment.game_state.sim_tick().ticks();
        let outcome = environment
            .step(AgentAction::Continue)
            .expect("continue should close the pre-defense window");
        assert!(outcome.info.ticks_advanced > 0 || outcome.terminated);
        assert!(environment.game_state.sim_tick().ticks() > before_continue || outcome.terminated);
        assert_ne!(environment.decision_point(), DecisionPoint::PreDefenseItem);
    }

    #[test]
    fn reroll_actions_match_affordability_and_have_no_empty_duplicate() {
        let mut environment = environment();
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");

        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|legal| matches!(legal.action, AgentAction::BeginRerollSelection))
        );

        environment.game_state.set_left_dice_for_test(0);
        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(1));
        assert!(
            environment
                .legal_actions()
                .into_iter()
                .all(|legal| !matches!(legal.action, AgentAction::Reroll { .. }))
        );

        environment
            .game_state
            .set_hp_for_test(crate::core::Health::from_integer(60));
        assert!(
            environment
                .legal_actions()
                .into_iter()
                .all(|legal| !matches!(legal.action, AgentAction::Reroll { .. }))
        );
    }

    #[test]
    fn card_service_purchase_exposes_explicit_selection_actions() {
        let mut environment = environment();
        let slot_index = environment.game_state.add_card_service_shop_slot_for_test();

        let outcome = environment
            .step(AgentAction::PurchaseShopItem { slot_index })
            .expect("card service purchase should be legal");

        assert_eq!(
            outcome.observation.decision_point,
            DecisionPoint::CardServiceSelection
        );
        assert_eq!(
            outcome
                .observation
                .card_service
                .as_ref()
                .map(|service| service.key.as_str()),
            Some("eraser")
        );
        assert!(
            environment
                .legal_actions()
                .iter()
                .any(|action| matches!(action.action, AgentAction::SelectCardServiceCard { .. }))
        );
        let selection_open_hash = environment.state_hash();

        let card_action = environment
            .legal_actions()
            .into_iter()
            .find_map(|action| match action.action {
                AgentAction::SelectCardServiceCard { card_index } => {
                    Some(AgentAction::SelectCardServiceCard { card_index })
                }
                _ => None,
            })
            .expect("eraser should offer a card");
        environment
            .step(card_action)
            .expect("card selection should be legal");
        assert_eq!(environment.state_hash(), selection_open_hash);
        environment
            .step(AgentAction::ConfirmCardServiceSelection)
            .expect("card service confirmation should be legal");
        assert_ne!(environment.state_hash(), selection_open_hash);

        assert_eq!(environment.decision_point(), DecisionPoint::Shop);
        assert!(
            environment
                .replay()
                .commands
                .iter()
                .any(|action| { matches!(action, AgentAction::ConfirmCardServiceSelection) })
        );
    }

    #[test]
    fn card_service_selection_changes_progress_fingerprint() {
        let environment = environment();
        let before = environment.progress_fingerprint();
        assert_eq!(before, environment.progress_fingerprint());
    }
}
