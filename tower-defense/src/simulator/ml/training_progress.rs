use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Instant;

pub(crate) fn add_duration(total: &mut f64, started: Instant) {
    *total += started.elapsed().as_secs_f64();
}

pub(crate) struct PpoProgress {
    display: MultiProgress,
    iterations: ProgressBar,
    rollout: ProgressBar,
}

#[derive(Clone, Copy)]
pub(crate) struct RolloutTiming {
    pub(crate) train_seconds: f64,
    pub(crate) optimization_seconds: f64,
    pub(crate) validation_seconds: f64,
    pub(crate) train_decisions_per_second: f64,
    pub(crate) validation_decisions_per_second: f64,
}

impl PpoProgress {
    pub(crate) fn new(iterations: usize) -> Self {
        let display = MultiProgress::new();
        let iterations_bar = display.add(ProgressBar::new(iterations as u64));
        iterations_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] {msg} [{bar:32.cyan/blue}] {pos}/{len}",
            )
            .expect("static PPO progress template should be valid"),
        );
        let rollout_bar = display.add(ProgressBar::new(0));
        rollout_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} {msg} [{bar:32.yellow/blue}] {pos}/{len}",
            )
            .expect("static rollout progress template should be valid"),
        );
        Self {
            display,
            iterations: iterations_bar,
            rollout: rollout_bar,
        }
    }

    pub(crate) fn begin_iteration(&self, iteration: usize, total_iterations: usize) {
        self.iterations.set_message(format!(
            "PPO iteration {}/{}",
            iteration + 1,
            total_iterations
        ));
    }

    pub(crate) fn begin_rollout(&self, phase: &str, episodes: usize) {
        self.rollout.set_length(episodes as u64);
        self.rollout.set_position(0);
        self.rollout.set_message(phase.to_string());
        self.rollout.reset_eta();
    }

    pub(crate) fn set_rollout_message(&self, message: impl Into<String>) {
        self.rollout.set_message(message.into());
    }
    pub(crate) fn finish_rollout(&self) {
        self.rollout.finish_and_clear();
    }
    pub(crate) fn print_event(&self, message: &str) {
        eprintln!("{message}");
    }

    pub(crate) fn finish_iteration(
        &self,
        train_clear_rate: f64,
        validation_clear_rate: f64,
        timing: RolloutTiming,
    ) {
        self.iterations.set_message(format!(
            "train {:.1}% val {:.1}% r={:.1}s/{:.0}d/s o={:.1}s v={:.1}s/{:.0}d/s",
            train_clear_rate * 100.0,
            validation_clear_rate * 100.0,
            timing.train_seconds,
            timing.train_decisions_per_second,
            timing.optimization_seconds,
            timing.validation_seconds,
            timing.validation_decisions_per_second
        ));
        self.iterations.inc(1);
    }

    pub(crate) fn finish_validation(&self, clear_rate: f64) {
        self.iterations
            .set_message(format!("validation {:.1}%", clear_rate * 100.0));
        self.iterations.inc(1);
    }

    pub(crate) fn rollout_bar(&self) -> &ProgressBar {
        &self.rollout
    }

    pub(crate) fn finish(&self) {
        self.rollout.finish_and_clear();
        self.iterations.finish();
        self.display.clear().ok();
    }
}
