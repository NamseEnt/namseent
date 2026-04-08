use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tower_defense::config::GameConfig;
use tower_defense::set_headless;
use tower_defense::simulator::HeadlessGame;
use tower_defense::simulator::recording::SimRecorder;
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

    /// Number of threads (0 = auto-detect)
    #[arg(short, long, default_value_t = 0)]
    threads: usize,

    /// Optional simulation config TOML file
    #[arg(long)]
    config: Option<PathBuf>,
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

    let recorder = Arc::new(SimRecorder::new(&cli.db)?);
    let completed = AtomicUsize::new(0);
    let victories = AtomicUsize::new(0);

    let num_threads = pool.current_num_threads();
    println!(
        "Running {} simulations on {} threads...",
        cli.samples, num_threads
    );

    let overall_pb = ProgressBar::new(cli.samples as u64);
    overall_pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )
        .unwrap(),
    );
    overall_pb.set_message("wins 0");

    pool.install(|| {
        (0..cli.samples).into_par_iter().for_each(|i| {
            let mut rng = rand::thread_rng();
            let seed: u64 = rand::Rng::r#gen(&mut rng);

            let shop_strategy: Box<dyn tower_defense::simulator::strategies::ShopStrategy> =
                if i % 2 == 0 {
                    Box::new(HeuristicShopStrategy)
                } else {
                    Box::new(BuyCheapestStrategy)
                };
            let card_strategy: Box<dyn tower_defense::simulator::strategies::CardRerollStrategy> =
                if i % 2 == 0 {
                    Box::new(NoRerollStrategy)
                } else {
                    Box::new(OptimalRerollStrategy)
                };
            let tower_strategy = HeuristicPlacementStrategy;
            let item_strategy: Box<dyn tower_defense::simulator::strategies::ItemUseStrategy> =
                if i % 2 == 0 {
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

            completed.fetch_add(1, Ordering::Relaxed);
            overall_pb.inc(1);
            let win_count = victories.load(Ordering::Relaxed);
            overall_pb.set_message(format!(
                "wins {win_count}, last {last:.1}%",
                last = result.clear_rate
            ));
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
    println!("Results saved to: {}", cli.db.display());

    Ok(())
}
