use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[
        OnExpireEffectKind::LoseGold,
        OnExpireEffectKind::LoseHealthOnContractEnd,
    ]
}
