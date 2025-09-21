use super::OnSignEffectKind;

pub fn kinds() -> &'static [OnSignEffectKind] {
    &[
        OnSignEffectKind::LoseHealth,
        OnSignEffectKind::LoseGold,
        OnSignEffectKind::AddChallengeMonsterNextRound,
    ]
}
