use super::{Contract, ContractEffect, constants, effect_kinds::ContractEffectType};
use crate::{
    game_state::contract::{reward, risk},
    rarity::Rarity,
};
use rand::{RngCore, distributions::WeightedIndex, prelude::*, thread_rng};

pub fn generate_contract(rarity: Rarity) -> Contract {
    let mut rng = thread_rng();
    generate_contract_with_rng(&mut rng, rarity)
}

pub fn generate_contract_with_rng(rng: &mut dyn RngCore, rarity: Rarity) -> Contract {
    let duration_stages = generate_duration_stages_with_rng(rng);
    let (risk_type, reward_type) = generate_contract_effect_types_with_rng(rng);
    let risk = generate_contract_effect_with_rng(rng, risk_type, true, rarity, duration_stages);
    let reward =
        generate_contract_effect_with_rng(rng, reward_type, false, rarity, duration_stages);
    Contract::new(rarity, duration_stages, risk, reward)
}

fn generate_duration_stages_with_rng(rng: &mut dyn RngCore) -> usize {
    let dist = WeightedIndex::new(constants::STAGE_DURATION_WEIGHTS).unwrap();
    let index = dist.sample(rng);
    2 + index
}

fn generate_contract_effect_types_with_rng(
    rng: &mut dyn RngCore,
) -> (ContractEffectType, ContractEffectType) {
    let types = [
        ContractEffectType::OnSign,
        ContractEffectType::WhileActive,
        ContractEffectType::OnStageStart,
        ContractEffectType::OnExpire,
    ];
    let risk_type = *types.choose(rng).unwrap();
    let available_types: Vec<_> = types.into_iter().filter(|&t| t != risk_type).collect();
    let reward_type = *available_types.choose(rng).unwrap();
    (risk_type, reward_type)
}

fn generate_contract_effect_with_rng(
    rng: &mut dyn RngCore,
    effect_type: ContractEffectType,
    is_risk: bool,
    rarity: Rarity,
    duration_stages: usize,
) -> ContractEffect {
    let effect = if is_risk {
        risk::generate_risk_effect_with_rng(rng, &effect_type, rarity, duration_stages)
    } else {
        reward::generate_reward_effect_with_rng(rng, &effect_type, rarity, duration_stages)
    };
    match effect_type {
        ContractEffectType::OnSign => ContractEffect::OnSign { effect },
        ContractEffectType::WhileActive => ContractEffect::WhileActive { effect },
        ContractEffectType::OnStageStart => ContractEffect::OnStageStart { effect },
        ContractEffectType::OnExpire => ContractEffect::OnExpire { effect },
    }
}
