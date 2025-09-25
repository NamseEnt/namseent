use super::OnStageStartEffectKind;

pub fn kinds() -> &'static [OnStageStartEffectKind] {
    &[
        OnStageStartEffectKind::AddBarricadeCardsToTowerPlacementHand,
        OnStageStartEffectKind::GainShield,
        OnStageStartEffectKind::HealHealth,
        OnStageStartEffectKind::GainGold,
    ]
}
