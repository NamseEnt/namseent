use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurriculumConfig {
    pub initial_max_stage: usize,
    pub final_max_stage: usize,
    pub promotion_threshold: f32,
    pub promotion_patience: usize,
    pub min_iterations_per_level: usize,
}

impl Default for CurriculumConfig {
    fn default() -> Self {
        Self {
            initial_max_stage: 1,
            final_max_stage: 1,
            promotion_threshold: 0.8,
            promotion_patience: 3,
            min_iterations_per_level: 5,
        }
    }
}

impl CurriculumConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.initial_max_stage == 0 || self.final_max_stage == 0 {
            return Err("curriculum stages must be positive".to_string());
        }
        if self.initial_max_stage > self.final_max_stage {
            return Err("initial curriculum stage exceeds final stage".to_string());
        }
        if !(0.0..=1.0).contains(&self.promotion_threshold) {
            return Err("curriculum promotion threshold must be between 0 and 1".to_string());
        }
        if self.promotion_patience == 0 || self.min_iterations_per_level == 0 {
            return Err("curriculum patience and minimum iterations must be positive".to_string());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurriculumState {
    pub current_max_stage: usize,
    pub consecutive_successes: usize,
    pub iterations_at_level: usize,
    pub promotion_iteration: Option<usize>,
    pub completed: bool,
    pub completion_count: usize,
}

impl CurriculumState {
    pub fn new(config: &CurriculumConfig) -> Result<Self, String> {
        config.validate()?;
        Ok(Self {
            current_max_stage: config.initial_max_stage,
            ..Self::default()
        })
    }

    pub fn observe_iteration(
        &mut self,
        config: &CurriculumConfig,
        success_rate: f32,
        iteration: usize,
    ) {
        if self.completed {
            return;
        }
        self.iterations_at_level += 1;
        if success_rate >= config.promotion_threshold {
            self.consecutive_successes += 1;
        } else {
            self.consecutive_successes = 0;
        }
        if self.current_max_stage >= config.final_max_stage {
            self.completed = true;
        } else if self.iterations_at_level >= config.min_iterations_per_level
            && self.consecutive_successes >= config.promotion_patience
        {
            self.current_max_stage += 1;
            self.iterations_at_level = 0;
            self.consecutive_successes = 0;
            self.promotion_iteration = Some(iteration);
            if self.current_max_stage >= config.final_max_stage {
                self.completed = true;
                self.completion_count = 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_stage_one_completes_once() {
        let config = CurriculumConfig::default();
        let mut state = CurriculumState::new(&config).expect("valid config");
        state.observe_iteration(&config, 1.0, 0);
        assert!(state.completed);
        let snapshot = state.clone();
        state.observe_iteration(&config, 0.0, 1);
        assert_eq!(state, snapshot);
    }

    #[test]
    fn promotion_requires_patience_and_minimum_iterations() {
        let config = CurriculumConfig {
            final_max_stage: 3,
            promotion_patience: 2,
            min_iterations_per_level: 2,
            ..CurriculumConfig::default()
        };
        let mut state = CurriculumState::new(&config).expect("valid config");
        state.observe_iteration(&config, 1.0, 0);
        assert_eq!(state.current_max_stage, 1);
        state.observe_iteration(&config, 1.0, 1);
        assert_eq!(state.current_max_stage, 2);
    }
}
