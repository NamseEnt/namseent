use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[
        OnExpireEffectKind::HealHealth,
        OnExpireEffectKind::GainGold,
        OnExpireEffectKind::GrantUpgrade,
        OnExpireEffectKind::GrantItem,
    ]
}
