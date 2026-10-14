use crate::config::GameConfig;
use crate::environment::{AgentAction, GameEnvironment, LegalAction};
use crate::ml::semantic_candidates::{CandidateMode, policy_candidates_with};
use std::sync::Arc;

mod tests {
    use super::*;
    use crate::ml::{
        model::{InferenceBackend, default_policy_device},
        policy_v2::PolicyNet,
        semantic_bc::semantic_decision_with,
        semantic_ppo::{actor_log_prob_vectors, load_ppo_actor},
    };

    #[test]
    #[ignore = "development experiment: runs hundreds of terminal teacher rollouts"]
    fn exploratory_construction_treasure_value_probe() {
        use crate::teacher::{RolloutTeacherConfig, evaluate_semantic_candidate_set};

        const STATE_SEED: u64 = 20261013;
        const SCENARIO_SEEDS: [u64; 4] = [2026101301, 2026101302, 2026101303, 2026101304];
        const MAX_CONTINUATIONS: usize = 2_000;
        let config = Arc::new(GameConfig::default_config());
        let mut plain = GameEnvironment::new(Arc::clone(&config), STATE_SEED);
        let mut treasure = GameEnvironment::new(config, STATE_SEED);
        for environment in [&mut plain, &mut treasure] {
            environment
                .test_only_set_cross_suit_straight_flush_hand()
                .expect("valid cross-suit hand fixture");
        }
        treasure
            .test_only_seed_owned_upgrade(td_core::UpgradeKind::BlackWhite)
            .expect("valid owned-upgrade fixture");

        let build_actions = |environment: &GameEnvironment| {
            let candidates = policy_candidates_with(environment, CandidateMode::AllBuildOptions)
                .expect("all build options");
            let actions = candidates
                .candidates
                .iter()
                .filter(|candidate| matches!(candidate.action, AgentAction::BuildTower { .. }))
                .map(|candidate| LegalAction {
                    id: candidate.action_id.clone(),
                    action: candidate.action.clone(),
                })
                .collect::<Vec<_>>();
            (candidates, actions)
        };
        let (plain_candidates, plain_actions) = build_actions(&plain);
        let (treasure_candidates, treasure_actions) = build_actions(&treasure);
        assert_eq!(plain_actions.len(), treasure_actions.len());
        let planned_continuations =
            (plain_actions.len() + treasure_actions.len() + 2) * SCENARIO_SEEDS.len();
        assert!(
            planned_continuations <= MAX_CONTINUATIONS,
            "probe budget exceeded before execution: {planned_continuations}"
        );
        let teacher_config = RolloutTeacherConfig {
            scenario_seeds: Vec::new(),
            horizon_sim_ticks: 1_000_000,
            build_tower_rollout_limit: None,
        };
        let paired_scenario_values = SCENARIO_SEEDS[..4]
            .iter()
            .map(|scenario_seed| {
                let config = RolloutTeacherConfig {
                    scenario_seeds: vec![*scenario_seed],
                    ..teacher_config.clone()
                };
                Ok(serde_json::json!({
                    "scenario_seed": scenario_seed,
                    "plain": evaluate_semantic_candidate_set(&plain, &plain_actions, &config)?,
                    "treasure": evaluate_semantic_candidate_set(&treasure, &treasure_actions, &config)?,
                }))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .expect("paired per-scenario treasure rollout values");

        let result = serde_json::json!({
            "schema_version": 1,
            "purpose": "exploratory real-Core construction value probe; no model fitting",
            "state_seed": STATE_SEED,
            "scenario_seeds": SCENARIO_SEEDS,
            "source_checkpoint": "artifacts/retraining/effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004",
            "source_checkpoint_hashes": {
                "actor.bin": "f1ff8a5c371194439cb9fd702521a712b89b444e46c1ba998c5ffa0dcba0bddc",
                "critic.bin": "a5e615bd65d17d4c6443715ac709e14b6abb228fc3101760b5419561ce631bb3",
                "ppo-actor.json": "c95f1192f67a77811657b8d6583bcc6e83c97ac3e5dcdfe751f8568eeec12d80"
            },
            "source_contract": {"observation_schema": 10, "entity_encoder": 6, "candidate_encoder": 1, "candidate_mode": "top8", "policy_representation": 4, "input_contract": "normalized_effects"},
            "source_checkpoint_used_for_rollout": false,
            "continuation_policy": "canonical_scripted_semantic_action",
            "horizon_sim_ticks": teacher_config.horizon_sim_ticks,
            "planned_continuations": planned_continuations,
            "actual_candidate_scenario_rollouts": paired_scenario_values.iter().map(|pair| pair["plain"]["candidate_count"].as_u64().unwrap_or_default() as usize + pair["treasure"]["candidate_count"].as_u64().unwrap_or_default() as usize).sum::<usize>(),
            "plain_state_hash": plain.state_hash(),
            "treasure_state_hash": treasure.state_hash(),
            "plain_observation": plain.snapshot(),
            "treasure_observation": treasure.snapshot(),
            "plain_candidate_count": plain_candidates.candidates.len(),
            "treasure_candidate_count": treasure_candidates.candidates.len(),
            "plain_build_option_info": plain_candidates.build_option_info,
            "treasure_build_option_info": treasure_candidates.build_option_info,
            "paired_scenario_values": paired_scenario_values,
            "aggregate_values": "see separate 8-scenario confirmation artifacts; raw paired scenarios are recorded here",
        });
        if let Some(path) = std::env::var_os("CONSTRUCTION_CONTEXT_STUDY_OUTPUT") {
            std::fs::write(
                path,
                serde_json::to_vec_pretty(&result).expect("serialize study result"),
            )
            .expect("write study result artifact");
        }
        println!(
            "CONSTRUCTION_CONTEXT_STUDY={}",
            serde_json::to_string(&result).unwrap()
        );
    }

    #[test]
    #[ignore = "development experiment: runs hundreds of terminal teacher rollouts"]
    fn exploratory_construction_deck_value_probe() {
        use crate::teacher::{RolloutTeacherConfig, evaluate_semantic_candidate_set};

        const STATE_SEED: u64 = 20261014;
        const SCENARIO_SEEDS: [u64; 4] = [2026101401, 2026101402, 2026101403, 2026101404];
        const MAX_CONTINUATIONS: usize = 2_000;
        let config = Arc::new(GameConfig::default_config());
        let mut baseline = GameEnvironment::new(Arc::clone(&config), STATE_SEED);
        let mut changed_deck = GameEnvironment::new(config, STATE_SEED);
        for environment in [&mut baseline, &mut changed_deck] {
            environment
                .test_only_set_cross_suit_straight_flush_hand()
                .expect("valid cross-suit hand fixture");
        }
        let moved_card_id = changed_deck
            .test_only_move_draw_card_to_discard()
            .expect("validated draw/discard membership edit");
        let build_actions = |environment: &GameEnvironment| {
            let candidates = policy_candidates_with(environment, CandidateMode::AllBuildOptions)
                .expect("all build options");
            let actions = candidates
                .candidates
                .iter()
                .filter(|candidate| matches!(candidate.action, AgentAction::BuildTower { .. }))
                .map(|candidate| LegalAction {
                    id: candidate.action_id.clone(),
                    action: candidate.action.clone(),
                })
                .collect::<Vec<_>>();
            (candidates, actions)
        };
        let (baseline_candidates, baseline_actions) = build_actions(&baseline);
        let (changed_candidates, changed_actions) = build_actions(&changed_deck);
        assert_eq!(baseline_actions.len(), changed_actions.len());
        let planned_continuations =
            (baseline_actions.len() + changed_actions.len() + 2) * SCENARIO_SEEDS.len();
        assert!(
            planned_continuations <= MAX_CONTINUATIONS,
            "probe budget exceeded before execution: {planned_continuations}"
        );
        let teacher_config = RolloutTeacherConfig {
            scenario_seeds: Vec::new(),
            horizon_sim_ticks: 1_000_000,
            build_tower_rollout_limit: None,
        };
        let paired_scenario_values = SCENARIO_SEEDS
            .iter()
            .map(|scenario_seed| {
                let config = RolloutTeacherConfig {
                    scenario_seeds: vec![*scenario_seed],
                    ..teacher_config.clone()
                };
                Ok(serde_json::json!({
                    "scenario_seed": scenario_seed,
                    "baseline": evaluate_semantic_candidate_set(&baseline, &baseline_actions, &config)?,
                    "changed": evaluate_semantic_candidate_set(&changed_deck, &changed_actions, &config)?,
                }))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .expect("paired per-scenario deck rollout values");
        let result = serde_json::json!({
            "schema_version": 1,
            "purpose": "exploratory real-Core construction value probe; no model fitting",
            "state_seed": STATE_SEED,
            "scenario_seeds": SCENARIO_SEEDS,
            "source_checkpoint": "artifacts/retraining/effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004",
            "source_checkpoint_hashes": {
                "actor.bin": "f1ff8a5c371194439cb9fd702521a712b89b444e46c1ba998c5ffa0dcba0bddc",
                "critic.bin": "a5e615bd65d17d4c6443715ac709e14b6abb228fc3101760b5419561ce631bb3",
                "ppo-actor.json": "c95f1192f67a77811657b8d6583bcc6e83c97ac3e5dcdfe751f8568eeec12d80"
            },
            "source_contract": {"observation_schema": 10, "entity_encoder": 6, "candidate_encoder": 1, "candidate_mode": "top8", "policy_representation": 4, "input_contract": "normalized_effects"},
            "source_checkpoint_used_for_rollout": false,
            "moved_card_id": moved_card_id,
            "continuation_policy": "canonical_scripted_semantic_action",
            "horizon_sim_ticks": teacher_config.horizon_sim_ticks,
            "planned_continuations": planned_continuations,
            "actual_candidate_scenario_rollouts": paired_scenario_values.iter().map(|pair| pair["baseline"]["candidate_count"].as_u64().unwrap_or_default() as usize + pair["changed"]["candidate_count"].as_u64().unwrap_or_default() as usize).sum::<usize>(),
            "baseline_state_hash": baseline.state_hash(),
            "changed_state_hash": changed_deck.state_hash(),
            "baseline_observation": baseline.snapshot(),
            "changed_observation": changed_deck.snapshot(),
            "baseline_candidate_count": baseline_candidates.candidates.len(),
            "changed_candidate_count": changed_candidates.candidates.len(),
            "baseline_build_option_info": baseline_candidates.build_option_info,
            "changed_build_option_info": changed_candidates.build_option_info,
            "paired_scenario_values": paired_scenario_values,
            "value_scope": "raw per-scenario estimates; 8-scenario aggregate is in the separate confirmation artifact",
        });
        if let Some(path) = std::env::var_os("CONSTRUCTION_CONTEXT_STUDY_OUTPUT") {
            std::fs::write(
                path,
                serde_json::to_vec_pretty(&result).expect("serialize study result"),
            )
            .expect("write study result artifact");
        }
        println!(
            "CONSTRUCTION_CONTEXT_STUDY={}",
            serde_json::to_string(&result).unwrap()
        );
    }

    #[test]
    #[ignore = "requires the retained release-smoke actor artifact"]
    fn smoke_actor_receives_treasure_and_deck_context_for_build_options() {
        let config = Arc::new(GameConfig::default_config());
        let mut plain = GameEnvironment::new(Arc::clone(&config), 20261013);
        let mut treasure = GameEnvironment::new(Arc::clone(&config), 20261013);
        let mut baseline_deck = GameEnvironment::new(Arc::clone(&config), 20261014);
        let mut changed_deck = GameEnvironment::new(config, 20261014);
        for environment in [
            &mut plain,
            &mut treasure,
            &mut baseline_deck,
            &mut changed_deck,
        ] {
            environment
                .test_only_set_cross_suit_straight_flush_hand()
                .expect("valid hand fixture");
        }
        treasure
            .test_only_seed_owned_upgrade(td_core::UpgradeKind::BlackWhite)
            .expect("valid owned-upgrade fixture");
        assert_eq!(
            changed_deck.test_only_move_draw_card_to_discard().unwrap(),
            46
        );

        let actor_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts/retraining/release-smoke-20261013/active-continuous/iter-0002");
        let device = default_policy_device();
        let actor: PolicyNet<InferenceBackend> =
            load_ppo_actor(&actor_path, &device).expect("load actual release smoke target actor");
        let actor_probabilities = |environment: &GameEnvironment| {
            let decision = semantic_decision_with(environment, CandidateMode::AllBuildOptions)
                .expect("all-build actor decision");
            let log_probs = actor_log_prob_vectors(&actor, &[&decision.encoded], &device)
                .expect("actual actor forward")
                .remove(0);
            let build_indices = decision
                .candidates
                .candidates
                .iter()
                .enumerate()
                .filter_map(|(index, candidate)| {
                    matches!(candidate.action, AgentAction::BuildTower { .. }).then_some(index)
                })
                .collect::<Vec<_>>();
            let denominator = build_indices
                .iter()
                .map(|index| log_probs[*index].exp())
                .sum::<f32>();
            let probabilities = build_indices
                .iter()
                .map(|index| {
                    let candidate = &decision.candidates.candidates[*index];
                    let AgentAction::BuildTower {
                        card_ids,
                        hand_slot_index,
                        ..
                    } = &candidate.action
                    else {
                        unreachable!()
                    };
                    let mut card_ids = card_ids.clone();
                    card_ids.sort_unstable();
                    serde_json::json!({
                        "action_id": candidate.action_id,
                        "card_ids": card_ids,
                        "tower_slot": hand_slot_index,
                        "probability_within_build_tower_family": log_probs[*index].exp() / denominator,
                        "outside_top8": decision.candidates.build_option_info[*index]
                            .as_ref().map(|info| info.outside_v1_top8),
                    })
                })
                .collect::<Vec<_>>();
            (decision.encoded, probabilities)
        };
        let (plain_encoded, plain_probabilities) = actor_probabilities(&plain);
        let (treasure_encoded, treasure_probabilities) = actor_probabilities(&treasure);
        let (baseline_encoded, baseline_probabilities) = actor_probabilities(&baseline_deck);
        let (changed_encoded, changed_probabilities) = actor_probabilities(&changed_deck);
        assert_ne!(plain_encoded.typed, treasure_encoded.typed);
        assert_ne!(baseline_encoded.typed, changed_encoded.typed);
        let result = serde_json::json!({
            "schema_version": 1,
            "actor_checkpoint": actor_path,
            "actor_checkpoint_sha256": {
                "actor.bin": "e19d4960ab27f6977875d5123686c46c83d7014ca39e938505e9329eff90b153",
                "critic.bin": "a7fc9f79267f4fcf242b7f53ff5387fb82afcdf2bf93ed4d1506010a4ede2cd9",
                "ppo-actor.json": "fc0b59746410ed3b735c9cceb88808a203100446e869b668bfec43b3568d95b5"
            },
            "source_checkpoint": "artifacts/retraining/effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004",
            "source_checkpoint_sha256": {
                "actor.bin": "f1ff8a5c371194439cb9fd702521a712b89b444e46c1ba998c5ffa0dcba0bddc",
                "critic.bin": "a5e615bd65d17d4c6443715ac709e14b6abb228fc3101760b5419561ce631bb3",
                "ppo-actor.json": "c95f1192f67a77811657b8d6583bcc6e83c97ac3e5dcdfe751f8568eeec12d80"
            },
            "candidate_mode": "all_build_options",
            "encoder_contract": {"observation_schema": 10, "entity_encoder": 7, "candidate_encoder": 2},
            "verified_encoding_source_sha256": {"candidate": "b434979dd2f803bede266790f507d80b166ecb96e511cecc55d2e221b2e3c12d"},
            "input_contract": "normalized_effects",
            "ppo_source_run": {"actor_learning_rate": 0.00001, "ppo_seed": 880013, "training_seed_range": [8800000, 8800007], "iterations": 2},
            "selection_probability_scope": "conditional within BuildTower family",
            "treasure": {
                "plain_state_hash": plain.state_hash(),
                "treasure_state_hash": treasure.state_hash(),
                "actor_state_input_changed": plain_encoded.typed != treasure_encoded.typed,
                "actor_effect_input_changed": plain_encoded.state_effects != treasure_encoded.state_effects,
                "plain_probabilities": plain_probabilities,
                "treasure_probabilities": treasure_probabilities,
            },
            "deck": {
                "baseline_state_hash": baseline_deck.state_hash(),
                "changed_state_hash": changed_deck.state_hash(),
                "moved_card_id": 46,
                "actor_state_input_changed": baseline_encoded.typed != changed_encoded.typed,
                "actor_effect_input_changed": baseline_encoded.state_effects != changed_encoded.state_effects,
                "baseline_probabilities": baseline_probabilities,
                "changed_probabilities": changed_probabilities,
            },
        });
        if let Some(path) = std::env::var_os("CONSTRUCTION_CONTEXT_ACTOR_OUTPUT") {
            std::fs::write(
                path,
                serde_json::to_vec_pretty(&result).expect("serialize actor response"),
            )
            .expect("write actor response artifact");
        }
        println!(
            "CONSTRUCTION_CONTEXT_ACTOR={}",
            serde_json::to_string(&result).unwrap()
        );
    }
}
