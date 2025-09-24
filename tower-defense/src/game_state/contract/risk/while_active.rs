use super::WhileActiveEffectKind;

pub fn kinds() -> &'static [WhileActiveEffectKind] {
    &[
        WhileActiveEffectKind::DecreaseAllTowersDamage,
        WhileActiveEffectKind::IncreaseIncomingDamage,
        WhileActiveEffectKind::DecreaseGoldGain,
        WhileActiveEffectKind::DisableItemAndUpgradePurchases,
        WhileActiveEffectKind::DisableItemUse,
        WhileActiveEffectKind::DecreaseCardSelectionHandMaxSlots,
        WhileActiveEffectKind::DecreaseCardSelectionHandMaxSlotsDuringContract,
        WhileActiveEffectKind::DecreaseCardSelectionHandMaxRerolls,
        WhileActiveEffectKind::DecreaseShopMaxRerolls,
        WhileActiveEffectKind::AddCardSelectionHandRerollHealthCost,
        WhileActiveEffectKind::AddShopRerollHealthCost,
        WhileActiveEffectKind::DecreaseEnemyHealth,
        WhileActiveEffectKind::RankTowerDisable,
        WhileActiveEffectKind::SuitTowerDisable,
    ]
}
