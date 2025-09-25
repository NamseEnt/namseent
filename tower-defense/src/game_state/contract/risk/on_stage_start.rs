use super::OnStageStartEffectKind;

pub fn kinds() -> &'static [OnStageStartEffectKind] {
    &[
        OnStageStartEffectKind::LoseHealthEachStageDuringContract,
        OnStageStartEffectKind::LoseGoldEachStageDuringContract,
        OnStageStartEffectKind::LoseGold,
    ]
}
