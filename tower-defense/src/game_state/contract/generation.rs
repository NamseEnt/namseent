use super::{Contract, ContractEffect, effect_kinds::ContractEffectType, reward, risk};
use crate::rarity::Rarity;
use rand::{distributions::WeightedIndex, prelude::*, thread_rng};

pub fn generate_contract(rarity: Rarity) -> Contract {
    let duration_stages = generate_duration_stages();

    let (risk_type, reward_type) = generate_contract_effect_types();
    let risk = generate_contract_effect(risk_type, true, rarity, duration_stages);
    let reward = generate_contract_effect(reward_type, false, rarity, duration_stages);

    Contract::new(rarity, duration_stages, risk, reward)
}

fn generate_duration_stages() -> usize {
    let weights = [8, 9, 5, 4, 3, 2, 1, 1, 1]; // for stages 2 to 10, average ~4.18
    let dist = WeightedIndex::new(weights).unwrap();
    let index = dist.sample(&mut thread_rng());
    2 + index
}

fn generate_contract_effect_types() -> (ContractEffectType, ContractEffectType) {
    let types = [
        ContractEffectType::OnSign,
        ContractEffectType::WhileActive,
        ContractEffectType::OnStageStart,
        ContractEffectType::OnExpire,
    ];
    let risk_type = *types.choose(&mut thread_rng()).unwrap();
    let available_types: Vec<_> = types.into_iter().filter(|&t| t != risk_type).collect();
    let reward_type = *available_types.choose(&mut thread_rng()).unwrap();
    (risk_type, reward_type)
}

fn generate_contract_effect(
    effect_type: ContractEffectType,
    is_risk: bool,
    rarity: Rarity,
    duration_stages: usize,
) -> ContractEffect {
    let effect = if is_risk {
        risk::generate_risk_effect(&effect_type, rarity)
    } else {
        reward::generate_reward_effect(&effect_type, rarity, duration_stages)
    };
    match effect_type {
        ContractEffectType::OnSign => ContractEffect::OnSign { effect },
        ContractEffectType::WhileActive => ContractEffect::WhileActive { effect },
        ContractEffectType::OnStageStart => ContractEffect::OnStageStart { effect },
        ContractEffectType::OnExpire => ContractEffect::OnExpire { effect },
    }
}
