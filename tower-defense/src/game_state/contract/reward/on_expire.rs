use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[
        OnExpireEffectKind::HealHealthOnContractEnd,
        OnExpireEffectKind::GainGoldOnContractEnd,
        OnExpireEffectKind::GrantUpgrade,
        OnExpireEffectKind::GrantItem,
    ]
}
