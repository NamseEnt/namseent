use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use tower_defense::set_headless;
use tower_defense::simulator::HeadlessGame;
use tower_defense::simulator::recording::SimRecorder;
use tower_defense::simulator::strategies::TowerPlacementStrategy;
use tower_defense::simulator::strategies::{
    card_reroll::{NoRerollStrategy, OptimalRerollStrategy},
    item_use::{DefaultItemUseStrategy, NoItemUseStrategy},
    shop::{BuyCheapestStrategy, NoBuyStrategy},
    tower_placement::SpiralPlacementStrategy,
};

struct ThreadStats {
    samples: usize,
    clear_rate_sum: f32,
    last_clear_rate: f32,
}

impl ThreadStats {
    fn record(&mut self, clear_rate: f32) {
        self.samples += 1;
        self.clear_rate_sum += clear_rate;
        self.last_clear_rate = clear_rate;
    }

    fn average_clear_rate(&self) -> f32 {
        if self.samples == 0 {
            0.0
        } else {
            self.clear_rate_sum / self.samples as f32
        }
    }
}

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    set_headless(true);

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

    let progress = MultiProgress::new();
    let overall_pb = progress.add(ProgressBar::new(cli.samples as u64));
    overall_pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
        )
        .unwrap(),
    );
    overall_pb.set_message("wins 0");

    let num_threads = pool.current_num_threads();
    let thread_stats = Arc::new(
        (0..num_threads)
            .map(|_| {
                Mutex::new(ThreadStats {
                    samples: 0,
                    clear_rate_sum: 0.0,
                    last_clear_rate: 0.0,
                })
            })
            .collect::<Vec<_>>(),
    );

    let thread_pbs = Arc::new(
        (0..num_threads)
            .map(|idx| {
                let pb = progress.add(ProgressBar::new(100));
                pb.set_style(
                    ProgressStyle::with_template("{prefix} [{bar:40.green/black}] {pos}% {msg}")
                        .unwrap(),
                );
                pb.set_prefix(format!("T{idx}"));
                pb.set_message("idle");
                pb
            })
            .collect::<Vec<_>>(),
    );

    progress.println(format!(
        "Running {} simulations on {} threads...",
        cli.samples, num_threads
    ))?;

    pool.install(|| {
        (0..cli.samples).into_par_iter().for_each(|i| {
            let mut rng = rand::thread_rng();
            let seed: u64 = rand::Rng::r#gen(&mut rng);

            let shop_strategy: Box<dyn tower_defense::simulator::strategies::ShopStrategy> =
                if i % 2 == 0 {
                    Box::new(NoBuyStrategy)
                } else {
                    Box::new(BuyCheapestStrategy)
                };
            let card_strategy: Box<dyn tower_defense::simulator::strategies::CardRerollStrategy> =
                if i % 2 == 0 {
                    Box::new(NoRerollStrategy)
                } else {
                    Box::new(OptimalRerollStrategy)
                };
            let tower_strategy = SpiralPlacementStrategy;
            let item_strategy: Box<dyn tower_defense::simulator::strategies::ItemUseStrategy> =
                if i % 2 == 0 {
                    Box::new(NoItemUseStrategy)
                } else {
                    Box::new(DefaultItemUseStrategy)
                };

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

            let thread_index = rayon::current_thread_index().unwrap_or(0);
            if let Some(thread_pb) = thread_pbs.get(thread_index) {
                thread_pb.set_message(format!("sample {i} running..."));
            }

            let mut game = HeadlessGame::new();
            let result = game.run(
                shop_strategy.as_ref(),
                card_strategy.as_ref(),
                &tower_strategy,
                item_strategy.as_ref(),
                &mut rng,
                |clear_rate| {
                    if let Some(thread_pb) = thread_pbs.get(thread_index) {
                        let clear_pos = clear_rate.clamp(0.0, 100.0) as u64;
                        thread_pb.set_position(clear_pos);
                        thread_pb.set_message(format!("sample {i} clear {clear_rate:.1}%",));
                    }
                },
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

            let thread_index = rayon::current_thread_index().unwrap_or(0);
            if let Some(thread_pb) = thread_pbs.get(thread_index) {
                let mut stats = thread_stats[thread_index].lock().unwrap();
                stats.record(result.clear_rate);
                let clear_pos = result.clear_rate.clamp(0.0, 100.0) as u64;
                thread_pb.set_position(clear_pos);
                thread_pb.set_message(format!(
                    "sample {i} done, avg {avg:.1}%",
                    avg = stats.average_clear_rate(),
                ));
            }

            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            overall_pb.inc(1);
            let win_count = victories.load(Ordering::Relaxed);
            overall_pb.set_message(format!(
                "wins {win_count}, last {last:.1}%",
                last = result.clear_rate
            ));

            if done % 100 == 0 || done == cli.samples {
                eprintln!(
                    "[{done}/{total}] Win rate: {rate:.1}%",
                    total = cli.samples,
                    rate = win_count as f64 / done as f64 * 100.0,
                );
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
    println!("Results saved to: {}", cli.db.display());

    Ok(())
}
