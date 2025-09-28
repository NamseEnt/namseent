pub mod types;
pub mod on_sign;
pub mod while_active;
pub mod on_stage_start;
pub mod on_expire;

use rand::prelude::*;
use crate::{rarity::Rarity, game_state::effect::Effect};
use super::effect_kinds::ContractEffectType;
use types::RewardGeneratorFn;

pub(crate) fn generate_reward_effect_with_rng(
    rng: &mut dyn rand::RngCore,
    effect_type: &ContractEffectType,
    rarity: Rarity,
    duration_stages: usize,
) -> Effect {
    let list: &[RewardGeneratorFn] = match effect_type {
        ContractEffectType::OnSign => on_sign::list(),
        ContractEffectType::WhileActive => while_active::list(),
        ContractEffectType::OnStageStart => on_stage_start::list(),
        ContractEffectType::OnExpire => on_expire::list(),
    };
    let generator = list.choose(rng).expect("reward effect list non-empty");
    generator(rng, rarity, duration_stages)
}
