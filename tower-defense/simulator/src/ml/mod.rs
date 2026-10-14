#[cfg(feature = "simulator")]
pub mod bc;
#[cfg(feature = "simulator")]
pub mod cli;
#[cfg(all(test, feature = "simulator"))]
mod construction_context_study;
pub mod contract;
#[cfg(feature = "simulator")]
pub mod curriculum;
#[cfg(feature = "simulator")]
pub mod dataset;
#[cfg(feature = "simulator")]
pub mod diagnostics;
#[cfg(feature = "simulator")]
pub mod encoding;
#[cfg(feature = "simulator")]
pub mod feature_contract;
#[cfg(feature = "simulator")]
pub mod features;
#[cfg(feature = "simulator")]
pub mod model;
#[cfg(feature = "simulator")]
pub mod neural_checkpoint;
#[cfg(feature = "simulator")]
pub mod phase4_analysis;
#[cfg(feature = "simulator")]
pub mod phase4_cli;
#[cfg(feature = "simulator")]
pub mod phase4_dataset;
#[cfg(feature = "simulator")]
pub mod phase4_eval;
#[cfg(feature = "simulator")]
pub mod policy_v2;
#[cfg(feature = "simulator")]
pub mod ppo;
#[cfg(feature = "simulator")]
pub mod retraining;
#[cfg(feature = "simulator")]
pub mod rollout;
pub mod seed;
#[cfg(feature = "simulator")]
pub mod semantic_bc;
#[cfg(feature = "simulator")]
pub mod semantic_candidates;
#[cfg(feature = "simulator")]
pub mod semantic_ppo;
#[cfg(feature = "simulator")]
pub mod spatial;
#[cfg(feature = "simulator")]
pub mod toy_overfit;
#[cfg(feature = "simulator")]
pub mod trainer_checkpoint;
#[cfg(feature = "simulator")]
pub(crate) mod training_progress;
#[cfg(feature = "simulator")]
pub mod validation;
#[cfg(feature = "simulator")]
pub mod vocabulary;

pub use contract::{ConditionedBalanceParameter, MlContract, MlContractError, PolicyBalanceScope};
pub use seed::{SeedRange, SeedRangeError, TrainingSeedSchedule};

#[cfg(feature = "simulator")]
pub mod effect_inputs;

#[cfg(feature = "simulator")]
pub mod content_teacher;

#[cfg(feature = "simulator")]
pub mod card_service_inputs;

#[cfg(all(test, feature = "experimental-content"))]
mod experimental_content_tests;
