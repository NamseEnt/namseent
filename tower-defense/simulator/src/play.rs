use std::sync::Arc;
use std::time::Instant;

use anyhow::{Result, bail};

use crate::config::GameConfig;
use crate::environment::{DecisionPoint, GameEnvironment};
use crate::policy_runner::canonical_scripted_semantic_action;
use crate::teacher::settle_forced_actions;
use crate::teacher_selection::{TeacherSelectionPools, select_teacher_action};
use crate::teacher_terminal_gate::MAX_EPISODE_DECISIONS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayPolicy {
    Scripted,
    Teacher,
}

pub struct PlaySummary {
    pub seed: u64,
    pub decision_count: usize,
    pub override_count: usize,
    pub victory: bool,
    pub clear_rate: f32,
    pub final_stage: usize,
    pub max_stages: usize,
    pub elapsed_seconds: f64,
}

pub fn play_episode(config: Arc<GameConfig>, seed: u64, policy: PlayPolicy) -> Result<PlaySummary> {
    let max_stages = config.player.max_stages;
    let pools = TeacherSelectionPools::production();
    let mut environment = GameEnvironment::new(config, seed);
    let started = Instant::now();
    let mut decision_count = 0;
    let mut override_count = 0;

    while !matches!(environment.decision_point(), DecisionPoint::Terminal) {
        if decision_count >= MAX_EPISODE_DECISIONS {
            bail!("episode did not finish within {MAX_EPISODE_DECISIONS} decisions");
        }
        let observation = environment.snapshot();
        let decision_started = Instant::now();
        let (action, overridden) = match policy {
            PlayPolicy::Scripted => (canonical_scripted_semantic_action(&environment)?, false),
            PlayPolicy::Teacher => {
                let (decision, action) = select_teacher_action(&environment, &pools)?;
                let overridden = decision.selected_action_id != decision.baseline_action_id;
                (action, overridden)
            }
        };
        if overridden {
            override_count += 1;
        }
        let hp_percent = observation.hp_raw as f64 * 100.0 / observation.max_hp_raw.max(1) as f64;
        println!(
            "#{decision_count:<4} stage {:>2}/{max_stages} hp {hp_percent:>5.1}% gold {:>4} {:<20} {}{} ({:.1}s)",
            observation.stage,
            observation.gold,
            format!("{:?}", observation.decision_point),
            action.action_id(),
            if overridden {
                "  [teacher override]"
            } else {
                ""
            },
            decision_started.elapsed().as_secs_f64(),
        );
        decision_count += 1;
        let mut outcome = environment
            .semantic_step(action)
            .map_err(|error| anyhow::anyhow!("play step failed: {error:?}"))?;
        settle_forced_actions(&mut environment, &mut outcome)?;
        if outcome.truncated {
            bail!("episode hit the environment tick limit");
        }
        if outcome.terminated {
            break;
        }
    }

    let clear_rate = environment.clear_rate();
    Ok(PlaySummary {
        seed,
        decision_count,
        override_count,
        victory: clear_rate >= 100.0,
        clear_rate,
        final_stage: environment.snapshot().stage,
        max_stages,
        elapsed_seconds: started.elapsed().as_secs_f64(),
    })
}
