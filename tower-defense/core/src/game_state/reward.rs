use crate::Observation;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RewardComponents {
    pub terminal: f32,
    pub shaping: std::collections::BTreeMap<String, f32>,
}

impl Default for RewardComponents {
    fn default() -> Self {
        Self {
            terminal: 0.0,
            shaping: std::collections::BTreeMap::new(),
        }
    }
}

impl RewardComponents {
    pub fn total(&self) -> f32 {
        self.terminal + self.shaping.values().sum::<f32>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StepReason {
    DecisionPoint,
    Terminal,
    MaxTicks,
    MaxDecisions,
    NoProgressCycle,
    CurriculumComplete,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StepInfo {
    pub reason: StepReason,
    pub ticks_advanced: u64,
    pub no_progress_cycle: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RewardConfig {
    pub terminal_win: f32,
    pub terminal_loss: f32,
    pub escaped_hp_penalty_scale: f32,
    pub player_hp_loss_penalty_scale: f32,
    pub potential_weight: f32,
    pub potential_gamma: f32,
    #[serde(default)]
    pub damage_progress_weight: f32,
    pub no_progress_cycle_penalty: f32,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StepOutcome {
    pub observation: Observation,
    pub reward: RewardComponents,
    pub terminated: bool,
    pub truncated: bool,
    pub info: StepInfo,
    pub state_hash: String,
}
