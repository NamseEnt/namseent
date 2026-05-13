use clap::{Parser, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use tower_defense::simulator::HeadlessGame;
use tower_defense::simulator::recording::SimRecorder;
use tower_defense::simulator::strategies::TowerPlacementStrategy;
use tower_defense::simulator::strategies::treasure::SynergyTreasureStrategy;
use tower_defense::simulator::strategies::{
    card_reroll::ItemAwareRerollStrategy, item_use::HeuristicItemUseStrategy,
    shop::SynergyShopStrategy, tower_placement::HeuristicPlacementStrategy,
};
use tower_defense::{MonsterKind, config::GameConfig, set_headless};

const TARGET_WIN_RATE: f32 = 0.05;
const MIN_ZERO_DAMAGE_SCALE: f32 = 0.05;
const BASE_LEARNING_RATE: f32 = 0.40;
const LEARNING_RATE_DECAY: f32 = 0.97;
const LEARNING_RATE_MAX: f32 = 2.00;
const STABLE_ITERATIONS_REQUIRED: usize = 3;
const STAGE_SCALE_CLAMP_MIN: f32 = 0.40;
const STAGE_SCALE_CLAMP_MAX: f32 = 4.00;
const STAGE_SCALE_CLAMP_MIN_FLOOR: f32 = 0.25;
const STAGE_SCALE_CLAMP_MAX_CEILING: f32 = 8.00;
const BLEND_ALPHA_BASE: f32 = 0.30;
const BLEND_ALPHA_MAX: f32 = 0.65;
const BLEND_ALPHA_WIN_DEFICIT_GAIN: f32 = 2.5;
const WIN_DEFICIT_CLAMP_GAIN: f32 = 1.20;
const WIN_SURPLUS_CLAMP_GAIN: f32 = 1.60;
const TARGET_STAGE_SURVIVAL_EXPONENT: f32 = 1.0;
const ARRIVAL_ERROR_WEIGHT_BASE: f32 = 0.65;
const DAMAGE_ERROR_WEIGHT_BASE: f32 = 0.20;
const DAMAGE_ERROR_WEIGHT_MAX: f32 = 0.45;
const DAMAGE_CV_WEIGHT_BOOST_THRESHOLD: f32 = 0.35;
const DAMAGE_CV_STABLE_THRESHOLD: f32 = 0.35;
const CLEAR_PEAK_STABLE_THRESHOLD: f32 = 1.30;
const GLOBAL_WIN_SCALE_GAIN: f32 = 5.0;
const GLOBAL_WIN_SCALE_MIN: f32 = 0.60;
const GLOBAL_WIN_SCALE_MAX: f32 = 1.30;
const GLOBAL_WIN_SCALE_BLEND: f32 = 0.05;
const CLEAR_RATE_BIN_COUNT: usize = 51;
const CDF_GLOBAL_GAIN: f32 = 0.30;
const CDF_GLOBAL_SMOOTHING: f32 = 0.65;
const GLOBAL_SCALE_MIN: f32 = 0.60;
const GLOBAL_SCALE_MAX: f32 = 4.00;
const KS_CONVERGENCE_THRESHOLD: f32 = 0.12;
const TOP_BIN_CONVERGENCE_THRESHOLD: f32 = 0.12;
const DEFAULT_DISTRIBUTION_STAGE_SIGNAL_GAIN: f32 = 2.50;
const DEFAULT_DEATH_DISTRIBUTION_SIGNAL_WEIGHT: f32 = 1.50;
const DEATH_DISTRIBUTION_SIGNAL_CLAMP: f32 = 0.60;
const ROUGH_BOOTSTRAP_SIGNAL_CLAMP: f32 = 0.60;
const ROUGH_TILT_MIN: f32 = -0.50;
const ROUGH_TILT_MAX: f32 = 4.00;
const ROUGH_GLOBAL_SCALE_MIN: f32 = 0.01;
const ROUGH_GLOBAL_SCALE_MAX: f32 = 1_000_000.0;
const ROUGH_STAGE_SCALE_MIN_FLOOR: f32 = 0.05;
const ROUGH_STAGE_SCALE_MAX_CEILING: f32 = 64.0;
const SPIKE_DETECT_WINDOW: usize = 2;
const SPIKE_RATIO_THRESHOLD: f32 = 1.35;
const SPIKE_TARGET_MULTIPLIER: f32 = 1.80;
const SPIKE_SPREAD_RADIUS: usize = 3;
const SPIKE_SIGNAL_GAIN: f32 = 0.45;
const DOMINANT_BIN_PIN_THRESHOLD: f32 = 0.20;
const DOMINANT_BIN_ESCAPE_GAIN: f32 = 0.45;
const DOMINANT_BIN_ESCAPE_RADIUS: usize = 7;
const EARLY_DOMINANT_BIN_LIMIT: usize = 9;
const EARLY_DOMINANT_TILT_DAMP_GAIN: f32 = 0.85;
const EARLY_DOMINANT_GLOBAL_DAMP_GAIN: f32 = 0.55;
const NEIGHBOR_DAMAGE_BLEND: f32 = 0.65;
const MIN_TARGET_ARRIVAL: f32 = 0.05;
const MAX_TARGET_ARRIVAL: f32 = 0.95;
const SIGNAL_CLAMP_MIN: f32 = -0.35;
const SIGNAL_CLAMP_MAX: f32 = 0.35;
const CONFIDENCE_SCALE: f32 = 2.0;
const MIN_ARRIVAL_CONFIDENCE: f32 = 0.20;
const LATER_STAGE_IMPORTANCE_FACTOR: f32 = 0.0;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CdfProfile {
    Uniform,
    Trapezoid,
}

#[derive(Parser)]
#[command(
    name = "td-auto-hp-balance",
    about = "Automatically tune monster HP for balanced stage progression"
)]
struct Cli {
    /// Number of simulation samples to run
    #[arg(short, long, default_value_t = 1000)]
    samples: usize,

    /// SQLite database path for recording results
    #[arg(short, long, default_value = "sim_results.db")]
    db: PathBuf,

    /// Delete the target database file before running
    #[arg(long)]
    fresh_db: bool,

    /// Number of threads (0 = auto-detect)
    #[arg(short, long, default_value_t = 0)]
    threads: usize,

    /// Input simulation config TOML file
    #[arg(long)]
    config: Option<PathBuf>,

    /// Output tuned game config TOML file
    #[arg(long, default_value = "gameconfig.toml")]
    output: PathBuf,

    /// Stop after this many tuning iterations
    #[arg(long)]
    max_iterations: Option<usize>,

    /// Suppress progress output while tuning
    #[arg(long)]
    quiet: bool,

    /// Target CDF profile used for distribution flattening
    #[arg(long, value_enum, default_value_t = CdfProfile::Trapezoid)]
    cdf_profile: CdfProfile,

    /// Stage-segment gain for early rounds (1..16)
    #[arg(long, default_value_t = 0.85)]
    early_segment_gain: f32,

    /// Stage-segment gain for middle rounds (17..34)
    #[arg(long, default_value_t = 1.00)]
    mid_segment_gain: f32,

    /// Stage-segment gain for late rounds (35..50)
    #[arg(long, default_value_t = 1.35)]
    late_segment_gain: f32,

    /// Distribution stage signal gain
    #[arg(long, default_value_t = DEFAULT_DISTRIBUTION_STAGE_SIGNAL_GAIN)]
    distribution_stage_signal_gain: f32,

    /// Death-distribution signal blending weight
    #[arg(long, default_value_t = DEFAULT_DEATH_DISTRIBUTION_SIGNAL_WEIGHT)]
    death_distribution_signal_weight: f32,

    /// Global CDF pressure gain
    #[arg(long, default_value_t = CDF_GLOBAL_GAIN)]
    cdf_global_gain: f32,

    /// Normalize all normal-monster base HP to this value before tuning
    #[arg(long)]
    initial_base_hp: Option<f32>,

    /// Enable rough bootstrap phase optimized for uniform-base initialization
    #[arg(long, default_value_t = true)]
    rough_initial_balance: bool,

    /// Number of bootstrap iterations with strong stage-spread forcing
    #[arg(long, default_value_t = 12)]
    rough_bootstrap_iterations: usize,

    /// Initial stage baseline curve gain for rough mode
    #[arg(long, default_value_t = 1.6)]
    rough_initial_curve_gain: f32,

    /// Bootstrap forcing gain for rough mode
    #[arg(long, default_value_t = 2.0)]
    rough_bootstrap_gain: f32,

    /// Initial stage difficulty tilt used in rough mode
    #[arg(long, default_value_t = 1.2)]
    rough_initial_tilt: f32,

    /// Update gain for learned stage tilt in rough mode
    #[arg(long, default_value_t = 0.9)]
    rough_tilt_update_gain: f32,

    /// Update gain for rough-mode global HP scale
    #[arg(long, default_value_t = 2.2)]
    rough_global_scale_gain: f32,
}

struct BalanceStats {
    win_rate: f32,
    stage_means: Vec<f32>,
    stage_arrivals: Vec<usize>,
    clear_rates: Vec<f32>,
    overall_damage_mean: f32,
    below_15_frac: f32,
}

#[derive(Default)]
struct DistributionControl {
    ks_distance: f32,
    top_bin_share: f32,
    pressure: f32,
    bin_errors: Vec<f32>,
    dominant_bin_idx: usize,
    dominant_bin_escape_direction: f32,
    dominant_bin_escape_strength: f32,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    set_headless(true);

    let mut base_config = if let Some(path) = &cli.config {
        GameConfig::from_toml(path)?
    } else {
        GameConfig::from_toml("gameconfig.toml")?
    };
    if let Some(initial_base_hp) = cli.initial_base_hp {
        normalize_normal_monster_hp(&mut base_config, initial_base_hp.max(1.0));
    }
    if cli.rough_initial_balance {
        apply_boss_hp_from_stage(&mut base_config);
    } else {
        zero_boss_hp(&mut base_config);
    }

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

    let tuned_config = tune_hp_balance(&pool, base_config, &cli, recorder, stop_requested)?;

    tuned_config.write_toml(&cli.output)?;
    println!("Saved tuned config to {}", cli.output.display());

    Ok(())
}

fn target_stage_survival(stage: usize, stages: usize) -> f32 {
    if stages <= 1 {
        return TARGET_WIN_RATE;
    }

    let normalized = (stage as f32 - 1.0) / (stages as f32 - 1.0);
    TARGET_WIN_RATE
        + (1.0 - TARGET_WIN_RATE) * (1.0 - normalized.powf(TARGET_STAGE_SURVIVAL_EXPONENT))
}

fn tune_hp_balance(
    pool: &rayon::ThreadPool,
    mut config: GameConfig,
    cli: &Cli,
    recorder: Arc<SimRecorder>,
    stop_requested: Arc<AtomicBool>,
) -> anyhow::Result<GameConfig> {
    let stages = config.player.max_stages;
    let original_hp: HashMap<MonsterKind, f32> = config
        .monsters
        .stats
        .iter()
        .map(|(&kind, stat)| (kind, stat.base_hp))
        .collect();
    let mut stage_primary = build_stage_primary_monster(&config);
    let mut stage_momentum = build_initial_stage_scale_curve(stages, cli);
    let mut stable_iterations = 0;
    let mut learning_rate = BASE_LEARNING_RATE;
    let mut global_scale = 1.0_f32;
    let mut rough_tilt = cli.rough_initial_tilt;
    let mut rough_global_scale = 1.0_f32;

    println!(
        "=== Running fixed-sample auto tuning: samples={} ===",
        cli.samples
    );

    for iteration in 1.. {
        if stop_requested.load(Ordering::SeqCst) {
            println!("Ctrl-C received before iteration {iteration}, finalizing current config...");
            break;
        }
        if let Some(max_iterations) = cli.max_iterations {
            if iteration > max_iterations {
                println!("Reached max_iterations={max_iterations}, stopping auto balance.");
                break;
            }
        }
        let metrics = run_simulations(
            pool,
            Arc::new(config.clone()),
            cli.samples,
            &recorder,
            cli.quiet,
            &stop_requested,
        )?;
        if metrics.is_none() {
            println!("Ctrl-C received during simulation batch, finalizing last complete config...");
            break;
        }
        let metrics = metrics.unwrap();

        let overall_mean = metrics.overall_damage_mean;
        let target_message = format!(
            "win {:.1}%, below15 {:.1}%, mean {:.1}",
            metrics.win_rate * 100.0,
            metrics.below_15_frac * 100.0,
            overall_mean
        );
        println!("Iteration {iteration}: {target_message}");

        let reference_damage_mean = compute_robust_reference_damage(&metrics.stage_means);
        let win_error = metrics.win_rate - TARGET_WIN_RATE;
        let clear_rate_peak_ratio = compute_clear_rate_peak_ratio(&metrics.clear_rates);
        let damage_cv = compute_damage_cv(&metrics.stage_means);
        let distribution_control = compute_distribution_control(&metrics.clear_rates, cli.cdf_profile);
        let damage_weight = if damage_cv > DAMAGE_CV_WEIGHT_BOOST_THRESHOLD {
            DAMAGE_ERROR_WEIGHT_MAX
        } else {
            DAMAGE_ERROR_WEIGHT_BASE
        };
        let arrival_weight = ARRIVAL_ERROR_WEIGHT_BASE;

        if !cli.rough_initial_balance {
            let global_scale_target = (global_scale
                * (-distribution_control.pressure * cli.cdf_global_gain).exp())
            .clamp(GLOBAL_SCALE_MIN, GLOBAL_SCALE_MAX);
            global_scale = (global_scale * CDF_GLOBAL_SMOOTHING
                + global_scale_target * (1.0 - CDF_GLOBAL_SMOOTHING))
                .clamp(GLOBAL_SCALE_MIN, GLOBAL_SCALE_MAX);
        }

        if cli.rough_initial_balance {
            let win_delta = metrics.win_rate - TARGET_WIN_RATE;
            let tilt_step = win_delta * cli.rough_tilt_update_gain
                + distribution_control.pressure * 0.20;
            rough_tilt = (rough_tilt + tilt_step).clamp(ROUGH_TILT_MIN, ROUGH_TILT_MAX);

            let rough_global_step = (win_delta * cli.rough_global_scale_gain).clamp(-2.0, 2.0);
            rough_global_scale = (rough_global_scale * rough_global_step.exp())
                .clamp(ROUGH_GLOBAL_SCALE_MIN, ROUGH_GLOBAL_SCALE_MAX);

            if distribution_control.dominant_bin_idx <= EARLY_DOMINANT_BIN_LIMIT
                && distribution_control.top_bin_share > DOMINANT_BIN_PIN_THRESHOLD
            {
                let pin_strength = (distribution_control.top_bin_share - DOMINANT_BIN_PIN_THRESHOLD)
                    / (1.0 - DOMINANT_BIN_PIN_THRESHOLD);
                let pin_strength = pin_strength.clamp(0.0, 1.0);
                rough_tilt = (rough_tilt - pin_strength * EARLY_DOMINANT_TILT_DAMP_GAIN)
                    .clamp(ROUGH_TILT_MIN, ROUGH_TILT_MAX);
                rough_global_scale = (rough_global_scale
                    * (-pin_strength * EARLY_DOMINANT_GLOBAL_DAMP_GAIN).exp())
                    .clamp(ROUGH_GLOBAL_SCALE_MIN, ROUGH_GLOBAL_SCALE_MAX);
            }
        }

        let win_deviation = (metrics.win_rate - TARGET_WIN_RATE).abs();
        let aggressive_learning_rate = (BASE_LEARNING_RATE
            + win_deviation * 1.2
            + (damage_cv - DAMAGE_CV_WEIGHT_BOOST_THRESHOLD).max(0.0) * 0.25)
            .min(LEARNING_RATE_MAX);
        let iteration_learning_rate = learning_rate.max(aggressive_learning_rate);
        let stage_deaths = compute_stage_deaths(
            &metrics.stage_arrivals,
            metrics.win_rate,
            cli.samples,
            stages,
        );
        let target_stage_deaths =
            (cli.samples as f32 * (1.0 - TARGET_WIN_RATE) / stages as f32).max(1.0);

        let mut max_change: f32 = 0.0;
        let mut stage_scale_min: f32 = f32::INFINITY;
        let mut stage_scale_max: f32 = f32::NEG_INFINITY;

        let mut smoothed_stage_scale = vec![1.0_f32; stages + 1];
        for (stage, primary_monster) in stage_primary.iter().enumerate().skip(1).take(stages) {
            if primary_monster.is_some() {
                let arrival_count = metrics.stage_arrivals[stage - 1];
                let observed = if arrival_count > 0 {
                    metrics.stage_means[stage - 1].max(MIN_ZERO_DAMAGE_SCALE)
                } else {
                    MIN_ZERO_DAMAGE_SCALE
                };
                let target_damage = compute_smoothed_target_damage(
                    stage,
                    &metrics.stage_means,
                    reference_damage_mean,
                );
                let signal = compute_stage_adjustment_signal(
                    stage,
                    arrival_count,
                    observed,
                    target_damage,
                    &distribution_control,
                    metrics.win_rate,
                    cli.samples,
                    stages,
                    arrival_weight,
                    damage_weight,
                    cli,
                );
                let stage_ratio = stage as f32 / stages as f32;
                let bootstrap_signal = if cli.rough_initial_balance
                    && iteration <= cli.rough_bootstrap_iterations
                {
                    let ramp = (stage_ratio - 0.5) * 2.0;
                    let win_excess = metrics.win_rate - TARGET_WIN_RATE;
                    (win_excess * cli.rough_bootstrap_gain * ramp)
                        .clamp(-ROUGH_BOOTSTRAP_SIGNAL_CLAMP, ROUGH_BOOTSTRAP_SIGNAL_CLAMP)
                } else {
                    0.0
                };
                let signal = (signal + bootstrap_signal).clamp(SIGNAL_CLAMP_MIN, SIGNAL_CLAMP_MAX);

                let observed_stage_deaths = stage_deaths[stage - 1];
                let death_distribution_signal = -((observed_stage_deaths + 1.0)
                    / (target_stage_deaths + 1.0))
                    .ln()
                    .clamp(
                        -DEATH_DISTRIBUTION_SIGNAL_CLAMP,
                        DEATH_DISTRIBUTION_SIGNAL_CLAMP,
                    );
                let signal = (signal
                    + death_distribution_signal * cli.death_distribution_signal_weight)
                    .clamp(SIGNAL_CLAMP_MIN, SIGNAL_CLAMP_MAX);

                let mut target_scale = (signal * iteration_learning_rate).exp();
                let global_win_scale = compute_global_win_scale(metrics.win_rate);
                let secondary_win_scale = 1.0 + (global_win_scale - 1.0) * GLOBAL_WIN_SCALE_BLEND;
                if cli.rough_initial_balance {
                    let curve_ramp = ((stage_ratio - 0.5) * 2.0 * rough_tilt).exp();
                    let curve_ramp = curve_ramp.clamp(0.20, 12.0);
                    target_scale *= curve_ramp;
                } else {
                    target_scale *= global_scale * secondary_win_scale;
                }
                let win_deficit = (TARGET_WIN_RATE - metrics.win_rate).max(0.0);
                let win_surplus = (metrics.win_rate - TARGET_WIN_RATE).max(0.0);
                let deficit_expand =
                    ((win_deficit / TARGET_WIN_RATE).clamp(0.0, 2.0)) * WIN_DEFICIT_CLAMP_GAIN;
                let surplus_expand = ((win_surplus / (1.0 - TARGET_WIN_RATE)).clamp(0.0, 2.0))
                    * WIN_SURPLUS_CLAMP_GAIN;
                let stage_scale_min_floor = if cli.rough_initial_balance {
                    ROUGH_STAGE_SCALE_MIN_FLOOR
                } else {
                    STAGE_SCALE_CLAMP_MIN_FLOOR
                };
                let stage_scale_max_ceiling = if cli.rough_initial_balance {
                    ROUGH_STAGE_SCALE_MAX_CEILING
                } else {
                    STAGE_SCALE_CLAMP_MAX_CEILING
                };
                let stage_clamp_min =
                    (STAGE_SCALE_CLAMP_MIN - deficit_expand).max(stage_scale_min_floor);
                let stage_clamp_max = (STAGE_SCALE_CLAMP_MAX
                    + surplus_expand
                    + if cli.rough_initial_balance {
                        4.0 + rough_tilt * stage_ratio * 5.0
                    } else {
                        0.0
                    }
                    + (clear_rate_peak_ratio - CLEAR_PEAK_STABLE_THRESHOLD).max(0.0) * 0.2)
                    .min(stage_scale_max_ceiling);
                let relative_clamp_min =
                    (stage_momentum[stage] * stage_clamp_min).max(stage_scale_min_floor);
                let relative_clamp_max =
                    (stage_momentum[stage] * stage_clamp_max).min(stage_scale_max_ceiling);
                let blend_alpha = (BLEND_ALPHA_BASE
                    + ((metrics.win_rate - TARGET_WIN_RATE).abs() / (1.0 - TARGET_WIN_RATE))
                        .powf(0.75)
                        .clamp(0.0, 1.0)
                        * BLEND_ALPHA_WIN_DEFICIT_GAIN)
                    .min(BLEND_ALPHA_MAX);
                let mut stage_scale =
                    stage_momentum[stage] * (1.0 - blend_alpha) + target_scale * blend_alpha;
                stage_scale = stage_scale.clamp(relative_clamp_min, relative_clamp_max);
                stage_scale = stage_scale.clamp(stage_scale_min_floor, stage_scale_max_ceiling);
                smoothed_stage_scale[stage] = stage_scale;
                stage_scale_min = stage_scale_min.min(stage_scale);
                stage_scale_max = stage_scale_max.max(stage_scale);
            }
        }
        stage_momentum = smoothed_stage_scale.clone();

        let mut kind_log_scale_sum: HashMap<MonsterKind, f32> = HashMap::new();
        let mut kind_weight_sum: HashMap<MonsterKind, f32> = HashMap::new();
        for stage in 1..=stages {
            if let Some(monster_kind) = stage_primary[stage] {
                let stage_scale = smoothed_stage_scale[stage].max(0.01);
                *kind_log_scale_sum.entry(monster_kind).or_insert(0.0) += stage_scale.ln();
                *kind_weight_sum.entry(monster_kind).or_insert(0.0) += 1.0;
            }
        }

        for (monster_kind, log_sum) in kind_log_scale_sum {
            if let Some(stat) = config.monsters.stats.get_mut(&monster_kind) {
                let weight = kind_weight_sum.get(&monster_kind).copied().unwrap_or(1.0).max(1.0);
                let kind_scale = (log_sum / weight).exp();
                let base_hp = original_hp
                    .get(&monster_kind)
                    .copied()
                    .unwrap_or(stat.base_hp);
                let global_scale = if cli.rough_initial_balance {
                    rough_global_scale
                } else {
                    1.0
                };
                let applied_scale = kind_scale * global_scale;
                stat.base_hp = (base_hp * applied_scale).max(base_hp * 0.25);
                max_change = max_change.max((applied_scale - 1.0).abs());
            }
        }

        if cli.rough_initial_balance {
            apply_boss_hp_from_stage(&mut config);
        }

        println!(
            "  debug: win_err={:.4} learn_rate={:.4} clear_peak={:.3} damage_cv={:.3} ks={:.3} top_bin={:.3} dom_bin={} gscale={:.3} tilt={:.3} rough_gscale={:.3} max_change={:.6} stage_scale=[{:.4},{:.4}] stable={}/{}",
            win_error,
            iteration_learning_rate,
            clear_rate_peak_ratio,
            damage_cv,
            distribution_control.ks_distance,
            distribution_control.top_bin_share,
            distribution_control.dominant_bin_idx + 1,
            global_scale,
            rough_tilt,
            rough_global_scale,
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

        if max_change < 1e-4 {
            println!("Iteration {iteration} converged with max change {max_change:.6}");
            break;
        }

        let arrival_rmse = compute_arrival_rmse(&metrics.stage_arrivals, cli.samples, stages);
        let win_stable = (metrics.win_rate - TARGET_WIN_RATE).abs() < 0.03;
        let clear_peak_stable = distribution_control.ks_distance < KS_CONVERGENCE_THRESHOLD;
        let clear_topbin_stable = distribution_control.top_bin_share < TOP_BIN_CONVERGENCE_THRESHOLD;
        let arrival_stable = arrival_rmse < 0.05;
        let damage_stable = damage_cv < DAMAGE_CV_STABLE_THRESHOLD;

        if win_stable
            && clear_peak_stable
            && clear_topbin_stable
            && arrival_stable
            && damage_stable
            && metrics.below_15_frac <= 0.40
        {
            stable_iterations += 1;
        } else {
            stable_iterations = 0;
        }

        if stable_iterations >= 2 {
            println!(
                "Iteration {iteration} reached stable metrics after {stable_iterations} iterations"
            );
            break;
        }

        if stop_requested.load(Ordering::SeqCst) {
            println!("Ctrl-C received after iteration {iteration}, finalizing current config...");
            break;
        }

        learning_rate = (iteration_learning_rate * LEARNING_RATE_DECAY)
            .max(aggressive_learning_rate * 0.75)
            .min(LEARNING_RATE_MAX);
        stage_primary = build_stage_primary_monster(&config);
    }

    apply_boss_hp_from_stage(&mut config);
    Ok(config)
}

fn zero_boss_hp(config: &mut GameConfig) {
    for (kind, stat) in config.monsters.stats.iter_mut() {
        if !kind.is_normal_monster() {
            stat.base_hp = 0.0;
        }
    }
}

fn normalize_normal_monster_hp(config: &mut GameConfig, hp: f32) {
    for (kind, stat) in config.monsters.stats.iter_mut() {
        if kind.is_normal_monster() {
            stat.base_hp = hp;
        }
    }
}

fn build_initial_stage_scale_curve(stages: usize, cli: &Cli) -> Vec<f32> {
    let mut curve = vec![1.0_f32; stages + 1];
    if !cli.rough_initial_balance || stages <= 1 {
        return curve;
    }

    for stage in 1..=stages {
        let r = stage as f32 / stages as f32;
        let centered = (r - 0.5) * 2.0;
        let scale = (centered * cli.rough_initial_curve_gain).exp();
        curve[stage] = scale.clamp(0.55, 1.95);
    }
    curve
}

fn build_stage_primary_monster(config: &GameConfig) -> Vec<Option<MonsterKind>> {
    let max_stage = config.player.max_stages;
    let mut stage_primary = vec![None; max_stage + 1];

    for stage_wave in &config.monsters.stage_waves {
        if stage_wave.stage > max_stage {
            continue;
        }

        stage_primary[stage_wave.stage] = stage_wave
            .entries
            .iter()
            .find(|entry| entry.kind.is_normal_monster())
            .map(|entry| entry.kind);
    }

    stage_primary
}

fn apply_boss_hp_from_stage(config: &mut GameConfig) {
    for stage_wave in &config.monsters.stage_waves {
        let normal_kind = stage_wave
            .entries
            .iter()
            .find(|entry| entry.kind.is_normal_monster())
            .map(|entry| entry.kind);

        let boss_kind = stage_wave
            .entries
            .iter()
            .find(|entry| !entry.kind.is_normal_monster())
            .map(|entry| entry.kind);

        if let (Some(normal_kind), Some(boss_kind)) = (normal_kind, boss_kind)
            && let Some(normal_hp) = config
                .monsters
                .stats
                .get(&normal_kind)
                .map(|stat| stat.base_hp)
            && let Some(boss_stat) = config.monsters.stats.get_mut(&boss_kind)
        {
            boss_stat.base_hp = normal_hp * 1.5;
        }
    }
}

fn compute_clear_rate_peak_ratio(clear_rates: &[f32]) -> f32 {
    if clear_rates.is_empty() {
        return 1.0;
    }

    let mut bins = [0usize; 51];
    for &rate in clear_rates {
        let idx = ((rate.clamp(0.0, 100.0) as usize) / 2).min(50);
        bins[idx] += 1;
    }

    let avg = clear_rates.len() as f32 / bins.len() as f32;
    let max_bin = *bins.iter().max().unwrap_or(&0) as f32;
    if avg > 0.0 {
        (max_bin / avg).max(1.0)
    } else {
        1.0
    }
}

fn compute_global_win_scale(win_rate: f32) -> f32 {
    let raw_win_scale = ((win_rate - TARGET_WIN_RATE) * GLOBAL_WIN_SCALE_GAIN).exp();
    raw_win_scale.clamp(GLOBAL_WIN_SCALE_MIN, GLOBAL_WIN_SCALE_MAX)
}

fn compute_smoothed_target_damage(stage: usize, stage_means: &[f32], reference_damage: f32) -> f32 {
    let idx = stage.saturating_sub(1);
    let stages = stage_means.len();
    let observed = stage_means[idx].max(MIN_ZERO_DAMAGE_SCALE);
    let prev = if idx > 0 {
        stage_means[idx - 1].max(MIN_ZERO_DAMAGE_SCALE)
    } else {
        observed
    };
    let next = if idx + 1 < stages {
        stage_means[idx + 1].max(MIN_ZERO_DAMAGE_SCALE)
    } else {
        observed
    };
    let neighbor_avg = (prev + observed + next) / 3.0;
    let target =
        neighbor_avg * NEIGHBOR_DAMAGE_BLEND + reference_damage * (1.0 - NEIGHBOR_DAMAGE_BLEND);
    target.max(MIN_ZERO_DAMAGE_SCALE)
}

fn compute_robust_reference_damage(stage_means: &[f32]) -> f32 {
    let mut filtered: Vec<f32> = stage_means
        .iter()
        .copied()
        .filter(|mean| *mean > MIN_ZERO_DAMAGE_SCALE)
        .collect();
    if filtered.is_empty() {
        return 1.0;
    }
    filtered.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = filtered.len() / 2;
    if filtered.len() % 2 == 0 {
        (filtered[mid - 1] + filtered[mid]) * 0.5
    } else {
        filtered[mid]
    }
    .max(1.0)
}

fn compute_stage_adjustment_signal(
    stage: usize,
    arrival_count: usize,
    observed_damage: f32,
    target_damage: f32,
    distribution_control: &DistributionControl,
    win_rate: f32,
    samples: usize,
    stages: usize,
    arrival_weight: f32,
    damage_weight: f32,
    cli: &Cli,
) -> f32 {
    let observed_arrival = arrival_count as f32 / samples as f32;
    let target_arrival =
        target_stage_survival(stage, stages).clamp(MIN_TARGET_ARRIVAL, MAX_TARGET_ARRIVAL);
    let raw_arrival_signal = if observed_arrival > 0.0 {
        (observed_arrival / target_arrival)
            .ln()
            .clamp(SIGNAL_CLAMP_MIN, SIGNAL_CLAMP_MAX)
    } else {
        -1.8
    };

    let arrival_confidence = if arrival_count == 0 {
        MIN_ARRIVAL_CONFIDENCE
    } else {
        ((arrival_count as f32 / samples as f32).sqrt() * CONFIDENCE_SCALE)
            .clamp(MIN_ARRIVAL_CONFIDENCE, 1.0)
    };

    let stage_importance = 1.0 + (stage as f32 / stages as f32) * LATER_STAGE_IMPORTANCE_FACTOR;
    let arrival_signal = raw_arrival_signal * arrival_confidence * stage_importance;

    let damage_signal = if arrival_count > 0 && observed_damage > 0.0 {
        (target_damage / observed_damage)
            .ln()
            .clamp(SIGNAL_CLAMP_MIN, SIGNAL_CLAMP_MAX)
            * stage_importance
    } else {
        0.0
    };

    let stage_ratio = stage as f32 / stages as f32;
    let bin_idx = ((stage_ratio * 100.0) / 2.0)
        .round()
        .clamp(0.0, (CLEAR_RATE_BIN_COUNT - 1) as f32) as usize;
    let bin_error = distribution_control
        .bin_errors
        .get(bin_idx)
        .copied()
        .unwrap_or(0.0);
    let left_error = if bin_idx > 0 {
        distribution_control.bin_errors[bin_idx - 1]
    } else {
        bin_error
    };
    let right_error = if bin_idx + 1 < distribution_control.bin_errors.len() {
        distribution_control.bin_errors[bin_idx + 1]
    } else {
        bin_error
    };
    let neighborhood_error = bin_error * 0.6 + left_error * 0.2 + right_error * 0.2;
    let bin_polarity = stage_ratio * 2.0 - 1.0;
    let segment_gain = compute_stage_segment_gain(stage, stages, cli);
    let raw_distribution_signal =
        neighborhood_error * bin_polarity * cli.distribution_stage_signal_gain * segment_gain;
    let distribution_signal = if win_rate > TARGET_WIN_RATE + 0.02 {
        raw_distribution_signal.max(0.0)
    } else if win_rate < TARGET_WIN_RATE - 0.02 {
        raw_distribution_signal.min(0.0)
    } else {
        raw_distribution_signal
    };

    let dominant_distance = bin_idx.abs_diff(distribution_control.dominant_bin_idx);
    let dominant_proximity = if dominant_distance <= DOMINANT_BIN_ESCAPE_RADIUS {
        (DOMINANT_BIN_ESCAPE_RADIUS + 1 - dominant_distance) as f32
            / (DOMINANT_BIN_ESCAPE_RADIUS + 1) as f32
    } else {
        0.0
    };
    let anti_pinning_signal = distribution_control.dominant_bin_escape_direction
        * distribution_control.dominant_bin_escape_strength
        * dominant_proximity;

    (arrival_weight * arrival_signal
        + damage_weight * damage_signal
        + distribution_signal
        + anti_pinning_signal)
    .clamp(SIGNAL_CLAMP_MIN, SIGNAL_CLAMP_MAX)
}

fn compute_distribution_control(clear_rates: &[f32], profile: CdfProfile) -> DistributionControl {
    if clear_rates.is_empty() {
        return DistributionControl::default();
    }

    let mut histogram_bins = vec![0usize; CLEAR_RATE_BIN_COUNT];
    for &rate in clear_rates {
        let histogram_idx = ((rate.clamp(0.0, 100.0) as usize) / 2).min(CLEAR_RATE_BIN_COUNT - 1);
        histogram_bins[histogram_idx] += 1;
    }

    let total = clear_rates.len() as f32;
    let target_weights = build_target_bin_weights(profile);
    let mut obs_cdf = 0.0_f32;
    let mut ks_distance = 0.0_f32;
    let mut low_mass = 0.0_f32;
    let mut high_mass = 0.0_f32;
    let mut top_bin_share = 0.0_f32;
    let mut bin_errors = vec![0.0_f32; CLEAR_RATE_BIN_COUNT];
    let mut shares = vec![0.0_f32; CLEAR_RATE_BIN_COUNT];

    for (idx, &count) in histogram_bins.iter().enumerate() {
        let share = count as f32 / total;
        shares[idx] = share;
        let target_share = target_weights[idx];
        bin_errors[idx] = share - target_share;
        obs_cdf += share;
        let tgt_cdf = target_weights.iter().take(idx + 1).sum::<f32>();
        let cdf_error = obs_cdf - tgt_cdf;
        ks_distance = ks_distance.max(cdf_error.abs());
        top_bin_share = top_bin_share.max(share);

        if idx < CLEAR_RATE_BIN_COUNT / 3 {
            low_mass += share;
        }
        if idx >= (CLEAR_RATE_BIN_COUNT * 2) / 3 {
            high_mass += share;
        }
    }

    let hard_bias = low_mass - high_mass;
    let target_top_share = target_weights
        .iter()
        .copied()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(1.0 / CLEAR_RATE_BIN_COUNT as f32);
    let spike_excess = (top_bin_share - target_top_share).max(0.0);
    let peak_idx = histogram_bins
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| **count)
        .map(|(idx, _)| idx)
        .unwrap_or(0);
    let spike_direction = if peak_idx < CLEAR_RATE_BIN_COUNT / 2 {
        1.0
    } else {
        -1.0
    };

    let pressure = (hard_bias + spike_excess * spike_direction).clamp(-1.0, 1.0);
    let dominant_bin_escape_direction = if peak_idx < CLEAR_RATE_BIN_COUNT / 2 {
        -1.0
    } else {
        1.0
    };
    let dominant_pin_excess = (top_bin_share - DOMINANT_BIN_PIN_THRESHOLD).max(0.0);
    let dominant_bin_escape_strength = (dominant_pin_excess
        / (1.0 - DOMINANT_BIN_PIN_THRESHOLD))
        .clamp(0.0, 1.0)
        * DOMINANT_BIN_ESCAPE_GAIN;

    let mut spike_overloads = vec![0.0_f32; CLEAR_RATE_BIN_COUNT];
    for idx in 0..CLEAR_RATE_BIN_COUNT {
        let start = idx.saturating_sub(SPIKE_DETECT_WINDOW);
        let end = (idx + SPIKE_DETECT_WINDOW).min(CLEAR_RATE_BIN_COUNT - 1);
        let mut local_sum = 0.0_f32;
        let mut local_count = 0usize;
        for j in start..=end {
            local_sum += shares[j];
            local_count += 1;
        }
        let local_mean = if local_count > 0 {
            local_sum / local_count as f32
        } else {
            0.0
        };
        let target_share = target_weights[idx];
        if local_mean > 1e-6
            && shares[idx] > local_mean * SPIKE_RATIO_THRESHOLD
            && shares[idx] > target_share * SPIKE_TARGET_MULTIPLIER
        {
            let spike_ratio = (shares[idx] / local_mean - SPIKE_RATIO_THRESHOLD).max(0.0);
            spike_overloads[idx] = spike_ratio.min(2.5);
        }
    }

    for idx in 0..CLEAR_RATE_BIN_COUNT {
        let spike = spike_overloads[idx];
        if spike <= 0.0 {
            continue;
        }
        for d in 0..=SPIKE_SPREAD_RADIUS {
            let decay = (SPIKE_SPREAD_RADIUS + 1 - d) as f32 / (SPIKE_SPREAD_RADIUS + 1) as f32;
            let spread = spike * decay * SPIKE_SIGNAL_GAIN;
            let left = idx.saturating_sub(d);
            let right = (idx + d).min(CLEAR_RATE_BIN_COUNT - 1);
            bin_errors[left] += spread;
            if right != left {
                bin_errors[right] += spread;
            }
        }
    }

    for err in &mut bin_errors {
        *err = err.clamp(-0.80, 0.80);
    }

    DistributionControl {
        ks_distance,
        top_bin_share,
        pressure,
        bin_errors,
        dominant_bin_idx: peak_idx,
        dominant_bin_escape_direction,
        dominant_bin_escape_strength,
    }
}

fn build_target_bin_weights(profile: CdfProfile) -> Vec<f32> {
    let mut weights = vec![0.0_f32; CLEAR_RATE_BIN_COUNT];
    match profile {
        CdfProfile::Uniform => {
            let w = 1.0 / CLEAR_RATE_BIN_COUNT as f32;
            weights.fill(w);
        }
        CdfProfile::Trapezoid => {
            for (idx, w) in weights.iter_mut().enumerate() {
                let x = idx as f32 / (CLEAR_RATE_BIN_COUNT - 1) as f32;
                *w = if x < 0.15 {
                    0.4 + x / 0.15 * 0.6
                } else if x <= 0.85 {
                    1.0
                } else {
                    1.0 - (x - 0.85) / 0.15 * 0.6
                };
            }
            let sum = weights.iter().sum::<f32>().max(1e-6);
            for w in &mut weights {
                *w /= sum;
            }
        }
    }
    weights
}

fn compute_stage_segment_gain(stage: usize, stages: usize, cli: &Cli) -> f32 {
    if stages <= 1 {
        return cli.mid_segment_gain;
    }
    let r = stage as f32 / stages as f32;
    if r <= 0.34 {
        cli.early_segment_gain
    } else if r <= 0.68 {
        cli.mid_segment_gain
    } else {
        cli.late_segment_gain
    }
}

fn compute_damage_cv(stage_means: &[f32]) -> f32 {
    let filtered: Vec<f32> = stage_means
        .iter()
        .copied()
        .filter(|mean| *mean > MIN_ZERO_DAMAGE_SCALE)
        .collect();
    if filtered.len() < 2 {
        return 0.0;
    }

    let mean = filtered.iter().sum::<f32>() / filtered.len() as f32;
    if mean <= 0.0 {
        return 0.0;
    }
    let variance = filtered
        .iter()
        .map(|v| {
            let d = *v - mean;
            d * d
        })
        .sum::<f32>()
        / filtered.len() as f32;
    variance.sqrt() / mean
}

fn compute_arrival_rmse(stage_arrivals: &[usize], samples: usize, stages: usize) -> f32 {
    let mut sum_sq = 0.0;
    for (idx, &count) in stage_arrivals.iter().enumerate().take(stages) {
        let observed = count as f32 / samples as f32;
        let target = target_stage_survival(idx + 1, stages);
        let error = observed - target;
        sum_sq += error * error;
    }
    (sum_sq / stages as f32).sqrt()
}

fn compute_stage_deaths(
    stage_arrivals: &[usize],
    win_rate: f32,
    samples: usize,
    stages: usize,
) -> Vec<f32> {
    let mut deaths = vec![0.0_f32; stages];
    for stage in 0..stages {
        let current = stage_arrivals.get(stage).copied().unwrap_or(0) as f32;
        let next = if stage + 1 < stages {
            stage_arrivals.get(stage + 1).copied().unwrap_or(0) as f32
        } else {
            win_rate * samples as f32
        };
        deaths[stage] = (current - next).max(0.0);
    }
    deaths
}

fn run_simulations(
    pool: &rayon::ThreadPool,
    config: Arc<GameConfig>,
    samples: usize,
    recorder: &Arc<SimRecorder>,
    quiet: bool,
    stop_requested: &Arc<AtomicBool>,
) -> anyhow::Result<Option<BalanceStats>> {
    let stage_count = config.player.max_stages;
    let stage_sums = Arc::new(Mutex::new(vec![0.0_f32; stage_count]));
    let stage_arrivals = Arc::new(Mutex::new(vec![0_usize; stage_count]));
    let stage_visits = Arc::new(Mutex::new(vec![0_usize; stage_count]));
    let below_15_count = AtomicUsize::new(0);
    let victories = AtomicUsize::new(0);
    let canceled = AtomicBool::new(false);
    let clear_rates = Arc::new(Mutex::new(Vec::<f32>::new()));

    let overall_pb = if quiet {
        None
    } else {
        let pb = ProgressBar::new(samples as u64);
        let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )?;
        pb.set_style(style);
        pb.set_message("wins 0");
        Some(pb)
    };

    pool.install(|| {
        (0..samples).into_par_iter().for_each(|_| {
            if stop_requested.load(Ordering::SeqCst) {
                canceled.store(true, Ordering::SeqCst);
                return;
            }

            let mut rng = rand::thread_rng();
            let seed: u64 = rng.r#gen();

            let shop_strategy: Box<dyn tower_defense::simulator::strategies::ShopStrategy> =
                Box::new(SynergyShopStrategy);
            let card_strategy: Box<dyn tower_defense::simulator::strategies::CardRerollStrategy> =
                Box::new(ItemAwareRerollStrategy);
            let tower_strategy = HeuristicPlacementStrategy;
            let item_strategy: Box<dyn tower_defense::simulator::strategies::ItemUseStrategy> =
                Box::new(HeuristicItemUseStrategy);
            let treasure_strategy = SynergyTreasureStrategy;

            let sim_id = format!("sim_{seed:016x}");

            if let Err(e) = recorder.record_simulation_start(
                &sim_id,
                shop_strategy.name(),
                card_strategy.name(),
                tower_strategy.name(),
                item_strategy.name(),
                seed,
            ) {
                eprintln!("Failed to record start for {sim_id}: {e}");
            }

            let mut game = HeadlessGame::new_with_config(config.clone());
            let result = game.run(
                shop_strategy.as_ref(),
                card_strategy.as_ref(),
                &tower_strategy,
                item_strategy.as_ref(),
                &treasure_strategy,
                &mut rng,
                |_clear_rate| !stop_requested.load(Ordering::SeqCst),
            );

            let mut arrived = vec![false; stage_count];
            {
                let mut sums = stage_sums.lock().unwrap();
                let mut arrivals = stage_arrivals.lock().unwrap();
                let mut visits = stage_visits.lock().unwrap();
                for &(stage, damage) in &result.stage_damage {
                    if stage == 0 || stage > stage_count {
                        continue;
                    }
                    let idx = stage - 1;
                    sums[idx] += damage;
                    visits[idx] += 1;
                    if !arrived[idx] {
                        arrived[idx] = true;
                        arrivals[idx] += 1;
                    }
                }
            }

            {
                let mut rates = clear_rates.lock().unwrap();
                rates.push(result.clear_rate);
            }

            if let Err(e) = recorder.record_simulation_end(
                &sim_id,
                result.victory,
                result.final_stage,
                result.clear_rate,
                result.final_hp,
                result.final_gold,
                result.total_towers_placed,
                result.total_items_used,
                result.total_damage_taken,
                result.total_gold_earned,
            ) {
                eprintln!("Failed to record end for {sim_id}: {e}");
            }

            if let Err(e) = recorder.record_events(&sim_id, &game.events) {
                eprintln!("Failed to record events for {sim_id}: {e}");
            }

            if result.victory {
                victories.fetch_add(1, Ordering::Relaxed);
            }

            if result.final_stage < 15 {
                below_15_count.fetch_add(1, Ordering::Relaxed);
            }

            if let Some(pb) = &overall_pb {
                pb.inc(1);
                let win_count = victories.load(Ordering::Relaxed);
                pb.set_message(format!("wins {win_count}"));
            }
        });
    });

    if let Some(pb) = overall_pb {
        pb.finish_and_clear();
    }

    let stage_arrivals = Arc::try_unwrap(stage_arrivals)
        .unwrap()
        .into_inner()
        .unwrap();
    let stage_visits = Arc::try_unwrap(stage_visits).unwrap().into_inner().unwrap();
    let stage_sums = Arc::try_unwrap(stage_sums).unwrap().into_inner().unwrap();
    let total_wins = victories.load(Ordering::Relaxed);
    let below_15_count = below_15_count.load(Ordering::Relaxed);
    let clear_rates = Arc::try_unwrap(clear_rates).unwrap().into_inner().unwrap();

    if canceled.load(Ordering::SeqCst) {
        return Ok(None);
    }

    let mut stage_means = vec![0.0_f32; stage_count];
    let mut total_damage = 0.0_f32;
    let mut total_visits = 0_usize;

    for stage in 0..stage_count {
        if stage_visits[stage] > 0 {
            stage_means[stage] = stage_sums[stage] / stage_visits[stage] as f32;
            total_damage += stage_sums[stage];
            total_visits += stage_visits[stage];
        }
    }

    let overall_damage_mean = if total_visits > 0 {
        total_damage / total_visits as f32
    } else {
        0.0
    };

    let below_15_frac = below_15_count as f32 / samples as f32;

    Ok(Some(BalanceStats {
        win_rate: total_wins as f32 / samples as f32,
        stage_means,
        stage_arrivals,
        clear_rates,
        overall_damage_mean,
        below_15_frac,
    }))
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
