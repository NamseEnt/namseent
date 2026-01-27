use super::{Language, Locale, LocalizedRichText, LocalizedText, rich_text_helpers::*};
use crate::{game_state::effect::Effect, theme::typography::TypographyBuilder, *};

#[allow(unreachable_patterns)]
#[derive(Clone, State)]
pub enum EffectText {
    Name(Effect),
    Description(Effect),
}

#[allow(unreachable_patterns)]
impl LocalizedText for EffectText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

#[allow(unreachable_patterns)]
impl LocalizedRichText for EffectText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        // 임시로 String 기반 구현을 사용
        let text = self.localized_text(locale);
        builder.text(text)
    }
}

#[allow(unreachable_patterns)]
impl EffectText {
    #[allow(unreachable_patterns)]
    pub(super) fn to_korean(&self) -> String {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => "치유".to_string(),
                Effect::Lottery { .. } => "복권".to_string(),
                Effect::ExtraReroll => "추가 리롤".to_string(),
                Effect::ExtraShopReroll => "상점 추가 리롤".to_string(),
                Effect::Shield { .. } => "방어막".to_string(),
                Effect::EarnGold { .. } => "골드 획득".to_string(),
                Effect::DamageReduction { .. } => "피해 감소".to_string(),
                Effect::UserDamageReduction { .. } => "피해 감소".to_string(),
                Effect::LoseHealth { .. } => "체력 감소".to_string(),
                Effect::LoseHealthRange { .. } => "랜덤 체력 감소".to_string(),
                Effect::LoseGoldRange { .. } => "랜덤 골드 감소".to_string(),
                Effect::LoseHealthExpire { .. } => "계약 만료 시 체력 감소".to_string(),
                Effect::LoseGoldExpire { .. } => "계약 만료 시 골드 감소".to_string(),
                Effect::LoseGold { .. } => "골드 감소".to_string(),
                Effect::GrantUpgrade { .. } => "업그레이드 획득".to_string(),
                Effect::GrantItem { .. } => "아이템 획득".to_string(),
                Effect::AddChallengeMonster => "도전 몬스터 추가".to_string(),
                Effect::IncreaseAllTowersDamage { .. } => "모든 타워 공격력 증가".to_string(),
                Effect::DecreaseAllTowersDamage { .. } => "모든 타워 공격력 감소".to_string(),
                Effect::IncreaseAllTowersAttackSpeed { .. } => {
                    "모든 타워 공격속도 증가".to_string()
                }
                Effect::IncreaseAllTowersRange { .. } => "모든 타워 사정거리 증가".to_string(),
                Effect::DecreaseIncomingDamage { .. } => "받는 피해 감소".to_string(),
                Effect::IncreaseIncomingDamage { .. } => "받는 피해 증가".to_string(),
                Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                    "카드 선택 최대 슬롯 증가".to_string()
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                    "카드 선택 최대 리롤 증가".to_string()
                }
                Effect::IncreaseShopMaxRerolls { .. } => "상점 최대 리롤 증가".to_string(),
                Effect::IncreaseGoldGain { .. } => "골드 획득량 증가".to_string(),
                Effect::DecreaseGoldGainPercent { .. } => "골드 획득량 감소".to_string(),
                Effect::DisableItemAndUpgradePurchases => "아이템/업그레이드 구매 불가".to_string(),
                Effect::DisableItemUse => "아이템 사용 불가".to_string(),
                Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                    "카드 선택 최대 슬롯 감소".to_string()
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                    "카드 선택 최대 리롤 감소".to_string()
                }
                Effect::DecreaseShopMaxRerolls { .. } => "상점 최대 리롤 감소".to_string(),
                Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                    "카드 선택 리롤 체력 비용".to_string()
                }
                Effect::AddShopRerollHealthCost { .. } => "상점 리롤 체력 비용".to_string(),
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    format!("적 체력 {}% 증가", percentage)
                }
                Effect::RankTowerDisable { rank } => {
                    format!("{} 랭크 타워 비활성화", rank)
                }
                Effect::SuitTowerDisable { suit } => {
                    format!("{} 수트 타워 비활성화", suit)
                }
                Effect::AddTowerCardToPlacementHand { .. } => "추가 타워".to_string(),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    format!("보호막 {}~{} 획득", min_amount, max_amount)
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    format!("체력 {}~{} 회복", min_amount, max_amount)
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    format!("골드 {:.0}~{:.0} 획득", min_amount, max_amount)
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    format!("체력 {:.0}~{:.0} 감소", min_amount, max_amount)
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "골드 {:.0}~{:.0} 감소 (부족 시 체력 {:.0}~{:.0} 감소)",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    )
                }
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    format!("{} 체력을 회복합니다", heal_icon(format!("{amount:.0}")))
                }
                Effect::Shield { amount } => {
                    format!(
                        "{} 피해를 흡수하는 방어막을 획득합니다",
                        shield_value(format!("{amount:.0}"))
                    )
                }
                Effect::ExtraReroll => {
                    format!("{}을 획득합니다", special_item_text("추가 리롤"))
                }
                Effect::ExtraShopReroll => {
                    format!("{}을 획득합니다", special_item_text("상점 추가 리롤"))
                }
                Effect::EarnGold { amount } => {
                    format!("{} 골드를 획득합니다", gold_icon(format!("{amount}")))
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% 확률로 {} 골드를 획득합니다",
                    probability * 100.0,
                    gold_icon(format!("{amount:.0}"))
                ),
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "받는 피해를 {} 감소시킵니다 ({})",
                    reduction_percentage(format!("{:.0}", (1.0 - damage_multiply) * 100.0)),
                    time_duration(format!("{:.1}초간", duration.as_secs_f32()))
                ),
                Effect::UserDamageReduction { multiply, duration } => format!(
                    "받는 피해를 {} 감소시킵니다 ({})",
                    reduction_percentage(format!("{:.0}", (1.0 - multiply) * 100.0)),
                    time_duration(format!("{:.1}초간", duration.as_secs_f32()))
                ),
                Effect::LoseHealth { amount } => {
                    format!("체력을 {} 잃습니다", heal_icon(format!("{amount:.0}")))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    format!("체력을 {:.0}~{:.0}만큼 잃습니다", min_amount, max_amount)
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "골드를 {:.0}~{:.0}만큼 잃습니다. 골드가 부족하면 체력을 {:.0}~{:.0}만큼 잃습니다",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    )
                }
                Effect::LoseHealthExpire {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "계약 만료 시 체력을 {:.0}~{:.0}만큼 잃습니다",
                        min_amount, max_amount
                    )
                }
                Effect::LoseGoldExpire {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "계약 만료 시 골드를 {:.0}~{:.0}만큼 잃습니다. 골드가 부족하면 체력을 {:.0}~{:.0}만큼 잃습니다",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    )
                }
                Effect::LoseGold { amount } => {
                    format!("골드를 {} 잃습니다", gold_icon(format!("{amount}")))
                }
                Effect::GrantUpgrade { .. } => "랜덤한 업그레이드를 획득합니다".to_string(),
                Effect::GrantItem { .. } => "아이템을 획득합니다".to_string(),
                Effect::AddChallengeMonster => "다음 라운드에 도전 몬스터가 추가됩니다".to_string(),
                Effect::IncreaseAllTowersDamage { multiplier } => {
                    format!(
                        "모든 타워의 공격력이 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::DecreaseAllTowersDamage { multiplier } => {
                    format!(
                        "모든 타워의 공격력이 {:.0}% 감소합니다",
                        (1.0 - multiplier) * 100.0
                    )
                }
                Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
                    format!(
                        "모든 타워의 공격속도가 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::IncreaseAllTowersRange { multiplier } => {
                    format!(
                        "모든 타워의 사정거리가 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    format!("받는 피해가 {:.0}% 감소합니다", (1.0 - multiplier) * 100.0)
                }
                Effect::IncreaseIncomingDamage { multiplier } => {
                    format!("받는 피해가 {:.0}% 증가합니다", (multiplier - 1.0) * 100.0)
                }
                Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
                    format!(
                        "카드 선택 시 최대 {}장의 카드를 받을 수 있습니다",
                        5 + bonus
                    )
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
                    format!("카드 선택 시 최대 {}번 리롤할 수 있습니다", 1 + bonus)
                }
                Effect::IncreaseShopMaxRerolls { bonus } => {
                    format!("상점 리롤 시 최대 {}번 리롤할 수 있습니다", 1 + bonus)
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    format!(
                        "골드 획득량이 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => {
                    format!(
                        "골드 획득량이 {:.0}% 감소합니다",
                        reduction_percentage * 100.0
                    )
                }
                Effect::DisableItemAndUpgradePurchases => {
                    "아이템과 업그레이드를 구매할 수 없습니다".to_string()
                }
                Effect::DisableItemUse => "아이템을 사용할 수 없습니다".to_string(),
                Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
                    format!("카드 선택 시 최대 슬롯이 {}개 감소합니다", penalty)
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
                    format!("카드 선택 시 최대 리롤 횟수가 {}회 감소합니다", penalty)
                }
                Effect::DecreaseShopMaxRerolls { penalty } => {
                    format!("상점 리롤 시 최대 횟수가 {}회 감소합니다", penalty)
                }
                Effect::AddCardSelectionHandRerollHealthCost { cost } => {
                    format!("카드 선택 리롤 시 체력을 {} 잃습니다", cost)
                }
                Effect::AddShopRerollHealthCost { cost } => {
                    format!("상점 리롤 시 체력을 {} 잃습니다", cost)
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    format!("적 체력이 {}% 증가합니다", percentage)
                }
                Effect::RankTowerDisable { rank } => {
                    format!("계약 기간 동안 {} 랭크 타워를 사용할 수 없습니다", rank)
                }
                Effect::SuitTowerDisable { suit } => {
                    format!("계약 기간 동안 {} 수트 타워를 사용할 수 없습니다", suit)
                }
                Effect::AddTowerCardToPlacementHand {
                    tower_kind,
                    suit,
                    rank,
                    count,
                } => {
                    let tower_name = tower_kind.to_text().to_korean();
                    match *tower_kind {
                        crate::game_state::tower::TowerKind::Barricade => {
                            format!(
                                "타워 설치 핸드에 {} 카드를 {}장 추가합니다",
                                tower_name, count
                            )
                        }
                        _ => {
                            format!(
                                "타워 설치 핸드에 {} {} {} 타워 카드를 {}장 추가합니다",
                                suit, rank, tower_name, count
                            )
                        }
                    }
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    format!("보호막을 {}~{}만큼 획득합니다", min_amount, max_amount)
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    format!("체력을 {}~{}만큼 회복합니다", min_amount, max_amount)
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    format!("골드를 {:.0}~{:.0}만큼 획득합니다", min_amount, max_amount)
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    format!("체력을 {:.0}~{:.0}만큼 감소합니다", min_amount, max_amount)
                }
            },
        }
    }

    pub(super) fn to_english(&self) -> String {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => "Heal".to_string(),
                Effect::Lottery { .. } => "Lottery".to_string(),
                Effect::ExtraReroll => "Extra Reroll".to_string(),
                Effect::ExtraShopReroll => "Extra Shop Reroll".to_string(),
                Effect::Shield { .. } => "Shield".to_string(),
                Effect::EarnGold { .. } => "Gold Gain".to_string(),
                Effect::DamageReduction { .. } => "Damage Reduction".to_string(),
                Effect::UserDamageReduction { .. } => "Damage Reduction".to_string(),
                Effect::LoseHealth { .. } => "Lose Health".to_string(),
                Effect::LoseHealthRange { .. } => "Lose Health (Random)".to_string(),
                Effect::LoseGoldRange { .. } => "Lose Gold (Random)".to_string(),
                Effect::LoseHealthExpire { .. } => "Lose Health On Contract End".to_string(),
                Effect::LoseGoldExpire { .. } => "Lose Gold On Contract End".to_string(),
                Effect::LoseGold { .. } => "Lose Gold".to_string(),
                Effect::GrantUpgrade { .. } => "Grant Upgrade".to_string(),
                Effect::GrantItem { .. } => "Grant Item".to_string(),
                Effect::AddChallengeMonster => "Add Challenge Monster".to_string(),
                Effect::IncreaseAllTowersDamage { .. } => "Increase All Towers Damage".to_string(),
                Effect::DecreaseAllTowersDamage { .. } => "Decrease All Towers Damage".to_string(),
                Effect::IncreaseAllTowersAttackSpeed { .. } => {
                    "Increase All Towers Attack Speed".to_string()
                }
                Effect::IncreaseAllTowersRange { .. } => "Increase All Towers Range".to_string(),
                Effect::DecreaseIncomingDamage { .. } => "Decrease Incoming Damage".to_string(),
                Effect::IncreaseIncomingDamage { .. } => "Increase Incoming Damage".to_string(),
                Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                    "Increase Card Selection Max Slots".to_string()
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                    "Increase Card Selection Max Rerolls".to_string()
                }
                Effect::IncreaseShopMaxRerolls { .. } => "Increase Shop Max Rerolls".to_string(),
                Effect::IncreaseGoldGain { .. } => "Increase Gold Gain".to_string(),
                Effect::DecreaseGoldGainPercent { .. } => "Decrease Gold Gain".to_string(),
                Effect::DisableItemAndUpgradePurchases => {
                    "Disable Item/Upgrade Purchases".to_string()
                }
                Effect::DisableItemUse => "Disable Item Use".to_string(),
                Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                    "Decrease Card Selection Max Slots".to_string()
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                    "Decrease Card Selection Max Rerolls".to_string()
                }
                Effect::DecreaseShopMaxRerolls { .. } => "Decrease Shop Max Rerolls".to_string(),
                Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                    "Card Selection Reroll Health Cost".to_string()
                }
                Effect::AddShopRerollHealthCost { .. } => "Shop Reroll Health Cost".to_string(),
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    format!("Enemy Health +{}%", percentage)
                }
                Effect::RankTowerDisable { rank } => {
                    format!("Disable {} Rank Towers", rank)
                }
                Effect::SuitTowerDisable { suit } => {
                    format!("Disable {} Suit Towers", suit)
                }
                Effect::AddTowerCardToPlacementHand { .. } => "Add Tower Card".to_string(),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    format!("Gain Shield ({}~{})", min_amount, max_amount)
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    format!("Heal Health ({}~{})", min_amount, max_amount)
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    format!("Gain Gold ({:.0}~{:.0})", min_amount, max_amount)
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    format!("Lose Health ({:.0}~{:.0})", min_amount, max_amount)
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "Lose Gold ({:.0}~{:.0}), if insufficient, lose health ({:.0}~{:.0})",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    )
                }
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    format!("Restores {} health", heal_icon(format!("{amount:.0}")))
                }
                Effect::Shield { amount } => {
                    format!(
                        "Gain a shield that absorbs {} damage",
                        shield_value(format!("{amount:.0}"))
                    )
                }
                Effect::ExtraReroll => {
                    format!("Gain an {}", special_item_text("extra reroll"))
                }
                Effect::ExtraShopReroll => {
                    format!("Gain an {}", special_item_text("extra shop reroll"))
                }
                Effect::EarnGold { amount } => {
                    format!("Gain {} gold", gold_icon(format!("{amount}")))
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% chance to gain {} gold",
                    probability * 100.0,
                    gold_icon(format!("{amount:.0}"))
                ),
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "Reduces damage taken by {} for {}",
                    reduction_percentage(format!("{:.0}", (1.0 - damage_multiply) * 100.0)),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                Effect::UserDamageReduction { multiply, duration } => format!(
                    "Reduces damage taken by {} for {}",
                    reduction_percentage(format!("{:.0}", (1.0 - multiply) * 100.0)),
                    time_duration(format!("{:.1}s", duration.as_secs_f32()))
                ),
                Effect::LoseHealth { amount } => {
                    format!("Lose {} health", heal_icon(format!("{amount:.0}")))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    format!("Lose {:.0}~{:.0} health", min_amount, max_amount)
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "Lose {:.0}~{:.0} gold, if insufficient, lose {:.0}~{:.0} health",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    )
                }
                Effect::LoseHealthExpire {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "Lose {:.0}~{:.0} health when contract expires",
                        min_amount, max_amount
                    )
                }
                Effect::LoseGoldExpire {
                    min_amount,
                    max_amount,
                } => {
                    format!(
                        "Lose {:.0}~{:.0} gold when contract expires, if insufficient, lose {:.0}~{:.0} health",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    )
                }
                Effect::LoseGold { amount } => {
                    format!("Lose {} gold", gold_icon(format!("{amount}")))
                }
                Effect::GrantUpgrade { .. } => "Gain a random upgrade".to_string(),
                Effect::GrantItem { .. } => "Gain an item".to_string(),
                Effect::AddChallengeMonster => "Add a challenge monster next round".to_string(),
                Effect::IncreaseAllTowersDamage { multiplier } => {
                    format!(
                        "Increase damage of all towers by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::DecreaseAllTowersDamage { multiplier } => {
                    format!(
                        "Decrease damage of all towers by {:.0}%",
                        (1.0 - multiplier) * 100.0
                    )
                }
                Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
                    format!(
                        "Increase attack speed of all towers by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::IncreaseAllTowersRange { multiplier } => {
                    format!(
                        "Increase range of all towers by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    format!(
                        "Reduce incoming damage by {:.0}%",
                        (1.0 - multiplier) * 100.0
                    )
                }
                Effect::IncreaseIncomingDamage { multiplier } => {
                    format!(
                        "Increase incoming damage by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    )
                }
                Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
                    format!("Can receive up to {} cards when selecting cards", 5 + bonus)
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
                    format!("Can reroll up to {} times when selecting cards", 1 + bonus)
                }
                Effect::IncreaseShopMaxRerolls { bonus } => {
                    format!("Can reroll shop up to {} times", 1 + bonus)
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    format!("Increase gold gain by {:.0}%", (multiplier - 1.0) * 100.0)
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => {
                    format!("Decrease gold gain by {:.0}%", reduction_percentage * 100.0)
                }
                Effect::DisableItemAndUpgradePurchases => {
                    "Cannot purchase items and upgrades".to_string()
                }
                Effect::DisableItemUse => "Cannot use items".to_string(),
                Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
                    format!("Reduce maximum card selection slots by {}", penalty)
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
                    format!("Reduce maximum card selection rerolls by {}", penalty)
                }
                Effect::DecreaseShopMaxRerolls { penalty } => {
                    format!("Reduce maximum shop rerolls by {}", penalty)
                }
                Effect::AddCardSelectionHandRerollHealthCost { cost } => {
                    format!("Lose {} health when rerolling card selection", cost)
                }
                Effect::AddShopRerollHealthCost { cost } => {
                    format!("Lose {} health when rerolling shop", cost)
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    format!("Increase enemy health by {}%", percentage)
                }
                Effect::RankTowerDisable { rank } => {
                    format!("Cannot use {} rank towers during contract", rank)
                }
                Effect::SuitTowerDisable { suit } => {
                    format!("Cannot use {} suit towers during contract", suit)
                }
                Effect::AddTowerCardToPlacementHand {
                    tower_kind,
                    suit,
                    rank,
                    count,
                } => {
                    let tower_name = tower_kind.to_text().to_english();
                    match *tower_kind {
                        crate::game_state::tower::TowerKind::Barricade => {
                            format!("Add {} {} cards to placement hand", count, tower_name)
                        }
                        _ => {
                            format!(
                                "Add {} {} {} {} tower cards to placement hand",
                                count, suit, rank, tower_name
                            )
                        }
                    }
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    format!("Gain shield ({}~{})", min_amount, max_amount)
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    format!("Heal health ({}~{})", min_amount, max_amount)
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    format!("Gain gold ({:.0}~{:.0})", min_amount, max_amount)
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    format!("Lose health ({:.0}~{:.0})", min_amount, max_amount)
                }
            },
        }
    }
}

/// Effect 실행 에러 메시지 다국어 지원
#[derive(Clone, State)]
pub struct EffectExecutionErrorText(pub crate::game_state::effect::EffectExecutionError);

impl LocalizedText for EffectExecutionErrorText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl EffectExecutionErrorText {
    fn to_korean(&self) -> String {
        use crate::game_state::effect::EffectExecutionError;
        match &self.0 {
            EffectExecutionError::ItemUseDisabled => "아이템을 사용할 수 없습니다".to_string(),
            EffectExecutionError::InvalidFlow { required } => {
                format!("잘못된 단계입니다 (필요: {})", required)
            }
        }
    }

    fn to_english(&self) -> String {
        use crate::game_state::effect::EffectExecutionError;
        match &self.0 {
            EffectExecutionError::ItemUseDisabled => "Cannot use items".to_string(),
            EffectExecutionError::InvalidFlow { required } => {
                format!("Invalid game flow (required: {})", required)
            }
        }
    }
}
