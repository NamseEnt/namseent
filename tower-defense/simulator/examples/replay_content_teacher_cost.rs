//! Replays saved teacher choices to count the actual collection decisions.
//! No rollout search or weight update is performed by this accounting pass.
use anyhow::{Result, ensure};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc, time::Instant};
use td_simulator::{
    environment::{DecisionPoint, GameEnvironment, Observation},
    ml::{semantic_candidates::PolicyCandidate, semantic_ppo::PpoRunMetadata},
};
#[derive(Deserialize)]
struct Sample {
    observation: Observation,
    candidates: Vec<PolicyCandidate>,
    target: usize,
}
fn main() -> Result<()> {
    for argument in std::env::args().skip(1) {
        let run = PathBuf::from(argument);
        let started = Instant::now();
        let metadata: PpoRunMetadata =
            serde_json::from_slice(&std::fs::read(run.join("ppo.json"))?)?;
        let transfer = metadata.transfer.as_ref().unwrap();
        let spec = transfer.content_teacher.as_ref().unwrap();
        let samples: Vec<Sample> =
            serde_json::from_slice(&std::fs::read(run.join("content-teacher-samples.json"))?)?;
        let configs = [
            transfer.source_environment.game_config.clone(),
            metadata.environment.as_ref().unwrap().game_config.clone(),
        ];
        let mut cursor = 0;
        let mut actual_decisions = 0;
        let mut game_decisions = Vec::new();
        for config in configs {
            let config = Arc::new(config);
            for seed in spec.seed_start..spec.seed_start + spec.games_per_config as u64 {
                let mut env = GameEnvironment::new(Arc::clone(&config), seed);
                let mut decisions = 0;
                while !matches!(env.decision_point(), DecisionPoint::Terminal) {
                    ensure!(
                        decisions < 10000,
                        "collection replay exceeded decision guard"
                    );
                    let action = if matches!(env.decision_point(), DecisionPoint::TreasureSelection)
                    {
                        let sample = &samples[cursor];
                        ensure!(
                            env.snapshot() == sample.observation,
                            "saved teacher observation mismatch at sample {cursor}"
                        );
                        cursor += 1;
                        sample.candidates[sample.target].action.clone()
                    } else {
                        td_simulator::policy_runner::canonical_scripted_semantic_action(&env)?
                    };
                    let mut outcome = env
                        .semantic_step(action)
                        .map_err(|e| anyhow::anyhow!("replay step: {e:?}"))?;
                    while !outcome.terminated && !outcome.truncated {
                        let Some(forced) = env.forced_action() else {
                            break;
                        };
                        outcome = env
                            .semantic_step(forced)
                            .map_err(|e| anyhow::anyhow!("forced step: {e:?}"))?;
                    }
                    ensure!(!outcome.truncated, "collection replay truncated");
                    decisions += 1;
                }
                actual_decisions += decisions;
                game_decisions.push(decisions);
            }
        }
        ensure!(cursor == samples.len(), "unused saved teacher samples");
        let report = serde_json::json!({"actual_collection_games":game_decisions.len(),"actual_collection_semantic_decisions":actual_decisions,"game_decisions":game_decisions,"verified_saved_observations":cursor,"accounting_replay_seconds":started.elapsed().as_secs_f64(),"scope":"replay of saved choices; accounting overhead separate from training; no new teacher labels or updates"});
        std::fs::write(
            run.join("content-teacher-collection-cost.json"),
            serde_json::to_vec_pretty(&report)?,
        )?;
        println!(
            "{}: {} actual collection decisions",
            run.display(),
            actual_decisions
        );
    }
    Ok(())
}
