use crate::config::GameConfig;
use crate::environment::StepReason;
use crate::ml::MlContract;
use crate::ml::managed;
use crate::ml::semantic_bc::SemanticPolicy;
use crate::ml::support::{current_git_revision, sha256_hex};
use crate::policy_runner::{BatchResult, PolicyRunnerConfig};
use crate::recording::{SimRecorder, SimulationProvenance};
use anyhow::Context;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const TARGET_ACT_CLEAR_RATES: [f32; 5] = [0.823, 0.734, 0.611, 0.456, 0.297];
const BASE_LEARNING_RATE: f32 = 1.00;
const STABLE_ITERATIONS_REQUIRED: usize = 3;
const STAGE_SCALE_CLAMP_MIN_FLOOR: f32 = 0.25;
const STAGE_SCALE_CLAMP_MAX_CEILING: f32 = 8.00;
const STAGE_SCALE_RELATIVE_MIN: f32 = 0.25;
const STAGE_SCALE_RELATIVE_MAX: f32 = 4.00;
const STAGE_SCALE_LOG_STEP_MAX: f32 = 0.20;
const BLEND_ALPHA: f32 = 0.65;
const ACT_CLEAR_SIGNAL_WEIGHT: f32 = 1.0;
const INTRA_ACT_SIGNAL_SMOOTHING: f32 = 0.25;
const CLEAR_RATE_BIN_COUNT: usize = 51;
const ACT_LOG_ERROR_CLAMP: f32 = 1.50;
const STAGE_LOG_ERROR_CLAMP: f32 = 1.00;
const STAGE_RESIDUAL_CLAMP: f32 = 0.50;
const STAGE_CLEAR_RATE_LOG_SHAPE: f32 = 0.10;
const MIN_CONVERGENCE_ARRIVALS: usize = 20;
const ACT_RATE_CONVERGENCE_TOLERANCE: f32 = 0.05;
const STAGE_CLEAR_RMSE_CONVERGENCE_THRESHOLD: f32 = 0.08;

#[derive(Args)]
pub struct BalanceOptions {
    /// Number of simulation samples to run
    #[arg(short, long, default_value_t = 1000)]
    samples: usize,

    /// Explicit inference checkpoint override; defaults to the managed active policy
    #[arg(long)]
    checkpoint: Option<PathBuf>,

    /// First simulation seed (the batch uses a contiguous seed range)
    #[arg(long, default_value_t = 0)]
    seed_start: u64,

    /// SQLite database path for recording results
    #[arg(short, long, default_value = "sim_results.db")]
    db: PathBuf,

    /// Delete the target database file before running
    #[arg(long)]
    fresh_db: bool,

    /// Number of threads (0 = auto-detect)
    #[arg(short, long, default_value_t = 0)]
    threads: usize,

    /// Input simulation config JSONC file
    #[arg(long)]
    config: Option<PathBuf>,

    /// Output tuned game config JSONC file; the source config is never overwritten by default
    #[arg(
        long,
        default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/artifacts/balance/gameconfig.tuned.jsonc")
    )]
    output: PathBuf,

    /// Stop after this many tuning iterations
    #[arg(long)]
    max_iterations: Option<usize>,

    /// Suppress progress output while tuning
    #[arg(long)]
    quiet: bool,

    /// Normalize all normal-monster base HP to this value before tuning
    #[arg(long)]
    initial_base_hp: Option<f32>,
}

struct BalancePolicy {
    policy: SemanticPolicy,
    checkpoint_path: String,
    checkpoint_iteration: usize,
    checkpoint_sha256: String,
    model_sha256: String,
    git_revision: String,
    policy_id: String,
}

struct BalanceTuning<'a> {
    pool: &'a rayon::ThreadPool,
    cli: &'a BalanceOptions,
    recorder: &'a Arc<SimRecorder>,
    stop_requested: &'a Arc<AtomicBool>,
    policy: &'a BalancePolicy,
    seed_end: u64,
}

struct BalanceStats {
    win_rate: f32,
    stage_arrivals: Vec<usize>,
    stage_clear_rates: Vec<f32>,
    act_clear_rates: Vec<f32>,
    act_survival_rates: Vec<f32>,
    clear_rates: Vec<f32>,
    overall_damage_mean: f32,
    below_15_frac: f32,
    termination_counts: BTreeMap<String, usize>,
    stage_one_termination_counts: BTreeMap<String, usize>,
    stage_one_cycle_action_pairs: BTreeMap<String, usize>,
    terminal_tower_mean: f32,
    terminal_player_damage_mean: f32,
    terminal_escaped_hp_mean: f32,
    terminal_item_use_mean: f32,
    terminal_tower_damage_mean: f32,
    config_digest: String,
    policy_id: String,
}

pub fn run(cli: BalanceOptions) -> anyhow::Result<()> {
    anyhow::ensure!(cli.samples > 0, "--samples must be positive");
    let mut base_config = if let Some(path) = &cli.config {
        GameConfig::from_jsonc(path).map_err(anyhow::Error::msg)?
    } else {
        crate::config::load_default_config()?
    };
    let source_config_digest = MlContract::from_config(&base_config).config_digest;
    let seed_end = cli
        .seed_start
        .checked_add(u64::try_from(cli.samples)? - 1)
        .context("balance seed range overflow")?;
    if let Some(initial_base_hp) = cli.initial_base_hp {
        normalize_normal_monster_hp(&mut base_config, initial_base_hp.max(1.0));
    }
    apply_boss_hp_from_stage(&mut base_config);

    let checkpoint_path = managed::resolve_selected_checkpoint(cli.checkpoint.as_deref())?;
    let (semantic_policy, iteration) =
        crate::simulation::load_semantic_policy(&checkpoint_path, &base_config, true)?;
    let model_sha256 = sha256_hex(&std::fs::read(checkpoint_path.join("actor.bin"))?);
    let policy = BalancePolicy {
        policy: semantic_policy,
        checkpoint_path: checkpoint_path.display().to_string(),
        checkpoint_iteration: iteration.unwrap_or_default(),
        checkpoint_sha256: model_sha256.clone(),
        policy_id: model_sha256.clone(),
        model_sha256,
        git_revision: current_git_revision()?,
    };
    let stop_requested = Arc::new(AtomicBool::new(false));
    {
        let stop_requested = stop_requested.clone();
        ctrlc::set_handler(move || {
            stop_requested.store(true, Ordering::SeqCst);
        })?;
    }

    let pool = {
        let builder = rayon::ThreadPoolBuilder::new().thread_name(|idx| format!("sim-{idx}"));
        let builder = if cli.threads == 0 {
            builder
        } else {
            builder.num_threads(cli.threads)
        };
        builder.build()?
    };

    if cli.fresh_db && cli.db.exists() {
        std::fs::remove_file(&cli.db)?;
    }

    let recorder = Arc::new(SimRecorder::new(&cli.db)?);

    let tuning = BalanceTuning {
        pool: &pool,
        cli: &cli,
        recorder: &recorder,
        stop_requested: &stop_requested,
        policy: &policy,
        seed_end,
    };
    let (tuned_config, iteration_reports) = tune_hp_balance(base_config, &tuning)?;

    if let Some(parent) = cli.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    tuned_config
        .write_jsonc(&cli.output)
        .map_err(anyhow::Error::msg)?;
    let report_path = cli.output.with_extension("balance-report.json");
    let final_contract = MlContract::from_config(&tuned_config);
    let report = serde_json::json!({
        "schema_version": 1,
        "policy_id": policy.policy_id,
        "checkpoint_path": policy.checkpoint_path,
        "checkpoint_sha256": policy.checkpoint_sha256,
        "model_sha256": policy.model_sha256,
        "git_revision": policy.git_revision,
        "checkpoint_iteration": policy.checkpoint_iteration,
        "target_full_clear_rate": target_act_clear_rates(tuned_config.player.max_stages)
            .iter()
            .product::<f32>(),
        "target_act_clear_rates": target_act_clear_rates(tuned_config.player.max_stages),
        "seed_schedule": format!("{}..={seed_end}", cli.seed_start),
        "source_config_digest": source_config_digest,
        "output_config_digest": final_contract.config_digest,
        "iterations": iteration_reports,
    });
    std::fs::write(
        &report_path,
        format!("{}\n", serde_json::to_string_pretty(&report)?),
    )?;
    println!("Saved tuned config to {}", cli.output.display());
    println!("Saved balance report to {}", report_path.display());

    Ok(())
}

fn target_cumulative_survival_for(acts_completed: usize) -> f32 {
    TARGET_ACT_CLEAR_RATES.iter().take(acts_completed).product()
}

fn target_act_clear_rate_for(act_index: usize) -> f32 {
    TARGET_ACT_CLEAR_RATES
        .get(act_index)
        .copied()
        .unwrap_or_default()
}

fn target_act_clear_rates(stages: usize) -> Vec<f32> {
    let act_count = stages.div_ceil(td_core::STAGES_PER_ACT);
    (0..act_count.min(TARGET_ACT_CLEAR_RATES.len()))
        .map(target_act_clear_rate_for)
        .collect()
}

fn compute_act_rates(
    stage_arrivals: &[usize],
    total_wins: usize,
    samples: usize,
    stages: usize,
) -> (Vec<f32>, Vec<f32>) {
    let act_count = stages.div_ceil(td_core::STAGES_PER_ACT);
    let mut clear_rates = Vec::with_capacity(act_count);
    let mut survival_rates = Vec::with_capacity(act_count);

    for act_index in 0..act_count {
        let entrants = if act_index == 0 {
            samples
        } else {
            stage_arrivals
                .get(act_index * td_core::STAGES_PER_ACT)
                .copied()
                .unwrap_or(0)
        };
        let clearers = if act_index + 1 == act_count {
            total_wins
        } else {
            stage_arrivals
                .get((act_index + 1) * td_core::STAGES_PER_ACT)
                .copied()
                .unwrap_or(0)
        };
        clear_rates.push(if entrants > 0 {
            clearers as f32 / entrants as f32
        } else {
            0.0
        });
        survival_rates.push(clearers as f32 / samples.max(1) as f32);
    }

    (clear_rates, survival_rates)
}

fn target_stage_clear_rate_for(stage: usize, stages: usize) -> f32 {
    let act_index = (stage - 1) / td_core::STAGES_PER_ACT;
    let act_start = act_index * td_core::STAGES_PER_ACT;
    let act_end = ((act_index + 1) * td_core::STAGES_PER_ACT).min(stages);
    let act_length = act_end.saturating_sub(act_start).max(1);
    let act_target = target_act_clear_rate_for(act_index);
    let base_stage_target = act_target.powf(1.0 / act_length as f32);
    let stage_position = stage - 1 - act_start;
    let centered_position = if act_length > 1 {
        0.5 - stage_position as f32 / (act_length - 1) as f32
    } else {
        0.0
    };
    let shape = STAGE_CLEAR_RATE_LOG_SHAPE.min(-2.0 * base_stage_target.ln());

    base_stage_target * (shape * centered_position).exp()
}

fn compute_stage_clear_rates(
    stage_arrivals: &[usize],
    total_wins: usize,
    stages: usize,
) -> Vec<f32> {
    (0..stages)
        .map(|index| {
            let entrants = stage_arrivals.get(index).copied().unwrap_or(0);
            let clearers = if index + 1 == stages {
                total_wins
            } else {
                stage_arrivals.get(index + 1).copied().unwrap_or(0)
            };
            if entrants > 0 {
                clearers as f32 / entrants as f32
            } else {
                0.0
            }
        })
        .collect()
}

fn act_rates_within_target(
    act_clear_rates: &[f32],
    stage_arrivals: &[usize],
    samples: usize,
    stages: usize,
) -> bool {
    let act_count = stages.div_ceil(td_core::STAGES_PER_ACT);
    let targets = target_act_clear_rates(stages);
    (0..act_count).all(|act_index| {
        let entrants = if act_index == 0 {
            samples
        } else {
            stage_arrivals
                .get(act_index * td_core::STAGES_PER_ACT)
                .copied()
                .unwrap_or(0)
        };
        let (Some(observed), Some(target)) =
            (act_clear_rates.get(act_index), targets.get(act_index))
        else {
            return false;
        };
        entrants >= MIN_CONVERGENCE_ARRIVALS
            && (*observed - *target).abs() <= ACT_RATE_CONVERGENCE_TOLERANCE
    })
}

fn compute_act_stage_clear_rmses(
    stage_arrivals: &[usize],
    stage_clear_rates: &[f32],
    stages: usize,
) -> Vec<Option<f32>> {
    let act_count = stages.div_ceil(td_core::STAGES_PER_ACT);
    (0..act_count)
        .map(|act_index| {
            let act_start = act_index * td_core::STAGES_PER_ACT;
            let act_end = ((act_index + 1) * td_core::STAGES_PER_ACT).min(stages);
            let mut squared_error_sum = 0.0;
            let mut stage_count = 0;
            for stage_index in act_start..act_end {
                let entrants = stage_arrivals.get(stage_index).copied().unwrap_or(0);
                if entrants < MIN_CONVERGENCE_ARRIVALS {
                    continue;
                }
                let observed = stage_clear_rates.get(stage_index).copied().unwrap_or(0.0);
                let target = target_stage_clear_rate_for(stage_index + 1, stages);
                squared_error_sum += (observed - target).powi(2);
                stage_count += 1;
            }

            (stage_count > 0).then(|| (squared_error_sum / stage_count as f32).sqrt())
        })
        .collect()
}

fn act_stage_clear_rmses_within_target(act_stage_clear_rmses: &[Option<f32>]) -> bool {
    !act_stage_clear_rmses.is_empty()
        && act_stage_clear_rmses
            .iter()
            .all(|rmse| rmse.is_some_and(|rmse| rmse < STAGE_CLEAR_RMSE_CONVERGENCE_THRESHOLD))
}

fn sample_confidence(sample_count: usize) -> f32 {
    (sample_count as f32 / (sample_count as f32 + MIN_CONVERGENCE_ARRIVALS as f32)).sqrt()
}

fn smoothed_clear_rate(observed_rate: f32, sample_count: usize) -> f32 {
    (observed_rate * sample_count as f32 + 0.5) / (sample_count as f32 + 1.0)
}

fn compute_act_stage_adjustments(
    stage_arrivals: &[usize],
    stage_clear_rates: &[f32],
    act_clear_rates: &[f32],
    samples: usize,
    stages: usize,
) -> Vec<f32> {
    let act_count = stages.div_ceil(td_core::STAGES_PER_ACT);
    let mut adjustments = vec![0.0; stages];

    for act_index in 0..act_count {
        let act_start = act_index * td_core::STAGES_PER_ACT;
        let act_end = ((act_index + 1) * td_core::STAGES_PER_ACT).min(stages);
        let act_length = act_end.saturating_sub(act_start).max(1);
        let act_entrants = if act_index == 0 {
            samples
        } else {
            stage_arrivals.get(act_start).copied().unwrap_or(0)
        };
        if act_entrants == 0 {
            continue;
        }

        let target_act_clear_rate = target_act_clear_rate_for(act_index);
        let observed_act_clear_rate = act_clear_rates.get(act_index).copied().unwrap_or(0.0);
        let act_log_error = (smoothed_clear_rate(observed_act_clear_rate, act_entrants)
            / target_act_clear_rate)
            .ln()
            .clamp(-ACT_LOG_ERROR_CLAMP, ACT_LOG_ERROR_CLAMP);
        let act_confidence = sample_confidence(act_entrants);
        let shared_act_adjustment = act_log_error * act_confidence / act_length as f32;
        let mut stage_residuals = Vec::with_capacity(act_length);
        let mut observed_stage_count = 0;

        for stage_index in act_start..act_end {
            let stage_entrants = stage_arrivals.get(stage_index).copied().unwrap_or(0);
            let stage_residual = if stage_entrants > 0 {
                observed_stage_count += 1;
                let observed_stage_clear_rate =
                    stage_clear_rates.get(stage_index).copied().unwrap_or(0.0);
                let stage_log_error =
                    (smoothed_clear_rate(observed_stage_clear_rate, stage_entrants)
                        / target_stage_clear_rate_for(stage_index + 1, stages))
                    .ln();
                let stage_confidence = sample_confidence(stage_entrants);
                (stage_log_error - act_log_error / act_length as f32)
                    .clamp(-STAGE_LOG_ERROR_CLAMP, STAGE_LOG_ERROR_CLAMP)
                    * stage_confidence
            } else {
                0.0
            };
            stage_residuals.push(stage_residual);
        }

        let smoothed_residuals = smooth_act_stage_residuals(&stage_residuals);
        let residual_mean = if observed_stage_count > 0 {
            smoothed_residuals
                .iter()
                .enumerate()
                .filter(|(offset, _)| {
                    stage_arrivals.get(act_start + offset).copied().unwrap_or(0) > 0
                })
                .map(|(_, residual)| residual)
                .sum::<f32>()
                / observed_stage_count as f32
        } else {
            0.0
        };
        for (offset, smoothed_residual) in smoothed_residuals.into_iter().enumerate() {
            let has_stage_samples =
                stage_arrivals.get(act_start + offset).copied().unwrap_or(0) > 0;
            let centered_residual = if has_stage_samples {
                (smoothed_residual - residual_mean)
                    .clamp(-STAGE_RESIDUAL_CLAMP, STAGE_RESIDUAL_CLAMP)
            } else {
                0.0
            };
            adjustments[act_start + offset] =
                shared_act_adjustment + centered_residual * ACT_CLEAR_SIGNAL_WEIGHT;
        }
    }

    adjustments
}

fn smooth_act_stage_residuals(stage_residuals: &[f32]) -> Vec<f32> {
    stage_residuals
        .iter()
        .enumerate()
        .map(|(index, residual)| {
            let mut weighted_sum = *residual;
            let mut total_weight = 1.0;
            if let Some(previous) = index.checked_sub(1).and_then(|i| stage_residuals.get(i)) {
                weighted_sum += previous * INTRA_ACT_SIGNAL_SMOOTHING;
                total_weight += INTRA_ACT_SIGNAL_SMOOTHING;
            }
            if let Some(next) = stage_residuals.get(index + 1) {
                weighted_sum += next * INTRA_ACT_SIGNAL_SMOOTHING;
                total_weight += INTRA_ACT_SIGNAL_SMOOTHING;
            }
            weighted_sum / total_weight
        })
        .collect()
}

fn update_stage_scale(current_scale: f32, signal: f32, learning_rate: f32) -> f32 {
    let log_step =
        (signal * learning_rate).clamp(-STAGE_SCALE_LOG_STEP_MAX, STAGE_SCALE_LOG_STEP_MAX);
    let target_scale = (current_scale * log_step.exp()).clamp(
        (current_scale * STAGE_SCALE_RELATIVE_MIN).max(STAGE_SCALE_CLAMP_MIN_FLOOR),
        (current_scale * STAGE_SCALE_RELATIVE_MAX).min(STAGE_SCALE_CLAMP_MAX_CEILING),
    );
    (current_scale * (1.0 - BLEND_ALPHA) + target_scale * BLEND_ALPHA)
        .clamp(STAGE_SCALE_CLAMP_MIN_FLOOR, STAGE_SCALE_CLAMP_MAX_CEILING)
}

fn tune_hp_balance(
    mut config: GameConfig,
    tuning: &BalanceTuning<'_>,
) -> anyhow::Result<(GameConfig, Vec<serde_json::Value>)> {
    let BalanceTuning {
        cli,
        stop_requested,
        policy,
        seed_end,
        ..
    } = tuning;
    let stages = config.player.max_stages;
    let original_hp: HashMap<u8, f32> = config
        .monsters
        .stats
        .iter()
        .map(|stat| (stat.kind, stat.base_hp_raw as f32 / 1_000.0))
        .collect();
    let mut stage_primary = build_stage_primary_monster(&config);
    let mut stage_momentum = vec![1.0_f32; stages + 1];
    let mut stable_iterations = 0;

    println!(
        "=== Running policy-backed auto tuning: samples={} policy={} seeds={}..={} ===",
        cli.samples, policy.policy_id, cli.seed_start, seed_end,
    );
    let mut iteration_reports = Vec::new();

    for iteration in 1.. {
        if stop_requested.load(Ordering::SeqCst) {
            println!("Ctrl-C received before iteration {iteration}, finalizing current config...");
            break;
        }
        if let Some(max_iterations) = cli.max_iterations
            && iteration > max_iterations
        {
            println!("Reached max_iterations={max_iterations}, stopping auto balance.");
            break;
        }
        let metrics = run_simulations(
            tuning,
            Arc::new(config.clone()),
            cli.samples,
            cli.seed_start,
            iteration,
            cli.quiet,
        )?;
        if metrics.is_none() {
            println!("Ctrl-C received during simulation batch, finalizing last complete config...");
            break;
        }
        let metrics = metrics.unwrap();

        let hp_before = config
            .monsters
            .stats
            .iter()
            .map(|stat| (stat.kind, stat.base_hp_raw as f32 / 1_000.0))
            .collect::<BTreeMap<_, _>>();
        let overall_mean = metrics.overall_damage_mean;
        let target_message = format!(
            "win {:.1}%, below15 {:.1}%, mean {:.1}, termination {:?}, stage1 {:?}, config_digest={}, policy={}",
            metrics.win_rate * 100.0,
            metrics.below_15_frac * 100.0,
            overall_mean,
            metrics.termination_counts,
            metrics.stage_one_termination_counts,
            metrics.config_digest,
            metrics.policy_id,
        );
        println!("Iteration {iteration}: {target_message}");
        for (act_index, (&clear_rate, &survival_rate)) in metrics
            .act_clear_rates
            .iter()
            .zip(&metrics.act_survival_rates)
            .enumerate()
        {
            let act = act_index + 1;
            let target_clear_rate = target_act_clear_rate_for(act_index);
            let target_survival_rate = target_cumulative_survival_for(act);
            println!(
                "  Act {act}: clear {:.1}% (target {:.1}%), cumulative survival {:.1}% (target {:.1}%)",
                clear_rate * 100.0,
                target_clear_rate * 100.0,
                survival_rate * 100.0,
                target_survival_rate * 100.0,
            );
        }
        println!(
            "Iteration {iteration}: terminal metrics towers {:.2}, player_damage {:.2}, escaped_hp {:.2}, items {:.2}, tower_damage {:.2}",
            metrics.terminal_tower_mean,
            metrics.terminal_player_damage_mean,
            metrics.terminal_escaped_hp_mean,
            metrics.terminal_item_use_mean,
            metrics.terminal_tower_damage_mean,
        );
        if !metrics.stage_one_cycle_action_pairs.is_empty() {
            println!(
                "Iteration {iteration}: stage1 cycle pairs {:?}",
                metrics.stage_one_cycle_action_pairs
            );
        }

        let target_act_rates = target_act_clear_rates(stages);
        let max_act_deviation = metrics
            .act_clear_rates
            .iter()
            .zip(&target_act_rates)
            .map(|(observed, target)| (observed - target).abs())
            .fold(0.0_f32, f32::max);
        let act_stage_clear_rmses = compute_act_stage_clear_rmses(
            &metrics.stage_arrivals,
            &metrics.stage_clear_rates,
            stages,
        );
        let iteration_learning_rate = BASE_LEARNING_RATE;
        let stage_adjustments = compute_act_stage_adjustments(
            &metrics.stage_arrivals,
            &metrics.stage_clear_rates,
            &metrics.act_clear_rates,
            cli.samples,
            stages,
        );

        let mut max_change: f32 = 0.0;
        let mut stage_scale_min: f32 = f32::INFINITY;
        let mut stage_scale_max: f32 = f32::NEG_INFINITY;

        let mut smoothed_stage_scale = vec![1.0_f32; stages + 1];
        for (stage, primary_monster) in stage_primary.iter().enumerate().skip(1).take(stages) {
            if primary_monster.is_some() {
                let stage_scale = update_stage_scale(
                    stage_momentum[stage],
                    stage_adjustments[stage - 1],
                    iteration_learning_rate,
                );
                smoothed_stage_scale[stage] = stage_scale;
                stage_scale_min = stage_scale_min.min(stage_scale);
                stage_scale_max = stage_scale_max.max(stage_scale);
            }
        }
        stage_momentum = smoothed_stage_scale.clone();

        let mut kind_log_scale_sum: HashMap<u8, f32> = HashMap::new();
        let mut kind_weight_sum: HashMap<u8, f32> = HashMap::new();
        for stage in 1..=stages {
            if let Some(monster_kind) = stage_primary[stage] {
                let stage_scale = smoothed_stage_scale[stage].max(0.01);
                *kind_log_scale_sum.entry(monster_kind).or_insert(0.0) += stage_scale.ln();
                *kind_weight_sum.entry(monster_kind).or_insert(0.0) += 1.0;
            }
        }

        for (monster_kind, log_sum) in kind_log_scale_sum {
            if let Some(stat) = config
                .monsters
                .stats
                .iter_mut()
                .find(|stat| stat.kind == monster_kind)
            {
                let weight = kind_weight_sum
                    .get(&monster_kind)
                    .copied()
                    .unwrap_or(1.0)
                    .max(1.0);
                let kind_scale = (log_sum / weight).exp();
                let base_hp = original_hp
                    .get(&monster_kind)
                    .copied()
                    .unwrap_or(stat.base_hp_raw as f32 / 1_000.0);
                let applied_scale = kind_scale;
                stat.base_hp_raw =
                    ((base_hp * applied_scale).max(base_hp * 0.25) * 1_000.0).round() as i64;
                max_change = max_change.max((applied_scale - 1.0).abs());
            }
        }

        apply_boss_hp_from_stage(&mut config);

        println!(
            "  debug: max_act_error={:.4} act_stage_clear_rmse={:?} learn_rate={:.2} max_scale_offset={:.6} stage_scale=[{:.4},{:.4}] stable={}/{}",
            max_act_deviation,
            act_stage_clear_rmses,
            iteration_learning_rate,
            max_change,
            stage_scale_min,
            stage_scale_max,
            stable_iterations,
            STABLE_ITERATIONS_REQUIRED,
        );
        if !metrics.clear_rates.is_empty() {
            println!("=== Clear Rate Distribution ===");
            print_clear_rate_histogram(&metrics.clear_rates);
        }
        let mut clear_rate_histogram = vec![0usize; CLEAR_RATE_BIN_COUNT];
        for &rate in &metrics.clear_rates {
            let index = ((rate.clamp(0.0, 100.0) as usize) / 2).min(CLEAR_RATE_BIN_COUNT - 1);
            clear_rate_histogram[index] += 1;
        }
        let hp_deltas = config
            .monsters
            .stats
            .iter()
            .map(|stat| {
                let before = hp_before.get(&stat.kind).copied().unwrap_or(0.0);
                let after = stat.base_hp_raw as f32 / 1_000.0;
                (
                    stat.kind.to_string(),
                    serde_json::json!({
                        "before": before,
                        "after": after,
                        "delta": after - before,
                    }),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let act_progress = (0..metrics.act_clear_rates.len())
            .map(|act_index| {
                let act = act_index + 1;
                serde_json::json!({
                    "act": act,
                    "target_clear_rate": target_act_clear_rate_for(act_index),
                    "observed_clear_rate": metrics.act_clear_rates[act_index],
                    "target_cumulative_survival": target_cumulative_survival_for(act),
                    "observed_cumulative_survival": metrics.act_survival_rates[act_index],
                })
            })
            .collect::<Vec<_>>();
        let stage_progress = (0..stages)
            .map(|stage_index| {
                serde_json::json!({
                    "stage": stage_index + 1,
                    "entrants": metrics.stage_arrivals[stage_index],
                    "target_clear_rate": target_stage_clear_rate_for(stage_index + 1, stages),
                    "observed_clear_rate": metrics.stage_clear_rates[stage_index],
                })
            })
            .collect::<Vec<_>>();
        iteration_reports.push(serde_json::json!({
            "iteration": iteration,
            "policy_id": policy.policy_id,
            "checkpoint_path": policy.checkpoint_path,
            "checkpoint_sha256": policy.checkpoint_sha256,
            "seed_schedule": format!("{}..={seed_end}", cli.seed_start),
            "config_digest": metrics.config_digest,
            "clear_rate_histogram_2_percent_bins": clear_rate_histogram,
            "act_progress": act_progress,
            "stage_progress": stage_progress,
            "act_stage_clear_rate_rmse": act_stage_clear_rmses,
            "max_hp_scale_change": max_change,
            "hp_deltas": hp_deltas,
        }));

        let act_rates_stable = act_rates_within_target(
            &metrics.act_clear_rates,
            &metrics.stage_arrivals,
            cli.samples,
            stages,
        );
        let stage_clear_stable = act_stage_clear_rmses_within_target(&act_stage_clear_rmses);

        if act_rates_stable && stage_clear_stable {
            stable_iterations += 1;
        } else {
            stable_iterations = 0;
        }

        if stable_iterations >= STABLE_ITERATIONS_REQUIRED {
            println!(
                "Iteration {iteration} reached stable metrics after {stable_iterations} iterations"
            );
            break;
        }

        if stop_requested.load(Ordering::SeqCst) {
            println!("Ctrl-C received after iteration {iteration}, finalizing current config...");
            break;
        }

        stage_primary = build_stage_primary_monster(&config);
    }

    apply_boss_hp_from_stage(&mut config);
    Ok((config, iteration_reports))
}

fn normalize_normal_monster_hp(config: &mut GameConfig, hp: f32) {
    for stat in config.monsters.stats.iter_mut() {
        if is_normal_monster(stat.kind) {
            stat.base_hp_raw = (hp * 1_000.0).round() as i64;
        }
    }
}

fn build_stage_primary_monster(config: &GameConfig) -> Vec<Option<u8>> {
    let max_stage = config.player.max_stages;
    let mut stage_primary = vec![None; max_stage + 1];

    for stage_wave in &config.monsters.stage_waves {
        if stage_wave.stage > max_stage {
            continue;
        }

        stage_primary[stage_wave.stage] = stage_wave
            .entries
            .iter()
            .find(|entry| is_normal_monster(entry.kind))
            .map(|entry| entry.kind);
    }

    stage_primary
}

fn apply_boss_hp_from_stage(config: &mut GameConfig) {
    let stage_waves = config.monsters.stage_waves.clone();
    for stage_wave in &stage_waves {
        let normal_kind = stage_wave
            .entries
            .iter()
            .find(|entry| is_normal_monster(entry.kind))
            .map(|entry| entry.kind);

        let boss_kind = stage_wave
            .entries
            .iter()
            .find(|entry| !is_normal_monster(entry.kind))
            .map(|entry| entry.kind);

        if let (Some(normal_kind), Some(boss_kind)) = (normal_kind, boss_kind)
            && let Some(normal_hp) = config
                .monsters
                .stats
                .iter()
                .find(|stat| stat.kind == normal_kind)
                .map(|stat| stat.base_hp_raw as f32 / 1_000.0)
            && let Some(boss_stat) = config
                .monsters
                .stats
                .iter_mut()
                .find(|stat| stat.kind == boss_kind)
        {
            boss_stat.base_hp_raw = (normal_hp * 3.0 * 1_000.0).round() as i64;
        }
    }
}

fn is_normal_monster(kind: u8) -> bool {
    kind < 50
}

fn run_simulations(
    tuning: &BalanceTuning<'_>,
    config: Arc<GameConfig>,
    samples: usize,
    seed_start: u64,
    iteration: usize,
    quiet: bool,
) -> anyhow::Result<Option<BalanceStats>> {
    let BalanceTuning {
        pool,
        recorder,
        stop_requested,
        policy,
        ..
    } = tuning;
    if stop_requested.load(Ordering::SeqCst) {
        return Ok(None);
    }

    let stage_count = config.player.max_stages;
    let seed_end = seed_start
        .checked_add(u64::try_from(samples)? - 1)
        .context("balance seed range overflow")?;
    let seeds = (seed_start..=seed_end).collect::<Vec<_>>();
    let progress = if quiet {
        None
    } else {
        let progress = ProgressBar::new(samples as u64);
        let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )?;
        progress.set_style(style);
        Some(progress)
    };
    let runner = PolicyRunnerConfig {
        record_steps: samples <= 100,
        ..PolicyRunnerConfig::default()
    };
    let episodes = pool.install(|| {
        seeds
            .par_iter()
            .map(|seed| {
                crate::simulation::run_semantic_statistics(
                    Arc::clone(&config),
                    *seed,
                    &runner,
                    &policy.policy,
                )
                .map(|recorded| recorded.episode)
            })
            .collect::<anyhow::Result<Vec<_>>>()
    })?;
    let batch = BatchResult { seeds, episodes };
    if let Some(progress) = progress {
        progress.set_position(samples as u64);
        progress.finish_and_clear();
    }
    if stop_requested.load(Ordering::SeqCst) {
        return Ok(None);
    }

    let mut stage_arrivals = vec![0_usize; stage_count];
    let mut clear_rates = Vec::with_capacity(batch.episodes.len());
    let mut total_damage = 0.0_f32;
    let mut total_visits = 0_usize;
    let mut total_wins = 0_usize;
    let mut below_15_count = 0_usize;
    let mut termination_counts = BTreeMap::new();
    let mut stage_one_termination_counts = BTreeMap::new();
    let mut stage_one_cycle_action_pairs = BTreeMap::new();
    let mut terminal_tower_total = 0_usize;
    let mut terminal_player_damage_total = 0.0_f32;
    let mut terminal_escaped_hp_total = 0.0_f32;
    let mut terminal_item_use_total = 0_usize;
    let mut terminal_tower_damage_total = 0.0_f32;
    let mut terminal_count = 0_usize;
    let contract = MlContract::from_config(config.as_ref());
    let provenance = SimulationProvenance {
        runner_kind: "policy_runner".to_string(),
        policy_kind: "semantic_policy".to_string(),
        checkpoint_path: Some(policy.checkpoint_path.clone()),
        checkpoint_iteration: Some(policy.checkpoint_iteration),
        checkpoint_sha256: Some(policy.checkpoint_sha256.clone()),
        model_sha256: Some(policy.model_sha256.clone()),
        git_revision: Some(policy.git_revision.clone()),
        environment_version: Some(contract.environment_version),
        action_schema_version: Some(contract.action_schema_version),
        config_digest: Some(contract.config_digest.clone()),
        config_override: false,
        seed_schedule: Some(format!("{seed_start}..={seed_end}")),
    };

    for episode in &batch.episodes {
        let sim_id = format!("iteration_{iteration}_sim_{:016x}", episode.seed);
        recorder.record_simulation_start_with_provenance(
            &sim_id,
            "environment_policy",
            "environment_policy",
            "environment_policy",
            "environment_policy",
            "environment_policy",
            episode.seed,
            &provenance,
        )?;

        let episode_damage = episode
            .metrics
            .stage_damage
            .iter()
            .map(|&(stage, damage)| {
                if stage > 0 && stage <= stage_count {
                    total_damage += damage;
                    total_visits += 1;
                }
                damage
            })
            .sum::<f32>();
        for arrival in stage_arrivals
            .iter_mut()
            .take(episode.final_observation.stage.min(stage_count))
        {
            *arrival += 1;
        }
        recorder.record_simulation_end(
            &sim_id,
            episode.victory,
            episode.final_observation.stage,
            episode.clear_rate,
            episode.final_observation.hp_raw as f32 / 1_000.0,
            episode.final_observation.gold,
            episode.metrics.total_towers_placed,
            episode.metrics.total_items_used,
            episode_damage,
            episode.metrics.total_gold_earned,
        )?;

        clear_rates.push(episode.clear_rate);
        total_wins += usize::from(episode.victory);
        below_15_count += usize::from(episode.final_observation.stage < 15);
        let reason = termination_reason_name(&episode.termination_reason);
        *termination_counts.entry(reason.to_string()).or_insert(0) += 1;
        if episode.final_observation.stage == 1 {
            *stage_one_termination_counts
                .entry(reason.to_string())
                .or_insert(0) += 1;
            if episode.termination_reason == StepReason::NoProgressCycle
                && let Some(steps) = &episode.steps
                && steps.len() >= 2
            {
                let previous = &steps[steps.len() - 2];
                let last = &steps[steps.len() - 1];
                let pair = format!(
                    "{:?}:{} -> {:?}:{}",
                    previous.observation.decision_point,
                    previous.action.action_id(),
                    last.observation.decision_point,
                    last.action.action_id()
                );
                *stage_one_cycle_action_pairs.entry(pair).or_insert(0) += 1;
            }
        }
        if episode.terminated {
            terminal_count += 1;
            terminal_tower_total += episode.metrics.total_towers_placed;
            terminal_player_damage_total += episode.metrics.total_player_damage;
            terminal_escaped_hp_total += episode.metrics.total_escaped_hp;
            terminal_item_use_total += episode.metrics.total_items_used;
            terminal_tower_damage_total += episode.metrics.total_tower_damage;
        }
    }

    let (act_clear_rates, act_survival_rates) =
        compute_act_rates(&stage_arrivals, total_wins, samples, stage_count);
    let stage_clear_rates = compute_stage_clear_rates(&stage_arrivals, total_wins, stage_count);

    Ok(Some(BalanceStats {
        win_rate: total_wins as f32 / samples.max(1) as f32,
        stage_arrivals,
        stage_clear_rates,
        act_clear_rates,
        act_survival_rates,
        clear_rates,
        overall_damage_mean: if total_visits > 0 {
            total_damage / total_visits as f32
        } else {
            0.0
        },
        below_15_frac: below_15_count as f32 / samples.max(1) as f32,
        termination_counts,
        stage_one_termination_counts,
        stage_one_cycle_action_pairs,
        terminal_tower_mean: terminal_tower_total as f32 / terminal_count.max(1) as f32,
        terminal_player_damage_mean: terminal_player_damage_total / terminal_count.max(1) as f32,
        terminal_escaped_hp_mean: terminal_escaped_hp_total / terminal_count.max(1) as f32,
        terminal_item_use_mean: terminal_item_use_total as f32 / terminal_count.max(1) as f32,
        terminal_tower_damage_mean: terminal_tower_damage_total / terminal_count.max(1) as f32,
        config_digest: contract.config_digest,
        policy_id: policy.policy_id.clone(),
    }))
}

fn termination_reason_name(reason: &StepReason) -> &'static str {
    match reason {
        StepReason::DecisionPoint => "DecisionPoint",
        StepReason::Terminal => "Terminal",
        StepReason::MaxTicks => "MaxTicks",
        StepReason::MaxDecisions => "MaxDecisions",
        StepReason::NoProgressCycle => "NoProgressCycle",
        StepReason::CurriculumComplete => "CurriculumComplete",
    }
}

fn print_clear_rate_histogram(clear_rates: &[f32]) {
    let mut bins = vec![0usize; 51];
    for &rate in clear_rates {
        let idx = ((rate.clamp(0.0, 100.0) as usize) / 2).min(50);
        bins[idx] += 1;
    }

    let max_count = bins.iter().copied().max().unwrap_or(1).max(1);
    let bar_width = 20;

    for (idx, &count) in bins.iter().enumerate() {
        let label = format!("{:02}", idx + 1);
        let bar_len = (count * bar_width + max_count / 2) / max_count;
        let bar = "█".repeat(bar_len);
        let padded_bar = format!("{:<width$}", bar, width = bar_width);
        println!("{label} | {padded_bar} {count}");
    }
}

#[cfg(test)]
mod act_clear_rate_tests {
    use super::*;

    #[test]
    fn act_targets_match_independent_clear_and_cumulative_survival_rates() {
        let target_clear_rates = target_act_clear_rates(50);
        let expected_clear_rates = [0.823, 0.734, 0.611, 0.456, 0.297];
        let expected_survival_rates = [0.823, 0.604082, 0.3690961, 0.16830782, 0.05];

        for (index, expected) in expected_clear_rates.iter().enumerate() {
            assert!((target_clear_rates[index] - expected).abs() < 0.001);
            let survival = target_cumulative_survival_for(index + 1);
            assert!((survival - expected_survival_rates[index]).abs() < 0.001);
        }
        assert_eq!(target_cumulative_survival_for(0), 1.0);
    }

    #[test]
    fn observed_act_rates_use_entries_and_clears_at_act_boundaries() {
        let mut stage_arrivals = vec![100; 50];
        stage_arrivals[10] = 82;
        stage_arrivals[20] = 60;
        stage_arrivals[30] = 37;
        stage_arrivals[40] = 17;

        let (clear_rates, survival_rates) = compute_act_rates(&stage_arrivals, 5, 100, 50);

        assert!((clear_rates[0] - 0.82).abs() < 0.001);
        assert!((clear_rates[1] - 60.0 / 82.0).abs() < 0.001);
        assert!((clear_rates[4] - 5.0 / 17.0).abs() < 0.001);
        assert_eq!(survival_rates, [0.82, 0.6, 0.37, 0.17, 0.05]);
    }

    #[test]
    fn stage_adjustments_follow_act_targets_and_stage_residuals() {
        let stages = 50;
        let target_act_rates = target_act_clear_rates(stages);
        let target_stage_rates = (1..=stages)
            .map(|stage| target_stage_clear_rate_for(stage, stages))
            .collect::<Vec<_>>();
        let stage_arrivals = vec![1_000; stages];

        let mut observed_act_rates = target_act_rates.clone();
        observed_act_rates[0] = 0.95;
        let mut observed_stage_rates = target_stage_rates.clone();
        let early_act_stage_scale = (observed_act_rates[0] / target_act_rates[0])
            .powf(1.0 / td_core::STAGES_PER_ACT as f32);
        for stage_rate in &mut observed_stage_rates[..td_core::STAGES_PER_ACT] {
            *stage_rate *= early_act_stage_scale;
        }
        let act_adjustments = compute_act_stage_adjustments(
            &stage_arrivals,
            &observed_stage_rates,
            &observed_act_rates,
            1_000,
            stages,
        );

        assert!(act_adjustments[0] > 0.0);
        assert!(act_adjustments[td_core::STAGES_PER_ACT] < act_adjustments[0]);

        let mut underperforming_act_rates = target_act_rates.clone();
        underperforming_act_rates[3] = 0.239;
        underperforming_act_rates[4] = 0.0;
        let underperforming_adjustments = compute_act_stage_adjustments(
            &stage_arrivals,
            &target_stage_rates,
            &underperforming_act_rates,
            1_000,
            stages,
        );
        assert!(underperforming_adjustments[30] < 0.0);
        assert!(underperforming_adjustments[40] < 0.0);

        let mut stage_outlier_rates = target_stage_rates;
        stage_outlier_rates[4] = 0.90;
        let stage_adjustments = compute_act_stage_adjustments(
            &stage_arrivals,
            &stage_outlier_rates,
            &target_act_rates,
            1_000,
            stages,
        );
        assert!(stage_adjustments[4] < stage_adjustments[3]);
        assert!(stage_adjustments[4] < 0.0);
    }

    #[test]
    fn stage_targets_decrease_within_each_act_without_changing_act_target() {
        let stages = 50_usize;

        for act_index in 0..stages.div_ceil(td_core::STAGES_PER_ACT) {
            let act_start = act_index * td_core::STAGES_PER_ACT;
            let act_end = ((act_index + 1) * td_core::STAGES_PER_ACT).min(stages);
            let targets = (act_start + 1..=act_end)
                .map(|stage| target_stage_clear_rate_for(stage, stages))
                .collect::<Vec<_>>();

            assert!(targets.windows(2).all(|pair| pair[0] > pair[1]));
            assert!(targets.iter().all(|target| *target <= 1.0));
            let act_target = targets.iter().product::<f32>();
            assert!((act_target - target_act_clear_rate_for(act_index)).abs() < 1e-5);
        }
    }

    #[test]
    fn stage_adjustments_make_early_stages_easier_and_late_stages_harder() {
        let stages = 50;
        let target_act_rates = target_act_clear_rates(stages);
        let stage_arrivals = vec![1_000; stages];
        let uniform_stage_rates = (1..=stages)
            .map(|stage| {
                let act_index = (stage - 1) / td_core::STAGES_PER_ACT;
                let act_start = act_index * td_core::STAGES_PER_ACT;
                let act_end = ((act_index + 1) * td_core::STAGES_PER_ACT).min(stages);
                target_act_rates[act_index].powf(1.0 / (act_end - act_start) as f32)
            })
            .collect::<Vec<_>>();

        let adjustments = compute_act_stage_adjustments(
            &stage_arrivals,
            &uniform_stage_rates,
            &target_act_rates,
            1_000,
            stages,
        );

        for act_index in 0..stages.div_ceil(td_core::STAGES_PER_ACT) {
            let act_start = act_index * td_core::STAGES_PER_ACT;
            let act_end = ((act_index + 1) * td_core::STAGES_PER_ACT).min(stages);
            assert!(adjustments[act_start] < 0.0);
            assert!(adjustments[act_end - 1] > 0.0);
        }
    }

    #[test]
    fn convergence_requires_supported_act_and_stage_samples() {
        let stages = 50;
        let target_act_rates = target_act_clear_rates(stages);
        let mut stage_arrivals = vec![100; stages];
        let stage_clear_rates = (1..=stages)
            .map(|stage| target_stage_clear_rate_for(stage, stages))
            .collect::<Vec<_>>();

        assert!(act_rates_within_target(
            &target_act_rates,
            &stage_arrivals,
            100,
            stages,
        ));
        let act_stage_clear_rmses =
            compute_act_stage_clear_rmses(&stage_arrivals, &stage_clear_rates, stages);
        assert!(act_stage_clear_rmses_within_target(&act_stage_clear_rmses));

        stage_arrivals[40..].fill(MIN_CONVERGENCE_ARRIVALS - 1);
        assert!(!act_rates_within_target(
            &target_act_rates,
            &stage_arrivals,
            100,
            stages,
        ));
        let unsupported_stage_rmses =
            compute_act_stage_clear_rmses(&stage_arrivals, &stage_clear_rates, stages);
        assert!(!act_stage_clear_rmses_within_target(
            &unsupported_stage_rmses
        ));
    }

    #[test]
    fn stage_scale_updates_accumulate_relative_to_current_scale() {
        let first = update_stage_scale(1.0, 0.1, BASE_LEARNING_RATE);
        let second = update_stage_scale(first, 0.1, BASE_LEARNING_RATE);

        assert!(first > 1.0);
        assert!(second > first);
        assert!(update_stage_scale(second, -0.1, BASE_LEARNING_RATE) < second);
    }
}
