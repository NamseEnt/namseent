use super::OnStageStartEffectKind;

pub fn kinds() -> &'static [OnStageStartEffectKind] {
    &[
        OnStageStartEffectKind::LoseHealthEachStageDuringContract,
        OnStageStartEffectKind::LoseGold,
    ]
}
