use crate::*;

#[derive(Clone, Copy, PartialEq, State)]
pub enum ContractEffectType {
    OnSign,
    WhileActive,
    OnStageStart,
    OnExpire,
}
