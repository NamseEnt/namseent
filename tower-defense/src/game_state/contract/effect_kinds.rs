#[derive(Clone, Copy, PartialEq)]
pub enum ContractEffectType {
    OnSign,
    WhileActive,
    OnStageStart,
    OnExpire,
}
