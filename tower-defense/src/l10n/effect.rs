use super::{Language, Locale, LocalizedText};
use crate::{game_state::effect::Effect, theme::typography::TypographyBuilder, *};

#[allow(unreachable_patterns)]
#[derive(Clone, State)]
pub enum EffectText {
    Name(Effect),
    Description(Effect),
}

#[allow(unreachable_patterns)]
impl LocalizedText for EffectText {
    fn apply_to_builder<'a>(
        self,
        builder: TypographyBuilder<'a>,
        locale: &Locale,
    ) -> TypographyBuilder<'a> {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

#[allow(unreachable_patterns)]
impl EffectText {
    fn apply_korean<'a>(self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => builder.text("치유"),
                Effect::Lottery { .. } => builder.text("복권"),
                Effect::ExtraReroll => builder.text("추가 리롤"),
                Effect::ExtraShopReroll => builder.text("상점 추가 리롤"),
                Effect::Shield { .. } => builder.text("방어막"),
                Effect::EarnGold { .. } => builder.text("골드 획득"),
                Effect::DamageReduction { .. } => builder.text("피해 감소"),
                Effect::UserDamageReduction { .. } => builder.text("피해 감소"),
                Effect::LoseHealth { .. } => builder.text("체력 감소"),
                Effect::LoseHealthRange { .. } => builder.text("랜덤 체력 감소"),
                Effect::LoseGoldRange { .. } => builder.text("랜덤 골드 감소"),
                Effect::LoseHealthExpire { .. } => builder.text("계약 만료 시 체력 감소"),
                Effect::LoseGoldExpire { .. } => builder.text("계약 만료 시 골드 감소"),
                Effect::LoseGold { .. } => builder.text("골드 감소"),
                Effect::GrantUpgrade { .. } => builder.text("업그레이드 획득"),
                Effect::GrantItem { .. } => builder.text("아이템 획득"),
                Effect::AddChallengeMonster => builder.text("도전 몬스터 추가"),
                Effect::IncreaseAllTowersDamage { .. } => builder.text("모든 타워 공격력 증가"),
                Effect::DecreaseAllTowersDamage { .. } => builder.text("모든 타워 공격력 감소"),
                Effect::IncreaseAllTowersAttackSpeed { .. } => {
                    builder.text("모든 타워 공격속도 증가")
                }
                Effect::IncreaseAllTowersRange { .. } => builder.text("모든 타워 사정거리 증가"),
                Effect::DecreaseIncomingDamage { .. } => builder.text("받는 피해 감소"),
                Effect::IncreaseIncomingDamage { .. } => builder.text("받는 피해 증가"),
                Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("카드 선택 최대 슬롯 증가")
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("카드 선택 최대 리롤 증가")
                }
                Effect::IncreaseShopMaxRerolls { .. } => builder.text("상점 최대 리롤 증가"),
                Effect::IncreaseGoldGain { .. } => builder.text("골드 획득량 증가"),
                Effect::DecreaseGoldGainPercent { .. } => builder.text("골드 획득량 감소"),
                Effect::DisableItemAndUpgradePurchases => builder.text("아이템/업그레이드 구매 불가"),
                Effect::DisableItemUse => builder.text("아이템 사용 불가"),
                Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("카드 선택 최대 슬롯 감소")
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("카드 선택 최대 리롤 감소")
                }
                Effect::DecreaseShopMaxRerolls { .. } => builder.text("상점 최대 리롤 감소"),
                Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                    builder.text("카드 선택 리롤 체력 비용")
                }
                Effect::AddShopRerollHealthCost { .. } => builder.text("상점 리롤 체력 비용"),
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("적 체력 {}% 증가", percentage))
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("{} 랭크 타워 비활성화", rank))
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("{} 수트 타워 비활성화", suit))
                }
                Effect::AddTowerCardToPlacementHand { .. } => builder.text("추가 타워"),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("보호막 {}~{} 획득", min_amount, max_amount))
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("체력 {}~{} 회복", min_amount, max_amount))
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("골드 {:.0}~{:.0} 획득", min_amount, max_amount))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("체력 {:.0}~{:.0} 감소", min_amount, max_amount))
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "골드 {:.0}~{:.0} 감소 (부족 시 체력 {:.0}~{:.0} 감소)",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    ))
                }
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    builder.text(format!("❤ {amount:.0} 체력을 회복합니다"))
                }
                Effect::Shield { amount } => {
                    builder.text(format!("{amount:.0} 피해를 흡수하는 방어막을 획득합니다"))
                }
                Effect::ExtraReroll => builder.text("추가 리롤을 획득합니다"),
                Effect::ExtraShopReroll => builder.text("상점 추가 리롤을 획득합니다"),
                Effect::EarnGold { amount } => {
                    builder.text(format!("💰 {amount} 골드를 획득합니다"))
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => builder.text(format!(
                    "{:.0}% 확률로 💰 {amount:.0} 골드를 획득합니다",
                    probability * 100.0
                )),
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => builder.text(format!(
                    "받는 피해를 {:.0}% 감소시킵니다 ({:.1}초간)",
                    (1.0 - damage_multiply) * 100.0,
                    duration.as_secs_f32()
                )),
                Effect::UserDamageReduction { multiply, duration } => builder.text(format!(
                    "받는 피해를 {:.0}% 감소시킵니다 ({:.1}초간)",
                    (1.0 - multiply) * 100.0,
                    duration.as_secs_f32()
                )),
                Effect::LoseHealth { amount } => {
                    builder.text(format!("체력을 ❤ {amount:.0} 잃습니다"))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("체력을 {:.0}~{:.0}만큼 잃습니다", min_amount, max_amount))
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "골드를 {:.0}~{:.0}만큼 잃습니다. 골드가 부족하면 체력을 {:.0}~{:.0}만큼 잃습니다",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    ))
                }
                Effect::LoseHealthExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "계약 만료 시 체력을 {:.0}~{:.0}만큼 잃습니다",
                        min_amount, max_amount
                    ))
                }
                Effect::LoseGoldExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "계약 만료 시 골드를 {:.0}~{:.0}만큼 잃습니다. 골드가 부족하면 체력을 {:.0}~{:.0}만큼 잃습니다",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    ))
                }
                Effect::LoseGold { amount } => {
                    builder.text(format!("💰 {amount} 골드를 잃습니다"))
                }
                Effect::GrantUpgrade { .. } => builder.text("랜덤한 업그레이드를 획득합니다"),
                Effect::GrantItem { .. } => builder.text("아이템을 획득합니다"),
                Effect::AddChallengeMonster => {
                    builder.text("다음 라운드에 도전 몬스터가 추가됩니다")
                }
                Effect::IncreaseAllTowersDamage { multiplier } => {
                    builder.text(format!(
                        "모든 타워의 공격력이 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::DecreaseAllTowersDamage { multiplier } => {
                    builder.text(format!(
                        "모든 타워의 공격력이 {:.0}% 감소합니다",
                        (1.0 - multiplier) * 100.0
                    ))
                }
                Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
                    builder.text(format!(
                        "모든 타워의 공격속도가 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::IncreaseAllTowersRange { multiplier } => {
                    builder.text(format!(
                        "모든 타워의 사정거리가 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    builder.text(format!(
                        "받는 피해가 {:.0}% 감소합니다",
                        (1.0 - multiplier) * 100.0
                    ))
                }
                Effect::IncreaseIncomingDamage { multiplier } => {
                    builder.text(format!(
                        "받는 피해가 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
                    builder.text(format!(
                        "카드 선택 시 최대 {}장의 카드를 받을 수 있습니다",
                        5 + bonus
                    ))
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
                    builder.text(format!("카드 선택 시 최대 {}번 리롤할 수 있습니다", 1 + bonus))
                }
                Effect::IncreaseShopMaxRerolls { bonus } => {
                    builder.text(format!("상점 리롤 시 최대 {}번 리롤할 수 있습니다", 1 + bonus))
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    builder.text(format!(
                        "골드 획득량이 {:.0}% 증가합니다",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => {
                    builder.text(format!(
                        "골드 획득량이 {:.0}% 감소합니다",
                        reduction_percentage * 100.0
                    ))
                }
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("아이템과 업그레이드를 구매할 수 없습니다")
                }
                Effect::DisableItemUse => builder.text("아이템을 사용할 수 없습니다"),
                Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
                    builder.text(format!("카드 선택 시 최대 슬롯이 {}개 감소합니다", penalty))
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
                    builder.text(format!(
                        "카드 선택 시 최대 리롤 횟수가 {}회 감소합니다",
                        penalty
                    ))
                }
                Effect::DecreaseShopMaxRerolls { penalty } => {
                    builder.text(format!("상점 리롤 시 최대 횟수가 {}회 감소합니다", penalty))
                }
                Effect::AddCardSelectionHandRerollHealthCost { cost } => {
                    builder.text(format!("카드 선택 리롤 시 체력을 {} 잃습니다", cost))
                }
                Effect::AddShopRerollHealthCost { cost } => {
                    builder.text(format!("상점 리롤 시 체력을 {} 잃습니다", cost))
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("적 체력이 {}% 증가합니다", percentage))
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!(
                        "계약 기간 동안 {} 랭크 타워를 사용할 수 없습니다",
                        rank
                    ))
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!(
                        "계약 기간 동안 {} 수트 타워를 사용할 수 없습니다",
                        suit
                    ))
                }
                Effect::AddTowerCardToPlacementHand {
                    tower_kind,
                    suit,
                    rank,
                    count,
                } => {
                    let tower_name = tower_kind.to_text().to_korean();
                    match tower_kind {
                        crate::game_state::tower::TowerKind::Barricade => {
                            builder.text(format!(
                                "타워 설치 핸드에 {} 카드를 {}장 추가합니다",
                                tower_name, count
                            ))
                        }
                        _ => {
                            builder.text(format!(
                                "타워 설치 핸드에 {} {} {} 타워 카드를 {}장 추가합니다",
                                suit, rank, tower_name, count
                            ))
                        }
                    }
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("보호막을 {}~{}만큼 획득합니다", min_amount, max_amount))
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("체력을 {}~{}만큼 회복합니다", min_amount, max_amount))
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("골드를 {:.0}~{:.0}만큼 획득합니다", min_amount, max_amount))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("체력을 {:.0}~{:.0}만큼 감소합니다", min_amount, max_amount))
                }
            },
        }
    }

    fn apply_english<'a>(self, builder: TypographyBuilder<'a>) -> TypographyBuilder<'a> {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => builder.text("Heal"),
                Effect::Lottery { .. } => builder.text("Lottery"),
                Effect::ExtraReroll => builder.text("Extra Reroll"),
                Effect::ExtraShopReroll => builder.text("Extra Shop Reroll"),
                Effect::Shield { .. } => builder.text("Shield"),
                Effect::EarnGold { .. } => builder.text("Gold Gain"),
                Effect::DamageReduction { .. } => builder.text("Damage Reduction"),
                Effect::UserDamageReduction { .. } => builder.text("Damage Reduction"),
                Effect::LoseHealth { .. } => builder.text("Lose Health"),
                Effect::LoseHealthRange { .. } => builder.text("Lose Health (Random)"),
                Effect::LoseGoldRange { .. } => builder.text("Lose Gold (Random)"),
                Effect::LoseHealthExpire { .. } => builder.text("Lose Health On Contract End"),
                Effect::LoseGoldExpire { .. } => builder.text("Lose Gold On Contract End"),
                Effect::LoseGold { .. } => builder.text("Lose Gold"),
                Effect::GrantUpgrade { .. } => builder.text("Grant Upgrade"),
                Effect::GrantItem { .. } => builder.text("Grant Item"),
                Effect::AddChallengeMonster => builder.text("Add Challenge Monster"),
                Effect::IncreaseAllTowersDamage { .. } => builder.text("Increase All Towers Damage"),
                Effect::DecreaseAllTowersDamage { .. } => builder.text("Decrease All Towers Damage"),
                Effect::IncreaseAllTowersAttackSpeed { .. } => {
                    builder.text("Increase All Towers Attack Speed")
                }
                Effect::IncreaseAllTowersRange { .. } => builder.text("Increase All Towers Range"),
                Effect::DecreaseIncomingDamage { .. } => builder.text("Decrease Incoming Damage"),
                Effect::IncreaseIncomingDamage { .. } => builder.text("Increase Incoming Damage"),
                Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("Increase Card Selection Max Slots")
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("Increase Card Selection Max Rerolls")
                }
                Effect::IncreaseShopMaxRerolls { .. } => builder.text("Increase Shop Max Rerolls"),
                Effect::IncreaseGoldGain { .. } => builder.text("Increase Gold Gain"),
                Effect::DecreaseGoldGainPercent { .. } => builder.text("Decrease Gold Gain"),
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("Disable Item/Upgrade Purchases")
                }
                Effect::DisableItemUse => builder.text("Disable Item Use"),
                Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("Decrease Card Selection Max Slots")
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("Decrease Card Selection Max Rerolls")
                }
                Effect::DecreaseShopMaxRerolls { .. } => builder.text("Decrease Shop Max Rerolls"),
                Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                    builder.text("Card Selection Reroll Health Cost")
                }
                Effect::AddShopRerollHealthCost { .. } => builder.text("Shop Reroll Health Cost"),
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("Enemy Health +{}%", percentage))
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("Disable {} Rank Towers", rank))
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("Disable {} Suit Towers", suit))
                }
                Effect::AddTowerCardToPlacementHand { .. } => builder.text("Add Tower Card"),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Gain Shield ({}~{})", min_amount, max_amount))
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Heal Health ({}~{})", min_amount, max_amount))
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Gain Gold ({:.0}~{:.0})", min_amount, max_amount))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Lose Health ({:.0}~{:.0})", min_amount, max_amount))
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "Lose Gold ({:.0}~{:.0}), if insufficient, lose health ({:.0}~{:.0})",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    ))
                }
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    builder.text(format!("Restores ❤ {amount:.0} health"))
                }
                Effect::Shield { amount } => {
                    builder.text(format!("Gain a shield that absorbs {amount:.0} damage"))
                }
                Effect::ExtraReroll => builder.text("Gain an extra reroll"),
                Effect::ExtraShopReroll => builder.text("Gain an extra shop reroll"),
                Effect::EarnGold { amount } => {
                    builder.text(format!("Gain 💰 {amount} gold"))
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => builder.text(format!(
                    "{:.0}% chance to gain 💰 {amount:.0} gold",
                    probability * 100.0
                )),
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => builder.text(format!(
                    "Reduces damage taken by {:.0}% for {:.1}s",
                    (1.0 - damage_multiply) * 100.0,
                    duration.as_secs_f32()
                )),
                Effect::UserDamageReduction { multiply, duration } => builder.text(format!(
                    "Reduces damage taken by {:.0}% for {:.1}s",
                    (1.0 - multiply) * 100.0,
                    duration.as_secs_f32()
                )),
                Effect::LoseHealth { amount } => {
                    builder.text(format!("Lose ❤ {amount:.0} health"))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Lose {:.0}~{:.0} health", min_amount, max_amount))
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "Lose {:.0}~{:.0} gold, if insufficient, lose {:.0}~{:.0} health",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    ))
                }
                Effect::LoseHealthExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "Lose {:.0}~{:.0} health when contract expires",
                        min_amount, max_amount
                    ))
                }
                Effect::LoseGoldExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!(
                        "Lose {:.0}~{:.0} gold when contract expires, if insufficient, lose {:.0}~{:.0} health",
                        min_amount,
                        max_amount,
                        min_amount / 10.0,
                        max_amount / 10.0
                    ))
                }
                Effect::LoseGold { amount } => {
                    builder.text(format!("Lose 💰 {amount} gold"))
                }
                Effect::GrantUpgrade { .. } => builder.text("Gain a random upgrade"),
                Effect::GrantItem { .. } => builder.text("Gain an item"),
                Effect::AddChallengeMonster => builder.text("Add a challenge monster next round"),
                Effect::IncreaseAllTowersDamage { multiplier } => {
                    builder.text(format!(
                        "Increase damage of all towers by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::DecreaseAllTowersDamage { multiplier } => {
                    builder.text(format!(
                        "Decrease damage of all towers by {:.0}%",
                        (1.0 - multiplier) * 100.0
                    ))
                }
                Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
                    builder.text(format!(
                        "Increase attack speed of all towers by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::IncreaseAllTowersRange { multiplier } => {
                    builder.text(format!(
                        "Increase range of all towers by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    builder.text(format!(
                        "Reduce incoming damage by {:.0}%",
                        (1.0 - multiplier) * 100.0
                    ))
                }
                Effect::IncreaseIncomingDamage { multiplier } => {
                    builder.text(format!(
                        "Increase incoming damage by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
                    builder.text(format!("Can receive up to {} cards when selecting cards", 5 + bonus))
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
                    builder.text(format!(
                        "Can reroll up to {} times when selecting cards",
                        1 + bonus
                    ))
                }
                Effect::IncreaseShopMaxRerolls { bonus } => {
                    builder.text(format!("Can reroll shop up to {} times", 1 + bonus))
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    builder.text(format!(
                        "Increase gold gain by {:.0}%",
                        (multiplier - 1.0) * 100.0
                    ))
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => {
                    builder.text(format!(
                        "Decrease gold gain by {:.0}%",
                        reduction_percentage * 100.0
                    ))
                }
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("Cannot purchase items and upgrades")
                }
                Effect::DisableItemUse => builder.text("Cannot use items"),
                Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
                    builder.text(format!("Reduce maximum card selection slots by {}", penalty))
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
                    builder.text(format!(
                        "Reduce maximum card selection rerolls by {}",
                        penalty
                    ))
                }
                Effect::DecreaseShopMaxRerolls { penalty } => {
                    builder.text(format!("Reduce maximum shop rerolls by {}", penalty))
                }
                Effect::AddCardSelectionHandRerollHealthCost { cost } => {
                    builder.text(format!("Lose {} health when rerolling card selection", cost))
                }
                Effect::AddShopRerollHealthCost { cost } => {
                    builder.text(format!("Lose {} health when rerolling shop", cost))
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("Increase enemy health by {}%", percentage))
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("Cannot use {} rank towers during contract", rank))
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("Cannot use {} suit towers during contract", suit))
                }
                Effect::AddTowerCardToPlacementHand {
                    tower_kind,
                    suit,
                    rank,
                    count,
                } => {
                    let tower_name = tower_kind.to_text().to_english();
                    match tower_kind {
                        crate::game_state::tower::TowerKind::Barricade => {
                            builder.text(format!("Add {} {} cards to placement hand", count, tower_name))
                        }
                        _ => {
                            builder.text(format!(
                                "Add {} {} {} {} tower cards to placement hand",
                                count, suit, rank, tower_name
                            ))
                        }
                    }
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Gain shield ({}~{})", min_amount, max_amount))
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Heal health ({}~{})", min_amount, max_amount))
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Gain gold ({:.0}~{:.0})", min_amount, max_amount))
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder.text(format!("Lose health ({:.0}~{:.0})", min_amount, max_amount))
                }
            },
        }
    }

    #[allow(unreachable_patterns)]
    pub(super) fn text_korean(&self) -> String {
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
                    format!("❤ {amount:.0} 체력을 회복합니다")
                }
                Effect::Shield { amount } => {
                    format!("{amount:.0} 피해를 흡수하는 방어막을 획득합니다")
                }
                Effect::ExtraReroll => "추가 리롤을 획득합니다".to_string(),
                Effect::ExtraShopReroll => "상점 추가 리롤을 획득합니다".to_string(),
                Effect::EarnGold { amount } => {
                    format!("💰 {amount} 골드를 획득합니다")
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% 확률로 💰 {amount:.0} 골드를 획득합니다",
                    probability * 100.0
                ),
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "받는 피해를 {:.0}% 감소시킵니다 ({:.1}초간)",
                    (1.0 - damage_multiply) * 100.0,
                    duration.as_secs_f32()
                ),
                Effect::UserDamageReduction { multiply, duration } => format!(
                    "받는 피해를 {:.0}% 감소시킵니다 ({:.1}초간)",
                    (1.0 - multiply) * 100.0,
                    duration.as_secs_f32()
                ),
                Effect::LoseHealth { amount } => {
                    format!("체력을 ❤ {amount:.0} 잃습니다")
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
                    format!("💰 {amount} 골드를 잃습니다")
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

    pub(super) fn text_english(&self) -> String {
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
                    format!("Restores ❤ {amount:.0} health")
                }
                Effect::Shield { amount } => {
                    format!("Gain a shield that absorbs {amount:.0} damage")
                }
                Effect::ExtraReroll => "Gain an extra reroll".to_string(),
                Effect::ExtraShopReroll => "Gain an extra shop reroll".to_string(),
                Effect::EarnGold { amount } => {
                    format!("Gain 💰 {amount} gold")
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => format!(
                    "{:.0}% chance to gain 💰 {amount:.0} gold",
                    probability * 100.0
                ),
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => format!(
                    "Reduces damage taken by {:.0}% for {:.1}s",
                    (1.0 - damage_multiply) * 100.0,
                    duration.as_secs_f32()
                ),
                Effect::UserDamageReduction { multiply, duration } => format!(
                    "Reduces damage taken by {:.0}% for {:.1}s",
                    (1.0 - multiply) * 100.0,
                    duration.as_secs_f32()
                ),
                Effect::LoseHealth { amount } => {
                    format!("Lose ❤ {amount:.0} health")
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
                    format!("Lose 💰 {amount} gold")
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

impl EffectExecutionErrorText {
    pub fn text_korean(&self) -> String {
        use crate::game_state::effect::EffectExecutionError;
        match &self.0 {
            EffectExecutionError::ItemUseDisabled => "아이템을 사용할 수 없습니다".to_string(),
            EffectExecutionError::InvalidFlow { required } => {
                format!("잘못된 단계입니다 (필요: {})", required)
            }
        }
    }

    pub fn text_english(&self) -> String {
        use crate::game_state::effect::EffectExecutionError;
        match &self.0 {
            EffectExecutionError::ItemUseDisabled => "Cannot use items".to_string(),
            EffectExecutionError::InvalidFlow { required } => {
                format!("Invalid game flow (required: {})", required)
            }
        }
    }
}
