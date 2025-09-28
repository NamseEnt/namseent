pub mod on_expire;
pub mod on_sign;
pub mod on_stage_start;
pub mod types;
pub mod while_active;

use super::effect_kinds::ContractEffectType;
use crate::{game_state::effect::Effect, rarity::Rarity};
use rand::prelude::*;
use types::RiskGeneratorFn;

pub(crate) fn generate_risk_effect_with_rng(
    rng: &mut dyn rand::RngCore,
    effect_type: &ContractEffectType,
    rarity: Rarity,
    duration_stages: usize,
) -> Effect {
    let list: &[RiskGeneratorFn] = match effect_type {
        ContractEffectType::OnSign => on_sign::list(),
        ContractEffectType::WhileActive => while_active::list(),
        ContractEffectType::OnStageStart => on_stage_start::list(),
        ContractEffectType::OnExpire => on_expire::list(),
    };
    let generator = list.choose(rng).expect("risk effect list non-empty");
    generator(rng, rarity, duration_stages)
}
