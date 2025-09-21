use super::OnStageStartEffectKind;

pub fn kinds() -> &'static [OnStageStartEffectKind] {
    &[
        OnStageStartEffectKind::LoseHealth,
        OnStageStartEffectKind::LoseGold,
    ]
}
