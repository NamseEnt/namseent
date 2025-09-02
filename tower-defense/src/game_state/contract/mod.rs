use crate::game_state::GameState;
use crate::game_state::effect::Effect;
use crate::rarity::Rarity;
use std::fmt::Display;
use std::sync::atomic::AtomicU64;

#[derive(Clone, Debug)]
pub enum ContractEffect {
    OnSign { effect: Effect },
    WhileActive { effect: Effect },
    OnStageStart { effect: Effect },
    OnExpire { effect: Effect },
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ContractStatus {
    Pending { duration_stages: usize },
    Active { remaining_stages: usize },
    Expired,
}
impl Display for ContractStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = match self {
            ContractStatus::Pending { duration_stages: n }
            | ContractStatus::Active {
                remaining_stages: n,
            } => *n,
            ContractStatus::Expired => 0,
        };
        write!(f, "S-{n}")
    }
}

#[derive(Clone, Debug)]
pub struct Contract {
    pub id: u64,
    pub rarity: Rarity,
    pub status: ContractStatus,
    pub risk: ContractEffect,
    pub reward: ContractEffect,
}
impl Contract {
    pub fn new(
        rarity: Rarity,
        duration_stages: usize,
        risk: ContractEffect,
        reward: ContractEffect,
    ) -> Self {
        static ID: AtomicU64 = AtomicU64::new(1);
        Contract {
            id: ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            rarity,
            status: ContractStatus::Pending { duration_stages },
            risk,
            reward,
        }
    }

    pub fn on_sign_effects(&self) -> Vec<&Effect> {
        [&self.reward, &self.risk]
            .iter()
            .filter_map(|contract_effect| {
                if let ContractEffect::OnSign { effect } = contract_effect {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn while_active_effects(&self) -> Vec<&Effect> {
        [&self.reward, &self.risk]
            .iter()
            .filter_map(|contract_effect| {
                if let ContractEffect::WhileActive { effect } = contract_effect {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn on_stage_start_effects(&self) -> Vec<&Effect> {
        [&self.reward, &self.risk]
            .iter()
            .filter_map(|contract_effect| {
                if let ContractEffect::OnStageStart { effect } = contract_effect {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn on_expire_effects(&self) -> Vec<&Effect> {
        [&self.reward, &self.risk]
            .iter()
            .filter_map(|contract_effect| {
                if let ContractEffect::OnExpire { effect } = contract_effect {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ContractState {
    // TODO: like upgrade state
}

#[allow(dead_code)]
pub fn sign_contract(game_state: &mut GameState, contract: Contract) {
    contract.on_sign_effects().iter().for_each(|effect| {
        run_effect(game_state, effect);
    });

    game_state.contracts.push(contract);
}

#[allow(dead_code)]
fn run_effect(_game_state: &mut GameState, _effect: &Effect) {
    // TODO
}
