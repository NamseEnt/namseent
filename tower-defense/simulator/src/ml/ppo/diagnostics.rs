use super::NeuralCheckpoint;
use super::PpoTrainingRun;

#[derive(Clone, Copy)]
pub(crate) struct InitialBestState {
    pub(crate) validation_clear_rate: f64,
    pub(crate) iteration: usize,
    pub(crate) full_clear_count: usize,
    pub(crate) truncated_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BestScore {
    pub(crate) full_clear_count: usize,
    pub(crate) clear_rate: f64,
    pub(crate) truncated_count: usize,
}

impl BestScore {
    pub(crate) fn is_better_than(self, other: Self) -> bool {
        (
            self.full_clear_count,
            self.clear_rate,
            usize::MAX - self.truncated_count,
        ) > (
            other.full_clear_count,
            other.clear_rate,
            usize::MAX - other.truncated_count,
        )
    }
}

pub(crate) fn best_score_from_checkpoint(checkpoint: &NeuralCheckpoint) -> BestScore {
    BestScore {
        full_clear_count: checkpoint.best_validation_full_clear_count,
        clear_rate: checkpoint.best_validation_clear_rate,
        truncated_count: checkpoint.best_validation_truncated_count,
    }
}

pub(crate) fn best_score_from_run(run: &PpoTrainingRun) -> BestScore {
    BestScore {
        full_clear_count: run.best_validation_full_clear_count,
        clear_rate: run.best_validation_clear_rate,
        truncated_count: run.best_validation_truncated_count,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OptimizationDiagnostics {
    pub policy_loss: f32,
    pub value_loss: f32,
    pub entropy: f32,
    pub approximate_kl: f32,
    pub clip_fraction: f32,
    pub gradient_norm: f32,
    pub update_count: usize,
    pub sample_count: usize,
    pub minibatch_count: usize,
    pub nonfinite_count: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct OptimizationDiagnosticsAccumulator {
    policy_loss_sum: f64,
    value_loss_sum: f64,
    entropy_sum: f64,
    approximate_kl_sum: f64,
    clip_fraction_sum: f64,
    gradient_norm_sum: f64,
    pub(crate) update_count: usize,
    pub(crate) sample_count: usize,
    pub(crate) minibatch_count: usize,
    pub(crate) nonfinite_count: usize,
}

impl OptimizationDiagnosticsAccumulator {
    pub(crate) fn record_nonfinite(&mut self) {
        self.nonfinite_count += 1;
    }

    pub(crate) fn record(
        &mut self,
        metrics: OptimizationMinibatchMetrics,
        sample_count: usize,
        gradient_norm: f32,
    ) {
        let weight = sample_count as f64;
        self.policy_loss_sum += metrics.policy_loss as f64 * weight;
        self.value_loss_sum += metrics.value_loss as f64 * weight;
        self.entropy_sum += metrics.entropy as f64 * weight;
        self.approximate_kl_sum += metrics.approximate_kl as f64 * weight;
        self.clip_fraction_sum += metrics.clip_fraction as f64 * weight;
        self.gradient_norm_sum += gradient_norm as f64 * weight;
        self.update_count += 1;
        self.sample_count += sample_count;
        self.minibatch_count += 1;
        if !metrics.is_finite() || !gradient_norm.is_finite() {
            self.nonfinite_count += 1;
        }
    }

    pub(crate) fn finish(self) -> OptimizationDiagnostics {
        let divisor = self.sample_count.max(1) as f64;
        OptimizationDiagnostics {
            policy_loss: (self.policy_loss_sum / divisor) as f32,
            value_loss: (self.value_loss_sum / divisor) as f32,
            entropy: (self.entropy_sum / divisor) as f32,
            approximate_kl: (self.approximate_kl_sum / divisor) as f32,
            clip_fraction: (self.clip_fraction_sum / divisor) as f32,
            gradient_norm: (self.gradient_norm_sum / divisor) as f32,
            update_count: self.update_count,
            sample_count: self.sample_count,
            minibatch_count: self.minibatch_count,
            nonfinite_count: self.nonfinite_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct OptimizationMinibatchMetrics {
    pub(crate) policy_loss: f32,
    pub(crate) value_loss: f32,
    pub(crate) entropy: f32,
    pub(crate) approximate_kl: f32,
    pub(crate) clip_fraction: f32,
}

impl OptimizationMinibatchMetrics {
    pub(crate) fn is_finite(self) -> bool {
        self.policy_loss.is_finite()
            && self.value_loss.is_finite()
            && self.entropy.is_finite()
            && self.approximate_kl.is_finite()
            && self.clip_fraction.is_finite()
    }
}
