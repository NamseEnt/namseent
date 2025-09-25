use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[
        OnExpireEffectKind::LoseHealthOnContractEnd,
        OnExpireEffectKind::LoseGoldOnContractEnd,
    ]
}
