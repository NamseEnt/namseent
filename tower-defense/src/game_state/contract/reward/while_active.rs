use super::WhileActiveEffectKind;

pub fn kinds() -> &'static [WhileActiveEffectKind] {
    &[
        WhileActiveEffectKind::IncreaseAllTowersDamage,
        WhileActiveEffectKind::IncreaseAllTowersAttackSpeed,
        WhileActiveEffectKind::IncreaseAllTowersRange,
        WhileActiveEffectKind::DecreaseIncomingDamage,
        WhileActiveEffectKind::IncreaseGoldGain,
        WhileActiveEffectKind::IncreaseCardSelectionHandMaxSlots,
        WhileActiveEffectKind::IncreaseCardSelectionHandMaxRerolls,
        WhileActiveEffectKind::IncreaseShopMaxRerolls,
    ]
}
