use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use tower_defense::config::GameConfig;
use tower_defense::set_headless;
use tower_defense::simulator::HeadlessGame;
use tower_defense::simulator::recording::SimRecorder;
use tower_defense::simulator::stats::Database;
use tower_defense::simulator::strategies::TowerPlacementStrategy;
use tower_defense::simulator::strategies::treasure::RandomTreasureStrategy;
use tower_defense::simulator::strategies::{
    card_reroll::{NoRerollStrategy, OptimalRerollStrategy},
    item_use::{DefaultItemUseStrategy, HeuristicItemUseStrategy},
    shop::{BuyCheapestStrategy, HeuristicShopStrategy},
    tower_placement::HeuristicPlacementStrategy,
};

#[derive(Parser)]
#[command(
    name = "td-simulator",
    about = "Tower Defense headless balance simulator"
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

    /// Optional simulation config TOML file
    #[arg(long)]
    config: Option<PathBuf>,

    /// Print per-round damage distribution statistics
    #[arg(long)]
    damage_distribution: bool,

    /// Print strategy win rate statistics after simulation
    #[arg(long)]
    strategy_stats: bool,

    /// Suppress progress bar output
    #[arg(long)]
    quiet: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    set_headless(true);

    let config = if let Some(path) = &cli.config {
        Arc::new(GameConfig::from_toml(path)?)
    } else {
        Arc::new(GameConfig::default_config())
    };

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
    let completed = AtomicUsize::new(0);
    let victories = AtomicUsize::new(0);

    let stage_damage_by_round = if cli.damage_distribution {
        Some(Arc::new(Mutex::new(HashMap::<usize, Vec<f32>>::new())))
    } else {
        None
    };

    let num_threads = pool.current_num_threads();
    if !cli.quiet {
        println!(
            "Running {} simulations on {} threads...",
            cli.samples, num_threads
        );
    }

    let overall_pb = if cli.quiet {
        None
    } else {
        let pb = ProgressBar::new(cli.samples as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
            )
            .unwrap(),
        );
        pb.set_message("wins 0");
        Some(pb)
    };

    pool.install(|| {
        (0..cli.samples).into_par_iter().for_each(|_i| {
            let mut rng = rand::thread_rng();
            let seed: u64 = rand::Rng::r#gen(&mut rng);

            let shop_strategy: Box<dyn tower_defense::simulator::strategies::ShopStrategy> =
                if rng.gen_bool(0.5) {
                    Box::new(HeuristicShopStrategy)
                } else {
                    Box::new(BuyCheapestStrategy)
                };
            let card_strategy: Box<dyn tower_defense::simulator::strategies::CardRerollStrategy> =
                if rng.gen_bool(0.5) {
                    Box::new(NoRerollStrategy)
                } else {
                    Box::new(OptimalRerollStrategy)
                };
            let tower_strategy = HeuristicPlacementStrategy;
            let item_strategy: Box<dyn tower_defense::simulator::strategies::ItemUseStrategy> =
                if rng.gen_bool(0.5) {
                    Box::new(HeuristicItemUseStrategy)
                } else {
                    Box::new(DefaultItemUseStrategy)
                };
            let treasure_strategy = RandomTreasureStrategy;

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
                return;
            }

            let mut game = HeadlessGame::new_with_config(config.clone());
            let result = game.run(
                shop_strategy.as_ref(),
                card_strategy.as_ref(),
                &tower_strategy,
                item_strategy.as_ref(),
                &treasure_strategy,
                &mut rng,
                |_clear_rate| {},
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

            if let Some(stage_damage_by_round) = &stage_damage_by_round {
                let mut map = stage_damage_by_round.lock().unwrap();
                for (idx, &damage) in result.stage_damage.iter().enumerate() {
                    map.entry(idx + 1).or_default().push(damage);
                }
            }

            completed.fetch_add(1, Ordering::Relaxed);
            if let Some(pb) = &overall_pb {
                pb.inc(1);
                let win_count = victories.load(Ordering::Relaxed);
                pb.set_message(format!(
                    "wins {win_count}, last {last:.1}%",
                    last = result.clear_rate
                ));
            }
        });
    });

    let total_wins = victories.load(Ordering::Relaxed);
    println!("=== Simulation Complete ===");
    println!("Samples: {}", cli.samples);
    println!(
        "Win rate: {:.1}% ({}/{})",
        total_wins as f64 / cli.samples as f64 * 100.0,
        total_wins,
        cli.samples
    );

    if let Some(stage_damage_by_round) = stage_damage_by_round {
        let map = stage_damage_by_round.lock().unwrap();
        if !map.is_empty() {
            println!("=== Damage Distribution by Round ===");
            let mut stages: Vec<usize> = map.keys().cloned().collect();
            stages.sort_unstable();
            for stage in stages {
                let damages = &map[&stage];
                if damages.is_empty() {
                    continue;
                }
                let count = damages.len();
                let sum: f32 = damages.iter().sum();
                let mean = sum / count as f32;
                let mut sorted = damages.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let median = if count % 2 == 1 {
                    sorted[count / 2]
                } else {
                    (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
                };
                let min = *sorted.first().unwrap_or(&0.0);
                let max = *sorted.last().unwrap_or(&0.0);
                println!(
                    "Round {stage}: count {count}, mean {mean:.1}, median {median:.1}, min {min:.1}, max {max:.1}",
                );
            }
        }
    }

    if cli.strategy_stats {
        let db = Database::open(&cli.db)?;
        let strategy_rows = db.list_strategy_win_rates()?;
        if !strategy_rows.is_empty() {
            println!("=== Strategy Win Rates ===");
            let mut current_category = String::new();
            for row in strategy_rows {
                if row.category != current_category {
                    if !current_category.is_empty() {
                        println!();
                    }
                    current_category = row.category.clone();
                    println!("{}:", current_category);
                }
                println!(
                    "  {:<16} {:>4}/{:<4} wins  ({:.1}%)  avg_clear {:.1}%  var {:.2}",
                    row.name,
                    row.win_count,
                    row.sample_count,
                    row.win_rate * 100.0,
                    row.avg_clear_rate,
                    row.clear_rate_variance,
                );
            }
        }
    }

    println!("Results saved to: {}", cli.db.display());

    Ok(())
}
