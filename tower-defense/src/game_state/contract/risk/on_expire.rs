use super::OnExpireEffectKind;

pub fn kinds() -> &'static [OnExpireEffectKind] {
    &[OnExpireEffectKind::LoseHealth, OnExpireEffectKind::LoseGold]
}
