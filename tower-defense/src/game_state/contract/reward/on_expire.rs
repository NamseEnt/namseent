use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[
        OnExpireEffectKind::HealHealthOnContractEnd,
        OnExpireEffectKind::GainGold,
        OnExpireEffectKind::GrantUpgrade,
        OnExpireEffectKind::GrantItem,
    ]
}
