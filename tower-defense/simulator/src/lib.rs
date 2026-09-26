//! Headless game simulation and machine-learning tooling.

pub mod config;
pub use config::GameConfig;
pub mod benchmark;
pub mod core;
#[allow(deprecated)]
pub use core::{
    CommandReceipt, CoreTickOutput, DefenseEndOutput, DefenseEndTransition, FastTickOutput,
    GameCore, RecordedTickOutput, StepOutput, TickTransition,
};
pub mod environment;
pub mod events;
#[cfg(feature = "simulator")]
pub mod hp_balance;
#[cfg(feature = "simulator")]
pub mod joint_action;
#[cfg(feature = "simulator")]
pub mod legality;
pub mod ml;
#[cfg(feature = "diagnostics")]
pub mod placement_diag;
#[cfg(feature = "simulator")]
pub mod play;
#[cfg(feature = "simulator")]
pub mod policy_action;
#[cfg(feature = "simulator")]
pub mod policy_runner;
pub mod recording;
pub mod stats;
#[cfg(feature = "simulator")]
pub mod teacher;
#[cfg(feature = "simulator")]
pub mod teacher_eval;
pub mod teacher_reroll_diag;
pub mod teacher_selection;
#[cfg(test)]
pub mod teacher_selection_phase3n_fixture;
pub mod teacher_terminal_gate;
pub mod trajectory;

pub(crate) fn canonicalize_kind_name(name: String) -> String {
    let trimmed = name.trim();
    let token: String = trimmed
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    let token = token.strip_suffix("Upgrade").unwrap_or(&token);
    token.to_string()
}
