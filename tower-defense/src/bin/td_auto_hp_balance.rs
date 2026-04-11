use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
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

    let tuned_config = tune_hp_balance(&pool, base_config, &cli, recorder)?;

    tuned_config.write_toml(&cli.output)?;
    println!("Saved tuned config to {}", cli.output.display());

    Ok(())
}

fn tune_hp_balance(
    pool: &rayon::ThreadPool,
    mut config: GameConfig,
    cli: &Cli,
    recorder: Arc<SimRecorder>,
) -> anyhow::Result<GameConfig> {
    let stages = config.player.max_stages;
    let mut stage_primary = build_stage_primary_monster(&config);

    let steps = [
        (cli.samples.saturating_div(10).max(1), 0.10, 8),
        (cli.samples.saturating_div(5).max(1), 0.05, 8),
        (cli.samples.max(1), 0.02, 6),
    ];

    for (step_index, (samples, max_delta, max_iters)) in steps.into_iter().enumerate() {
        let step_name = step_index + 1;
        println!(
            "=== Step {step_name}: samples={} max_delta={:.2}% ===",
            samples,
            max_delta * 100.0
        );

        for iteration in 1..=max_iters {
            let metrics = run_simulations(
                pool,
                Arc::new(config.clone()),
                samples,
                &recorder,
                cli.quiet,
            )?;

            let overall_mean = metrics.overall_damage_mean;
            let target_message = format!(
                "win {:.1}%, below15 {:.1}%, mean {:.1}",
                metrics.win_rate * 100.0,
                metrics.below_15_frac * 100.0,
                overall_mean
            );
            println!("Step {step_name} iter {iteration}: {target_message}");

            let global_scale = ((metrics.win_rate - 0.10) * 0.2).clamp(-0.08, 0.08) + 1.0;
            let early_ease = if metrics.below_15_frac > 0.50 {
                ((metrics.below_15_frac - 0.50) * 0.20).min(0.08)
            } else {
                0.0
            };

            let mut max_change: f32 = 0.0;

            for stage in 1..=stages {
                if let Some(monster_kind) = stage_primary.get(stage).copied().flatten() {
                    let count = metrics.stage_counts[stage - 1];
                    if count == 0 {
                        continue;
                    }

                    let observed = metrics.stage_means[stage - 1];
                    if observed <= 0.0 {
                        continue;
                    }

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
                    let target_mean = (prev_mean + observed + next_mean) / 3.0;

                    let mut stage_scale = (target_mean / observed)
                        .clamp(1.0 - max_delta * 1.2, 1.0 + max_delta * 1.2);
                    stage_scale *= global_scale;

                    if stage <= 15 {
                        stage_scale *= 1.0 - early_ease;
                    }

                    stage_scale = stage_scale.clamp(1.0 - max_delta * 1.2, 1.0 + max_delta * 1.2);

                    if let Some(stat) = config.monsters.stats.get_mut(&monster_kind) {
                        let new_hp = stat.base_hp * stage_scale;
                        let applied_scale = (new_hp / stat.base_hp)
                            .clamp(1.0 - max_delta * 1.5, 1.0 + max_delta * 1.5);
                        stat.base_hp *= applied_scale;
                        max_change = max_change.max((applied_scale - 1.0).abs());
                    }
                }
            }

            if max_change < 1e-4 {
                println!("Step {step_name} converged at iteration {iteration}");
                break;
            }

            if (0.095..=0.105).contains(&metrics.win_rate) && metrics.below_15_frac <= 0.40 {
                println!("Step {step_name} reached target metrics and will stop early");
                break;
            }
        }

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
) -> anyhow::Result<BalanceStats> {
    let stage_count = config.player.max_stages;
    let stage_sums = Arc::new(Mutex::new(vec![0.0_f32; stage_count]));
    let stage_counts = Arc::new(Mutex::new(vec![0_usize; stage_count]));
    let below_15_count = AtomicUsize::new(0);
    let victories = AtomicUsize::new(0);

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
                |_| {},
            );

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

    Ok(BalanceStats {
        win_rate: total_wins as f32 / samples as f32,
        stage_means,
        stage_counts,
        overall_damage_mean,
        below_15_frac,
    })
}
