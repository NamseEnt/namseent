#![allow(deprecated)]

use super::{GameState, RecordedPlayerCommand};
use crate::config::GameConfig;
use serde::Deserialize as SerdeDeserialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub const REPLAY_SCHEMA_VERSION: u32 = 2;
pub const RNG_ALGORITHM_VERSION: u32 = td_core::RNG_ALGORITHM_VERSION;

/// Read-only decoder for replay JSON emitted before `td_core::CoreReplay`.
///
/// The headed app never writes this schema. Keep this migration input until
/// previously exported replay files have an archive conversion path.
#[deprecated(note = "legacy replay JSON is migration input only; write CoreReplay")]
#[derive(Clone, Debug, PartialEq, Eq, SerdeDeserialize)]
struct LegacyReplay {
    pub schema_version: u32,
    pub rng_algorithm_version: u32,
    pub config_version: u32,
    pub config_digest: String,
    pub seed: u64,
    pub commands: Vec<RecordedPlayerCommand>,
    #[serde(default)]
    pub checkpoints: Vec<ReplayCheckpoint>,
}

pub use td_core::ReplayCheckpoint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ReplayError {
    UnsupportedSchemaVersion(u32),
    UnsupportedRngAlgorithmVersion(u32),
    UnsupportedConfigVersion(u32),
    ConfigDigestMismatch {
        expected: String,
        actual: String,
    },
    CommandTickRewound {
        sequence: u64,
        current_tick: u64,
        command_tick: u64,
    },
    CommandRejected {
        sequence: u64,
        error: crate::game_state::player_command::CommandError,
    },
    ConfigMismatch,
    CoreValidation(td_core::CoreReplayError),
    Divergence(Box<td_core::ReplayDivergence>),
    Malformed(String),
}

#[derive(Debug)]
pub(crate) struct ReplayRun {
    pub(crate) checkpoints: Vec<ReplayCheckpoint>,
}

impl LegacyReplay {
    fn into_core_replay(self, config: &GameConfig) -> Result<td_core::CoreReplay, ReplayError> {
        if self.schema_version != REPLAY_SCHEMA_VERSION {
            return Err(ReplayError::UnsupportedSchemaVersion(self.schema_version));
        }
        if self.rng_algorithm_version != RNG_ALGORITHM_VERSION {
            return Err(ReplayError::UnsupportedRngAlgorithmVersion(
                self.rng_algorithm_version,
            ));
        }
        if self.config_version != crate::config::GAME_CONFIG_VERSION {
            return Err(ReplayError::UnsupportedConfigVersion(self.config_version));
        }
        let expected_digest = legacy_config_digest(config);
        if self.config_digest != expected_digest {
            return Err(ReplayError::ConfigDigestMismatch {
                expected: self.config_digest,
                actual: expected_digest,
            });
        }
        let replay = td_core::CoreReplay {
            replay_schema_version: td_core::CORE_REPLAY_SCHEMA_VERSION,
            config_version: td_core::CORE_CONFIG_SCHEMA_VERSION,
            config_digest_version: td_core::CORE_CONFIG_DIGEST_VERSION,
            rng_algorithm_version: td_core::CORE_RNG_ALGORITHM_VERSION,
            event_digest_version: td_core::CORE_EVENT_DIGEST_VERSION,
            config: config.to_core_state(),
            config_digest: core_config_digest(&config.to_core_state()),
            seed: self.seed,
            commands: self.commands,
            checkpoints: self.checkpoints,
            policy_trace: None,
        };
        replay.validate().map_err(ReplayError::CoreValidation)?;
        Ok(replay)
    }
}

pub(crate) fn run_replay(
    replay: &td_core::CoreReplay,
    config: Arc<GameConfig>,
) -> Result<ReplayRun, ReplayError> {
    replay.validate().map_err(ReplayError::CoreValidation)?;
    if replay.config != config.to_core_state() {
        return Err(ReplayError::ConfigMismatch);
    }

    let mut game_state = super::create_game_state_with_config(config, replay.seed);
    game_state.headless = true;
    let mut actual_checkpoints = Vec::with_capacity(replay.commands.len());

    for recorded_command in &replay.commands {
        let current_tick = game_state.sim_tick().ticks();
        if recorded_command.completed_sim_tick < current_tick {
            return Err(ReplayError::CommandTickRewound {
                sequence: recorded_command.sequence,
                current_tick,
                command_tick: recorded_command.completed_sim_tick,
            });
        }
        while game_state.sim_tick().ticks() < recorded_command.completed_sim_tick {
            super::tick::advance_simulation_tick(&mut game_state);
        }

        game_state
            .apply_player_command(recorded_command.command.clone().into())
            .map_err(|error| ReplayError::CommandRejected {
                sequence: recorded_command.sequence,
                error,
            })?;
        let checkpoint = game_state
            .raw_core
            .replay_checkpoints()
            .last()
            .cloned()
            .expect("accepted player command must create a replay checkpoint");
        actual_checkpoints.push(checkpoint);
        game_state.drain_core_events();
    }

    if !replay.checkpoints.is_empty()
        && let Some(divergence) =
            td_core::first_divergence(&replay.checkpoints, &actual_checkpoints)
    {
        return Err(ReplayError::Divergence(Box::new(divergence)));
    }
    Ok(ReplayRun {
        checkpoints: actual_checkpoints,
    })
}

fn legacy_config_digest(config: &GameConfig) -> String {
    let bytes = toml::to_string(config)
        .expect("GameConfig serialization must succeed")
        .into_bytes();
    sha256_hex(&bytes)
}

fn core_config_digest(config: &td_core::GameConfig) -> String {
    let bytes = serde_json::to_vec(config).expect("core config serialization must succeed");
    sha256_hex(&bytes)
}

pub(crate) fn decode_replay_json(
    json: &str,
    legacy_config: &GameConfig,
) -> Result<td_core::CoreReplay, ReplayError> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|error| ReplayError::Malformed(error.to_string()))?;
    if value.get("replay_schema_version").is_some() {
        let mut replay = serde_json::from_value::<td_core::CoreReplay>(value)
            .map_err(|error| ReplayError::Malformed(error.to_string()))?;
        replay
            .migrate_legacy_metadata()
            .map_err(ReplayError::CoreValidation)?;
        replay.validate().map_err(ReplayError::CoreValidation)?;
        Ok(replay)
    } else {
        let legacy = serde_json::from_value::<LegacyReplay>(value)
            .map_err(|error| ReplayError::Malformed(error.to_string()))?;
        legacy.into_core_replay(legacy_config)
    }
}

pub(crate) fn authoritative_hash(game_state: &GameState) -> String {
    td_core::authoritative_hash(game_state.raw_core_state())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_replay_state(seed: u64) -> crate::game_state::GameState {
        crate::game_state::create_game_state_with_config(
            std::sync::Arc::new(GameConfig::default_config()),
            seed,
        )
    }

    fn replay_state_with_tower_selection(seed: u64) -> crate::game_state::GameState {
        let mut state = new_replay_state(seed);
        state.headless = true;
        state
            .apply_player_command(PlayerCommand::SelectTreasure { option_index: 0 }.into())
            .expect("initial treasure should be selectable");
        state.consume_core_events_now();
        state
            .apply_player_command(PlayerCommand::StartSelectingTower.into())
            .expect("tower selection should start");
        state
    }
    use crate::game_state::PlayerCommand;

    #[test]
    fn headed_core_replay_json_round_trips_versioned_metadata_and_commands() {
        let source = crate::game_state::create_game_state_with_seed(42);
        let replay = source.export_core_replay();
        let decoded: td_core::CoreReplay =
            serde_json::from_str(&serde_json::to_string_pretty(&replay).unwrap()).unwrap();

        assert_eq!(decoded, replay);
        decoded.validate().unwrap();
        assert_eq!(
            decoded.config_digest,
            serde_json::from_str::<td_core::CoreReplay>(&serde_json::to_string(&replay).unwrap())
                .unwrap()
                .config_digest
        );
    }

    #[test]
    fn legacy_replay_json_migrates_to_core_replay() {
        let config = GameConfig::default_config();
        let source = replay_state_with_tower_selection(42);
        let core_replay = source.export_core_replay();
        let legacy = LegacyReplay {
            schema_version: REPLAY_SCHEMA_VERSION,
            rng_algorithm_version: RNG_ALGORITHM_VERSION,
            config_version: crate::config::GAME_CONFIG_VERSION,
            config_digest: legacy_config_digest(&config),
            seed: core_replay.seed,
            commands: core_replay.commands.clone(),
            checkpoints: core_replay.checkpoints.clone(),
        };

        let legacy_json = serde_json::json!({
            "schema_version": legacy.schema_version,
            "rng_algorithm_version": legacy.rng_algorithm_version,
            "config_version": legacy.config_version,
            "config_digest": legacy.config_digest,
            "seed": legacy.seed,
            "commands": legacy.commands,
            "checkpoints": legacy.checkpoints,
        });
        let migrated =
            decode_replay_json(&legacy_json.to_string(), &config).expect("legacy replay migration");

        assert_eq!(migrated, core_replay);
        migrated.validate().unwrap();
    }

    #[test]
    fn core_replay_config_digest_mutation_is_rejected() {
        let config = GameConfig::default_config();
        let replay = crate::game_state::create_game_state_with_seed(42).export_core_replay();
        let mut value = serde_json::to_value(replay).unwrap();
        value["config_digest"] = serde_json::json!("invalid");

        let error = decode_replay_json(&value.to_string(), &config).unwrap_err();

        assert!(matches!(
            error,
            ReplayError::CoreValidation(td_core::CoreReplayError::ConfigDigestMismatch { .. })
        ));
    }

    #[test]
    fn malformed_core_replay_command_log_is_rejected() {
        let config = GameConfig::default_config();
        let mut replay = crate::game_state::create_game_state_with_seed(42).export_core_replay();
        replay.commands.push(RecordedPlayerCommand {
            sequence: 3,
            completed_sim_tick: 0,
            command: PlayerCommand::StartSelectingTower,
        });

        let error =
            decode_replay_json(&serde_json::to_string(&replay).unwrap(), &config).unwrap_err();

        assert!(matches!(
            error,
            ReplayError::CoreValidation(td_core::CoreReplayError::NonMonotonicCommand { .. })
        ));
    }

    #[test]
    fn first_divergence_reports_the_first_mismatching_command() {
        let expected = vec![
            ReplayCheckpoint {
                sequence: 0,
                completed_sim_tick: 3,
                state_hash: "a".to_string(),
                event_count: 0,
                event_digest: String::new(),
            },
            ReplayCheckpoint {
                sequence: 1,
                completed_sim_tick: 8,
                state_hash: "b".to_string(),
                event_count: 0,
                event_digest: String::new(),
            },
        ];
        let actual = vec![
            expected[0].clone(),
            ReplayCheckpoint {
                state_hash: "c".to_string(),
                ..expected[1].clone()
            },
        ];

        let divergence = td_core::first_divergence(&expected, &actual).unwrap();

        assert_eq!(divergence.sequence, 1);
        assert_eq!(divergence.completed_sim_tick, 8);
        assert_eq!(divergence.expected_hash, "b");
        assert_eq!(divergence.actual_hash, "c");
    }

    #[test]
    fn replay_runner_reproduces_checkpoints_and_reports_mutated_checkpoint() {
        let source = replay_state_with_tower_selection(42);
        let config = GameConfig::default_config();
        let replay_json = serde_json::to_string_pretty(&source.export_core_replay()).unwrap();
        let replay = decode_replay_json(&replay_json, &config).unwrap();
        let config = std::sync::Arc::new(config);

        let run = run_replay(&replay, config.clone()).unwrap();

        assert_eq!(run.checkpoints, source.replay_checkpoints);

        let mut mutated = replay;
        mutated.checkpoints[0].state_hash = "different".to_string();
        let error = run_replay(&mutated, config).unwrap_err();
        let ReplayError::Divergence(divergence) = error else {
            panic!("mutated checkpoint should be rejected as a divergence");
        };
        assert_eq!(divergence.sequence, 0);
        assert_eq!(divergence.completed_sim_tick, 0);
        assert_eq!(divergence.expected_hash, "different");
        assert_eq!(
            divergence.actual_hash,
            source.replay_checkpoints[0].state_hash
        );
    }

    #[test]
    fn replay_runner_reports_mutated_event_checkpoint() {
        let source = replay_state_with_tower_selection(42);
        let config = GameConfig::default_config();
        let replay_json = serde_json::to_string(&source.export_core_replay()).unwrap();
        let mut replay = decode_replay_json(&replay_json, &config).unwrap();
        replay.checkpoints[0].event_digest = "different".to_string();

        let error = run_replay(&replay, std::sync::Arc::new(config)).unwrap_err();
        let ReplayError::Divergence(divergence) = error else {
            panic!("mutated event checkpoint should be rejected");
        };
        assert_eq!(
            divergence.expected_event_count,
            replay.checkpoints[0].event_count
        );
        assert_eq!(
            divergence.actual_event_count,
            source.replay_checkpoints[0].event_count
        );
        assert_eq!(divergence.expected_event_digest, "different");
        assert_eq!(
            divergence.actual_event_digest,
            source.replay_checkpoints[0].event_digest
        );
    }

    #[test]
    fn command_only_replay_runs_without_checkpoint_divergence() {
        let config = GameConfig::default_config();
        let source = replay_state_with_tower_selection(42);
        let mut replay = source.export_core_replay();
        replay.checkpoints.clear();

        let run = run_replay(&replay, std::sync::Arc::new(config)).unwrap();

        assert_eq!(run.checkpoints.len(), 2);
    }

    #[test]
    fn headed_commands_match_public_core_session_checkpoints() {
        let mut source = new_replay_state(0xC0DE);
        source
            .apply_player_command(PlayerCommand::SelectTreasure { option_index: 0 }.into())
            .expect("initial treasure should be selectable");
        source.consume_core_events_now();
        for command in [
            PlayerCommand::StartSelectingTower,
            PlayerCommand::Reroll {
                selected_slot_indices: Vec::new(),
            },
        ] {
            source
                .apply_player_command(command.into())
                .expect("headed command should be accepted");
        }

        let replay = source.export_core_replay();
        let mut core = td_core::CoreSession::new(replay.config.clone(), replay.seed);
        for recorded in &replay.commands {
            while core.sim_tick().ticks() < recorded.completed_sim_tick {
                core.advance_tick();
            }
            let receipt = core
                .apply(recorded.command.clone())
                .expect("core command should be accepted");
            assert_eq!(
                receipt.state_hash,
                replay.checkpoints[recorded.sequence as usize].state_hash,
            );
            assert_eq!(
                receipt.event_digest,
                replay.checkpoints[recorded.sequence as usize].event_digest
            );
        }

        assert_eq!(core.replay(), replay);
        assert_eq!(core.authoritative_hash(), source.authoritative_hash());
    }

    #[test]
    fn tracking_revisions_do_not_change_authoritative_hash() {
        let mut deck_revision_changed = crate::game_state::create_game_state_with_seed(42);
        let mut upgrade_revision_changed = crate::game_state::create_game_state_with_seed(42);
        let baseline = crate::game_state::create_game_state_with_seed(42);

        deck_revision_changed.deck.discard(std::iter::empty());
        upgrade_revision_changed.upgrade_state.revision = upgrade_revision_changed
            .upgrade_state
            .revision
            .wrapping_add(1);

        assert_eq!(
            authoritative_hash(&baseline),
            authoritative_hash(&deck_revision_changed)
        );
        assert_eq!(
            authoritative_hash(&baseline),
            authoritative_hash(&upgrade_revision_changed)
        );
    }

    #[test]
    fn first_divergence_reports_the_extra_checkpoint_side() {
        let expected = vec![ReplayCheckpoint {
            sequence: 0,
            completed_sim_tick: 2,
            state_hash: "a".to_string(),
            event_count: 0,
            event_digest: String::new(),
        }];
        let actual = vec![
            expected[0].clone(),
            ReplayCheckpoint {
                sequence: 1,
                completed_sim_tick: 5,
                state_hash: "b".to_string(),
                event_count: 0,
                event_digest: String::new(),
            },
        ];

        let divergence = td_core::first_divergence(&expected, &actual).unwrap();

        assert_eq!(divergence.sequence, 1);
        assert_eq!(divergence.completed_sim_tick, 5);
        assert!(divergence.expected_hash.is_empty());
        assert_eq!(divergence.actual_hash, "b");
    }
}
