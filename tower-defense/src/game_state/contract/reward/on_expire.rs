use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[
        OnExpireEffectKind::HealHealthOnContractEnd,
        OnExpireEffectKind::GainGoldOnContractEnd,
        OnExpireEffectKind::GrantUpgradeOnContractEnd,
        OnExpireEffectKind::GrantItemOnContractEnd,
    ]
}
