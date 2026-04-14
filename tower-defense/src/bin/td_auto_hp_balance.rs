use clap::Parser;
use ctrlc;
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
const WIN_RATE_LEARNING_RATE: f32 = 2.50;
const STAGE_SHAPE_BIAS: f32 = 0.06;
const TARGET_STAGE_COUNT_ADJUSTMENT_RATE: f32 = 0.25;
const TARGET_STAGE_COUNT_CLAMP: f32 = 0.40;
const CUMULATIVE_SURVIVAL_ADJUSTMENT_RATE: f32 = 0.30;
const SPIKE_PENALTY_FACTOR: f32 = 0.18;
const MIN_ZERO_DAMAGE_SCALE: f32 = 0.05;
const MOMENTUM: f32 = 0.18;
const MAX_STAGE_ADJUST: f32 = 0.22;
const BASE_LEARNING_RATE: f32 = 0.18;
const LEARNING_RATE_DECAY: f32 = 0.92;
const WIN_RATE_TOLERANCE: f32 = 0.010;
const STABLE_ITERATIONS_REQUIRED: usize = 3;

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

    /// Suppress progress output while tuning
    #[arg(long)]
    quiet: bool,
}

struct BalanceStats {
    win_rate: f32,
    stage_means: Vec<f32>,
    stage_counts: Vec<usize>,
    clear_rates: Vec<f32>,
    overall_damage_mean: f32,
    below_15_frac: f32,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    set_headless(true);

    let mut base_config = if let Some(path) = &cli.config {
        GameConfig::from_toml(path)?
    } else {
        GameConfig::from_toml("gameconfig.toml")?
    };
    zero_boss_hp(&mut base_config);

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

fn target_stage_count(stage: usize, stages: usize, samples: usize) -> f32 {
    if stages <= 1 {
        return samples as f32;
    }

    let normalized = (stage as f32 - 1.0) / (stages as f32 - 1.0);
    let survival_fraction =
        TARGET_WIN_RATE + (1.0 - TARGET_WIN_RATE) * (1.0 - normalized.powf(2.5));
    (samples as f32 * survival_fraction).max(samples as f32 * TARGET_WIN_RATE)
}

fn target_stage_survival(stage: usize, stages: usize) -> f32 {
    if stages <= 1 {
        return TARGET_WIN_RATE;
    }

    let normalized = (stage as f32 - 1.0) / (stages as f32 - 1.0);
    TARGET_WIN_RATE + (1.0 - TARGET_WIN_RATE) * (1.0 - normalized.powf(2.5))
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
        .map(|(kind, stat)| (kind.clone(), stat.base_hp))
        .collect();
    let mut stage_primary = build_stage_primary_monster(&config);
    let mut stage_momentum = vec![1.0_f32; stages + 1];
    let mut stable_iterations = 0;
    let mut learning_rate = BASE_LEARNING_RATE;

    println!(
        "=== Running fixed-sample auto tuning: samples={} ===",
        cli.samples
    );

    for iteration in 1.. {
        if stop_requested.load(Ordering::SeqCst) {
            println!("Ctrl-C received before iteration {iteration}, finalizing current config...");
            break;
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

        let win_error = metrics.win_rate - TARGET_WIN_RATE;
        let global_scale = (1.0 + win_error * WIN_RATE_LEARNING_RATE).clamp(1.0 - 0.30, 1.0 + 0.30);

        let mut max_change: f32 = 0.0;
        let mut stage_scale_min: f32 = f32::INFINITY;
        let mut stage_scale_max: f32 = f32::NEG_INFINITY;

        for stage in 1..=stages {
            if let Some(monster_kind) = stage_primary.get(stage).copied().flatten() {
                let count = metrics.stage_counts[stage - 1];
                if count == 0 {
                    continue;
                }

                let observed = metrics.stage_means[stage - 1].max(MIN_ZERO_DAMAGE_SCALE);
                let stage_position = if stages > 1 {
                    (stage as f32 - 1.0) / (stages as f32 - 1.0)
                } else {
                    0.0
                };
                let actual_count = metrics.stage_counts[stage - 1] as f32;
                let target_count = target_stage_count(stage, stages, cli.samples);
                let count_error = if target_count > 0.0 {
                    actual_count / target_count - 1.0
                } else {
                    0.0
                };
                let target_bias = (1.0 + count_error * TARGET_STAGE_COUNT_ADJUSTMENT_RATE).clamp(
                    1.0 - TARGET_STAGE_COUNT_CLAMP,
                    1.0 + TARGET_STAGE_COUNT_CLAMP,
                );

                let observed_survival = metrics.stage_counts[stage - 1] as f32 / cli.samples as f32;
                let target_survival = target_stage_survival(stage, stages);
                let survival_error = if target_survival > 0.0 {
                    observed_survival / target_survival - 1.0
                } else {
                    0.0
                };
                let survival_bias = (1.0 + survival_error * CUMULATIVE_SURVIVAL_ADJUSTMENT_RATE)
                    .clamp(
                        1.0 - TARGET_STAGE_COUNT_CLAMP,
                        1.0 + TARGET_STAGE_COUNT_CLAMP,
                    );

                let prev_mean = if stage > 1 {
                    metrics.stage_means[stage - 2]
                } else {
                    observed
                };
                let next_mean = if stage < stages {
                    metrics.stage_means[stage]
                } else {
                    observed
                };
                let smoothed_mean = (prev_mean + observed + next_mean) / 3.0;
                let local_ratio =
                    (smoothed_mean / observed).clamp(1.0 - learning_rate, 1.0 + learning_rate);
                let shape_bias =
                    1.0 + (0.5 - stage_position) * STAGE_SHAPE_BIAS * (1.0 - stage_position);

                let neighbor_avg = if stage > 1 && stage < stages {
                    (prev_mean + next_mean) * 0.5
                } else if stage > 1 {
                    prev_mean
                } else {
                    next_mean
                };
                let spike_ratio = if observed > neighbor_avg {
                    let excess = (observed - neighbor_avg) / neighbor_avg;
                    1.0 - excess.clamp(0.0, 0.5) * SPIKE_PENALTY_FACTOR * stage_position
                } else {
                    1.0
                };

                let mut stage_scale = global_scale
                    * shape_bias
                    * local_ratio
                    * spike_ratio
                    * target_bias
                    * survival_bias;
                stage_scale =
                    stage_momentum[stage] + (stage_scale - stage_momentum[stage]) * MOMENTUM;
                stage_momentum[stage] = stage_scale;
                stage_scale = stage_scale.clamp(1.0 - MAX_STAGE_ADJUST, 1.0 + MAX_STAGE_ADJUST);
                stage_scale_min = stage_scale_min.min(stage_scale);
                stage_scale_max = stage_scale_max.max(stage_scale);

                if let Some(stat) = config.monsters.stats.get_mut(&monster_kind) {
                    let base_hp = original_hp
                        .get(&monster_kind)
                        .copied()
                        .unwrap_or(stat.base_hp);
                    stat.base_hp = (base_hp * stage_scale).max(base_hp * 0.25);
                    max_change = max_change.max((stage_scale - 1.0).abs());
                }
            }
        }

        println!(
            "  debug: win_err={:.4} global_scale={:.4} learn_rate={:.4} max_change={:.6} stage_scale=[{:.4},{:.4}] stable={}/{}",
            win_error,
            global_scale,
            learning_rate,
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

        let target_range =
            (TARGET_WIN_RATE - WIN_RATE_TOLERANCE)..=(TARGET_WIN_RATE + WIN_RATE_TOLERANCE);
        if target_range.contains(&metrics.win_rate) && metrics.below_15_frac <= 0.40 {
            stable_iterations += 1;
        } else {
            stable_iterations = 0;
        }

        if stable_iterations >= STABLE_ITERATIONS_REQUIRED {
            println!(
                "Iteration {iteration} reached stable target win rate for {STABLE_ITERATIONS_REQUIRED} iterations"
            );
            break;
        }

        if stop_requested.load(Ordering::SeqCst) {
            println!("Ctrl-C received after iteration {iteration}, finalizing current config...");
            break;
        }

        learning_rate *= LEARNING_RATE_DECAY;
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

        if let (Some(normal_kind), Some(boss_kind)) = (normal_kind, boss_kind) {
            if let Some(normal_hp) = config
                .monsters
                .stats
                .get(&normal_kind)
                .map(|stat| stat.base_hp)
            {
                if let Some(boss_stat) = config.monsters.stats.get_mut(&boss_kind) {
                    boss_stat.base_hp = normal_hp * 1.5;
                }
            }
        }
    }
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
    let stage_counts = Arc::new(Mutex::new(vec![0_usize; stage_count]));
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

            {
                let mut sums = stage_sums.lock().unwrap();
                let mut counts = stage_counts.lock().unwrap();
                for (idx, &damage) in result.stage_damage.iter().enumerate() {
                    if idx >= stage_count {
                        break;
                    }
                    sums[idx] += damage;
                    counts[idx] += 1;
                }
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

    let stage_counts = Arc::try_unwrap(stage_counts).unwrap().into_inner().unwrap();
    let stage_sums = Arc::try_unwrap(stage_sums).unwrap().into_inner().unwrap();
    let total_wins = victories.load(Ordering::Relaxed);
    let below_15_count = below_15_count.load(Ordering::Relaxed);
    let clear_rates = Arc::try_unwrap(clear_rates).unwrap().into_inner().unwrap();

    if canceled.load(Ordering::SeqCst) {
        return Ok(None);
    }

    let mut stage_means = vec![0.0_f32; stage_count];
    let mut total_damage = 0.0_f32;
    let mut total_count = 0_usize;

    for stage in 0..stage_count {
        if stage_counts[stage] > 0 {
            stage_means[stage] = stage_sums[stage] / stage_counts[stage] as f32;
            total_damage += stage_sums[stage];
            total_count += stage_counts[stage];
        }
    }

    let overall_damage_mean = if total_count > 0 {
        total_damage / total_count as f32
    } else {
        0.0
    };

    let below_15_frac = below_15_count as f32 / samples as f32;

    Ok(Some(BalanceStats {
        win_rate: total_wins as f32 / samples as f32,
        stage_means,
        stage_counts,
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
