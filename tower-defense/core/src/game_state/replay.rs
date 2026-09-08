use sha2::Digest;

pub const CORE_REPLAY_SCHEMA_VERSION: u32 = 2;
pub const CORE_CONFIG_SCHEMA_VERSION: u32 = 1;
pub const CORE_CONFIG_DIGEST_VERSION: u32 = 1;
pub const CORE_RNG_ALGORITHM_VERSION: u32 = 1;
pub const POLICY_TRACE_SCHEMA_VERSION: u32 = 1;
pub const CORE_EVENT_DIGEST_VERSION: u32 = 1;
pub const CORE_EVENT_DIGEST_DOMAIN: &str = "td-core/core-event-digest";

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoreReplay {
    pub replay_schema_version: u32,
    pub config_version: u32,
    #[serde(default)]
    pub config_digest_version: u32,
    pub rng_algorithm_version: u32,
    #[serde(default)]
    pub event_digest_version: u32,
    pub config: crate::GameConfigState,
    pub config_digest: String,
    pub seed: u64,
    pub commands: Vec<crate::RecordedPlayerCommand>,
    pub checkpoints: Vec<ReplayCheckpoint>,
    #[serde(default)]
    pub policy_trace: Option<PolicyTrace>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayDivergence {
    pub sequence: u64,
    pub completed_sim_tick: u64,
    pub expected_hash: String,
    pub actual_hash: String,
    pub expected_event_count: u64,
    pub actual_event_count: u64,
    pub expected_event_digest: String,
    pub actual_event_digest: String,
}

impl CoreReplay {
    pub fn from_state(state: &crate::CoreState) -> Self {
        let config = state.config.clone();
        let config_digest = config_digest(&config);
        Self {
            replay_schema_version: CORE_REPLAY_SCHEMA_VERSION,
            config_version: CORE_CONFIG_SCHEMA_VERSION,
            config_digest_version: CORE_CONFIG_DIGEST_VERSION,
            rng_algorithm_version: CORE_RNG_ALGORITHM_VERSION,
            event_digest_version: CORE_EVENT_DIGEST_VERSION,
            config,
            config_digest,
            seed: state.rng.seed,
            commands: state.player_commands.clone(),
            checkpoints: state.replay_checkpoints.clone(),
            policy_trace: None,
        }
    }

    pub fn with_policy_trace(mut self, trace: PolicyTrace) -> Self {
        self.policy_trace = Some(trace);
        self
    }

    pub fn migrate_legacy_metadata(&mut self) -> Result<(), CoreReplayError> {
        if self.config_digest_version == 0 {
            self.config_digest_version = CORE_CONFIG_DIGEST_VERSION;
        } else if self.config_digest_version != CORE_CONFIG_DIGEST_VERSION {
            return Err(CoreReplayError::UnsupportedConfigDigestVersion(
                self.config_digest_version,
            ));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), CoreReplayError> {
        if self.replay_schema_version != CORE_REPLAY_SCHEMA_VERSION {
            return Err(CoreReplayError::UnsupportedReplaySchema(
                self.replay_schema_version,
            ));
        }

        if self.config_version != CORE_CONFIG_SCHEMA_VERSION {
            return Err(CoreReplayError::UnsupportedConfigSchema(
                self.config_version,
            ));
        }
        if self.config_digest_version != 0
            && self.config_digest_version != CORE_CONFIG_DIGEST_VERSION
        {
            return Err(CoreReplayError::UnsupportedConfigDigestVersion(
                self.config_digest_version,
            ));
        }
        if self.rng_algorithm_version != CORE_RNG_ALGORITHM_VERSION {
            return Err(CoreReplayError::UnsupportedRngAlgorithm(
                self.rng_algorithm_version,
            ));
        }
        if self.event_digest_version != CORE_EVENT_DIGEST_VERSION {
            return Err(CoreReplayError::UnsupportedEventDigestVersion(
                self.event_digest_version,
            ));
        }
        let actual_digest = config_digest(&self.config);
        if self.config_digest != actual_digest {
            return Err(CoreReplayError::ConfigDigestMismatch {
                expected: self.config_digest.clone(),
                actual: actual_digest,
            });
        }
        for (index, command) in self.commands.iter().enumerate() {
            let expected_sequence = index as u64;
            if command.sequence != expected_sequence {
                return Err(CoreReplayError::NonMonotonicCommand {
                    expected_sequence,
                    actual_sequence: command.sequence,
                });
            }
            if index > 0 && command.completed_sim_tick < self.commands[index - 1].completed_sim_tick
            {
                return Err(CoreReplayError::CommandTickRewound {
                    sequence: command.sequence,
                    previous_tick: self.commands[index - 1].completed_sim_tick,
                    command_tick: command.completed_sim_tick,
                });
            }
        }

        if self.checkpoints.len() > self.commands.len() {
            return Err(CoreReplayError::TooManyCheckpoints {
                commands: self.commands.len(),
                checkpoints: self.checkpoints.len(),
            });
        }
        for (index, checkpoint) in self.checkpoints.iter().enumerate() {
            let command = &self.commands[index];
            if checkpoint.sequence != command.sequence
                || checkpoint.completed_sim_tick != command.completed_sim_tick
            {
                return Err(CoreReplayError::CheckpointMetadataMismatch {
                    sequence: command.sequence,
                });
            }
        }

        if let Some(trace) = &self.policy_trace {
            trace.validate_against_replay(self)?;
        }
        Ok(())
    }
}

pub fn first_divergence(
    expected: &[ReplayCheckpoint],
    actual: &[ReplayCheckpoint],
) -> Option<ReplayDivergence> {
    expected
        .iter()
        .zip(actual)
        .find_map(|(expected, actual)| {
            (expected.sequence != actual.sequence
                || expected.completed_sim_tick != actual.completed_sim_tick
                || expected.state_hash != actual.state_hash
                || checkpoint_event_metadata_differs(expected, actual))
            .then(|| ReplayDivergence {
                sequence: expected.sequence,
                completed_sim_tick: expected.completed_sim_tick,
                expected_hash: expected.state_hash.clone(),
                actual_hash: actual.state_hash.clone(),
                expected_event_count: expected.event_count,
                actual_event_count: actual.event_count,
                expected_event_digest: expected.event_digest.clone(),
                actual_event_digest: actual.event_digest.clone(),
            })
        })
        .or_else(|| {
            (expected.len() != actual.len()).then(|| {
                match (expected.get(actual.len()), actual.get(expected.len())) {
                    (Some(expected), None) => ReplayDivergence {
                        sequence: expected.sequence,
                        completed_sim_tick: expected.completed_sim_tick,
                        expected_hash: expected.state_hash.clone(),
                        actual_hash: String::new(),
                        expected_event_count: expected.event_count,
                        actual_event_count: 0,
                        expected_event_digest: expected.event_digest.clone(),
                        actual_event_digest: String::new(),
                    },
                    (None, Some(actual)) => ReplayDivergence {
                        sequence: actual.sequence,
                        completed_sim_tick: actual.completed_sim_tick,
                        expected_hash: String::new(),
                        actual_hash: actual.state_hash.clone(),
                        expected_event_count: 0,
                        actual_event_count: actual.event_count,
                        expected_event_digest: String::new(),
                        actual_event_digest: actual.event_digest.clone(),
                    },
                    _ => unreachable!("checkpoint lengths differ at exactly one boundary"),
                }
            })
        })
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PolicyTrace {
    pub policy_trace_schema_version: u32,
    pub core_replay_schema_version: u32,
    pub environment_version: u32,
    pub action_schema_version: u32,
    pub config_version: u32,
    pub config_digest: String,
    pub rng_algorithm_version: u32,
    pub seed: u64,
    pub steps: Vec<PolicyTraceStep>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PolicyTraceStep {
    pub index: u64,
    pub decision_point: crate::DecisionPoint,
    pub agent_action: crate::AgentAction,
    pub legal_actions: Vec<crate::AgentAction>,
    pub action_mask: Vec<bool>,
    #[serde(default)]
    pub player_command: Option<crate::PlayerCommand>,
    pub pre_observation: crate::Observation,
    pub post_observation: crate::Observation,
    pub reward: crate::RewardComponents,
    pub terminated: bool,
    pub truncated: bool,
    pub info: crate::StepInfo,
    pub pre_state_hash: String,
    pub post_state_hash: String,
}

impl PolicyTrace {
    pub fn from_core_replay(
        replay: &CoreReplay,
        environment_version: u32,
        action_schema_version: u32,
    ) -> Self {
        Self {
            policy_trace_schema_version: POLICY_TRACE_SCHEMA_VERSION,
            core_replay_schema_version: replay.replay_schema_version,
            environment_version,
            action_schema_version,
            config_version: replay.config_version,
            config_digest: replay.config_digest.clone(),
            rng_algorithm_version: replay.rng_algorithm_version,
            seed: replay.seed,
            steps: Vec::new(),
        }
    }

    pub fn validate_against_replay(&self, replay: &CoreReplay) -> Result<(), CoreReplayError> {
        if self.policy_trace_schema_version != POLICY_TRACE_SCHEMA_VERSION {
            return Err(CoreReplayError::UnsupportedPolicyTraceSchema(
                self.policy_trace_schema_version,
            ));
        }
        if self.core_replay_schema_version != replay.replay_schema_version {
            return Err(CoreReplayError::UnsupportedReplaySchema(
                self.core_replay_schema_version,
            ));
        }
        if self.config_version != replay.config_version
            || self.config_digest != replay.config_digest
            || self.rng_algorithm_version != replay.rng_algorithm_version
            || self.seed != replay.seed
        {
            return Err(CoreReplayError::PolicyTraceReplayMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreReplayError {
    UnsupportedReplaySchema(u32),
    UnsupportedConfigSchema(u32),
    UnsupportedConfigDigestVersion(u32),
    UnsupportedRngAlgorithm(u32),
    UnsupportedEventDigestVersion(u32),
    UnsupportedPolicyTraceSchema(u32),
    ConfigDigestMismatch {
        expected: String,
        actual: String,
    },
    PolicyTraceReplayMismatch,
    NonMonotonicCommand {
        expected_sequence: u64,
        actual_sequence: u64,
    },
    CommandTickRewound {
        sequence: u64,
        previous_tick: u64,
        command_tick: u64,
    },
    TooManyCheckpoints {
        commands: usize,
        checkpoints: usize,
    },
    CheckpointMetadataMismatch {
        sequence: u64,
    },
}

fn config_digest(config: &crate::GameConfigState) -> String {
    let bytes = serde_json::to_vec(config).expect("core config serialization must succeed");
    let digest = sha2::Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReplayCheckpoint {
    pub sequence: u64,
    pub completed_sim_tick: u64,
    pub state_hash: String,
    #[serde(default)]
    pub event_count: u64,
    #[serde(default)]
    pub event_digest: String,
}

fn checkpoint_event_metadata_differs(
    expected: &ReplayCheckpoint,
    actual: &ReplayCheckpoint,
) -> bool {
    if expected.event_digest.is_empty() && expected.event_count == 0 {
        return false;
    }
    expected.event_count != actual.event_count || expected.event_digest != actual.event_digest
}

pub fn event_digest(events: &[crate::CoreEvent]) -> String {
    let bytes = serde_json::to_vec(&(CORE_EVENT_DIGEST_DOMAIN, CORE_EVENT_DIGEST_VERSION, events))
        .expect("core event serialization must succeed");
    let digest = sha2::Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn event_metadata(events: &[crate::CoreEvent]) -> (u64, String) {
    (
        u64::try_from(events.len()).expect("core event count must fit in u64"),
        event_digest(events),
    )
}

pub const AUTHORITATIVE_HASH_VERSION: u32 = 3;

pub fn authoritative_hash(state: &crate::CoreState) -> String {
    let mut state = state.clone();
    normalize_for_hash(&mut state);
    let bytes = serde_json::to_vec(&(AUTHORITATIVE_HASH_VERSION, state))
        .expect("core state hash serialization must succeed");
    let digest = sha2::Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn normalize_for_hash(state: &mut crate::CoreState) {
    state.progress.player_command_sequence = 0;
    state.deck.revision = 0;
    state.upgrades.revision = 0;
    state.player_commands.clear();
    state.replay_checkpoints.clear();

    for item in &mut state.items.items {
        item.id = 0;
    }
    for slot in &mut state.hand.slots {
        slot.id = 0;
        slot.selected = false;
    }
    state.hand.next_hand_slot_id = 0;
    match &mut state.flow {
        crate::GameFlowState::Shopping(shop) => {
            for slot in &mut shop.slots {
                slot.id = 0;
                normalize_shop_slot(&mut slot.slot);
            }
        }
        crate::GameFlowState::TreasureSelection { options, .. } => {
            for option in options {
                option.id = 0;
            }
        }
        crate::GameFlowState::Initializing
        | crate::GameFlowState::SelectingTower
        | crate::GameFlowState::PlacingTower
        | crate::GameFlowState::Defense(_)
        | crate::GameFlowState::Result { .. } => {}
    }
}

fn normalize_shop_slot(slot: &mut crate::ShopSlotState) {
    match slot {
        crate::ShopSlotState::Item { item, .. } => item.id = 0,
        crate::ShopSlotState::Upgrade { upgrade, .. } => upgrade.id = 0,
        crate::ShopSlotState::CardService { .. } => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> crate::GameConfigState {
        crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 5,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: vec![crate::TowerConfigEntryState {
                    kind: 0,
                    damage_raw: 1_000,
                    range_raw: 1_000_000,
                    cooldown_ms: 1_000,
                }],
            },
            monsters: crate::MonsterConfigState {
                stats: vec![crate::MonsterConfigEntryState {
                    kind: 0,
                    base_hp_raw: 1_000,
                    velocity_mul_raw: crate::RATIO_SCALE,
                    damage_raw: 100,
                    reward: 1,
                }],
                stage_waves: vec![crate::StageWaveState {
                    stage: 1,
                    entries: vec![crate::StageWaveEntryState { kind: 0, count: 1 }],
                }],
            },
        }
    }

    #[test]
    fn event_digest_is_deterministic_and_empty_events_are_supported() {
        let events = vec![crate::CoreEvent::DefenseStarted { stage: 1 }];

        assert_eq!(event_digest(&events), event_digest(&events));
        assert_eq!(event_metadata(&[]).0, 0);
        assert_eq!(event_digest(&[]), event_metadata(&[]).1);
        assert!(!event_digest(&[]).is_empty());
    }

    #[test]
    fn event_digest_is_sensitive_to_event_order() {
        let first = vec![
            crate::CoreEvent::DefenseStarted { stage: 1 },
            crate::CoreEvent::GameFinished { victory: true },
        ];
        let second = vec![
            crate::CoreEvent::GameFinished { victory: true },
            crate::CoreEvent::DefenseStarted { stage: 1 },
        ];

        assert_ne!(event_digest(&first), event_digest(&second));
    }

    #[test]
    fn replay_round_trip_preserves_event_contract() {
        let config = test_config();
        let events = vec![crate::CoreEvent::DefenseStarted { stage: 1 }];
        let (event_count, expected_digest) = event_metadata(&events);
        let replay = CoreReplay {
            replay_schema_version: CORE_REPLAY_SCHEMA_VERSION,
            config_version: CORE_CONFIG_SCHEMA_VERSION,
            config_digest_version: CORE_CONFIG_DIGEST_VERSION,
            rng_algorithm_version: CORE_RNG_ALGORITHM_VERSION,
            event_digest_version: CORE_EVENT_DIGEST_VERSION,
            config_digest: config_digest(&config),
            config,
            seed: 7,
            commands: vec![crate::RecordedPlayerCommand {
                sequence: 0,
                completed_sim_tick: 0,
                command: crate::PlayerCommand::StartDefense,
            }],
            checkpoints: vec![ReplayCheckpoint {
                sequence: 0,
                completed_sim_tick: 0,
                state_hash: "state".to_string(),
                event_count,
                event_digest: expected_digest,
            }],
            policy_trace: None,
        };

        let encoded = serde_json::to_vec(&replay).expect("replay serialization");
        let decoded: CoreReplay = serde_json::from_slice(&encoded).expect("replay deserialization");

        assert_eq!(decoded, replay);
        decoded.validate().expect("replay should validate");
    }

    #[test]
    fn replay_without_digest_version_is_accepted_as_legacy_current_schema() {
        let config = test_config();
        let replay = CoreReplay::from_state(crate::CoreSession::new(config, 7).state());
        let mut value = serde_json::to_value(replay).expect("replay serialization");
        value
            .as_object_mut()
            .expect("object")
            .remove("config_digest_version");
        let decoded: CoreReplay = serde_json::from_value(value).expect("legacy replay");
        assert_eq!(decoded.config_digest_version, 0);
        let mut decoded = decoded;
        decoded
            .migrate_legacy_metadata()
            .expect("missing digest version is a supported legacy form");
        assert_eq!(decoded.config_digest_version, CORE_CONFIG_DIGEST_VERSION);
        decoded.validate().expect("migrated replay should validate");
    }

    #[test]
    fn replay_rejects_unsupported_digest_version() {
        let config = test_config();
        let mut replay = CoreReplay::from_state(crate::CoreSession::new(config, 7).state());
        replay.config_digest_version += 1;
        assert!(matches!(
            replay.validate(),
            Err(CoreReplayError::UnsupportedConfigDigestVersion(_))
        ));
    }

    #[test]
    fn hand_identity_allocator_is_excluded_from_authoritative_hash() {
        let state = crate::CoreState::new_initial(test_config(), 7);
        let expected = crate::authoritative_hash(&state);
        let mut changed = state.clone();
        changed
            .edit_snapshot(|parts| {
                parts.hand.next_hand_slot_id = parts.hand.next_hand_slot_id.saturating_add(100);
            })
            .expect("allocator-only snapshot edit should be valid");

        assert_eq!(crate::authoritative_hash(&changed), expected);
    }

    #[test]
    fn first_divergence_reports_event_metadata() {
        let events = vec![crate::CoreEvent::DefenseStarted { stage: 1 }];
        let (event_count, event_digest) = event_metadata(&events);
        let expected = ReplayCheckpoint {
            sequence: 0,
            completed_sim_tick: 0,
            state_hash: "same".to_string(),
            event_count,
            event_digest,
        };
        let actual = ReplayCheckpoint {
            event_count: 0,
            event_digest: super::event_digest(&[]),
            ..expected.clone()
        };

        let divergence = first_divergence(&[expected], &[actual]).expect("event divergence");

        assert_eq!(divergence.expected_event_count, 1);
        assert_eq!(divergence.actual_event_count, 0);
        assert_ne!(
            divergence.expected_event_digest,
            divergence.actual_event_digest
        );
    }
}
