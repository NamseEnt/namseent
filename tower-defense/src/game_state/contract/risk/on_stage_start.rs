use super::OnStageStartEffectKind;

pub fn kinds() -> &'static [OnStageStartEffectKind] {
    &[
        OnStageStartEffectKind::LoseHealthRange,
        OnStageStartEffectKind::LoseGoldRange,
    ]
}
