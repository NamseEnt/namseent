use super::OnSignEffectKind;

pub fn kinds() -> &'static [OnSignEffectKind] {
    &[
        OnSignEffectKind::HealHealth,
        OnSignEffectKind::GainGold,
        OnSignEffectKind::GrantUpgrade,
        OnSignEffectKind::GrantItem,
    ]
}
