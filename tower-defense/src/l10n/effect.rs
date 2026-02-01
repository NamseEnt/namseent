use super::{Language, Locale, LocalizedText, rich_text_helpers::RichTextHelpers};
use crate::{game_state::effect::Effect, theme::typography::TypographyBuilder, *};

#[allow(unreachable_patterns)]
#[derive(Clone, State)]
pub enum EffectText {
    Name(Effect),
    Description(Effect),
}

#[allow(unreachable_patterns)]
impl LocalizedText for EffectText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

#[allow(unreachable_patterns)]
impl EffectText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => {
                    builder.text("치유");
                }
                Effect::Lottery { .. } => {
                    builder.text("복권");
                }
                Effect::ExtraReroll => {
                    builder.text("추가 리롤");
                }
                Effect::ExtraShopReroll => {
                    builder.text("상점 추가 리롤");
                }
                Effect::Shield { .. } => {
                    builder.text("방어막");
                }
                Effect::EarnGold { .. } => {
                    builder.text("골드 획득");
                }
                Effect::DamageReduction { .. } => {
                    builder.text("피해 감소");
                }
                Effect::UserDamageReduction { .. } => {
                    builder.text("피해 감소");
                }
                Effect::LoseHealth { .. } => {
                    builder.text("체력 감소");
                }
                Effect::LoseHealthRange { .. } => {
                    builder.text("랜덤 체력 감소");
                }
                Effect::LoseGoldRange { .. } => {
                    builder.text("랜덤 골드 감소");
                }
                Effect::LoseHealthExpire { .. } => {
                    builder.text("계약 만료 시 체력 감소");
                }
                Effect::LoseGoldExpire { .. } => {
                    builder.text("계약 만료 시 골드 감소");
                }
                Effect::LoseGold { .. } => {
                    builder.text("골드 감소");
                }
                Effect::GrantUpgrade { .. } => {
                    builder.text("업그레이드 획득");
                }
                Effect::GrantItem { .. } => {
                    builder.text("아이템 획득");
                }
                Effect::AddChallengeMonster => {
                    builder.text("도전 몬스터 추가");
                }
                Effect::IncreaseAllTowersDamage { .. } => {
                    builder.text("모든 타워 공격력 증가");
                }
                Effect::DecreaseAllTowersDamage { .. } => {
                    builder.text("모든 타워 공격력 감소");
                }
                Effect::IncreaseAllTowersAttackSpeed { .. } => {
                    builder.text("모든 타워 공격속도 증가");
                }
                Effect::IncreaseAllTowersRange { .. } => {
                    builder.text("모든 타워 사정거리 증가");
                }
                Effect::DecreaseIncomingDamage { .. } => {
                    builder.text("받는 피해 감소");
                }
                Effect::IncreaseIncomingDamage { .. } => {
                    builder.text("받는 피해 증가");
                }
                Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("카드 선택 최대 슬롯 증가");
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("카드 선택 최대 리롤 증가");
                }
                Effect::IncreaseShopMaxRerolls { .. } => {
                    builder.text("상점 최대 리롤 증가");
                }
                Effect::IncreaseGoldGain { .. } => {
                    builder.text("골드 획득량 증가");
                }
                Effect::DecreaseGoldGainPercent { .. } => {
                    builder.text("골드 획득량 감소");
                }
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("아이템/업그레이드 구매 불가");
                }
                Effect::DisableItemUse => {
                    builder.text("아이템 사용 불가");
                }
                Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("카드 선택 최대 슬롯 감소");
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("카드 선택 최대 리롤 감소");
                }
                Effect::DecreaseShopMaxRerolls { .. } => {
                    builder.text("상점 최대 리롤 감소");
                }
                Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                    builder.text("카드 선택 리롤 체력 비용");
                }
                Effect::AddShopRerollHealthCost { .. } => {
                    builder.text("상점 리롤 체력 비용");
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("적 체력 {}% 증가", percentage));
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("{} 랭크 타워 비활성화", rank));
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("{} 수트 타워 비활성화", suit));
                }
                Effect::AddTowerCardToPlacementHand { .. } => {
                    builder.text("추가 타워");
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("보호막 ")
                        .with_shield_value(format!("{}~{}", min_amount, max_amount))
                        .static_text(" 획득");
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("체력 ")
                        .with_health_value(format!("{}~{}", min_amount, max_amount))
                        .static_text(" 회복");
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("골드 ")
                        .with_gold_value(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" 획득");
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("체력 ")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" 감소");
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("골드 ")
                        .with_gold_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" 감소 (부족 시 체력 ")
                        .with_health_loss(format!(
                            "{:.0}~{:.0}",
                            min_amount / 10.0,
                            max_amount / 10.0
                        ))
                        .static_text(" 감소)");
                }
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    builder
                        .with_heal_icon(format!("{amount:.0}"))
                        .static_text(" 체력을 회복합니다");
                }
                Effect::Shield { amount } => {
                    builder
                        .with_shield_value(format!("{amount:.0}"))
                        .static_text(" 피해를 흡수하는 방어막을 획득합니다");
                }
                Effect::ExtraReroll => {
                    builder.text("추가 리롤을 획득합니다");
                }
                Effect::ExtraShopReroll => {
                    builder.text("상점 추가 리롤을 획득합니다");
                }
                Effect::EarnGold { amount } => {
                    builder
                        .with_gold_icon(format!("{amount}"))
                        .static_text(" 골드를 획득합니다");
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => {
                    builder
                        .text(format!("{:.0}% 확률로 ", probability * 100.0))
                        .with_gold_icon(format!("{amount:.0}"))
                        .static_text(" 골드를 획득합니다");
                }
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => {
                    builder
                        .static_text("받는 피해를 ")
                        .with_reduction_percentage(format!(
                            "{:.0}",
                            (1.0 - damage_multiply) * 100.0
                        ))
                        .static_text(" 감소시킵니다 (")
                        .with_time_duration(format!("{:.1}초", duration.as_secs_f32()))
                        .static_text("간)");
                }
                Effect::UserDamageReduction { multiply, duration } => {
                    builder
                        .static_text("받는 피해를 ")
                        .with_reduction_percentage(format!("{:.0}", (1.0 - multiply) * 100.0))
                        .static_text(" 감소시킵니다 (")
                        .with_time_duration(format!("{:.1}초", duration.as_secs_f32()))
                        .static_text("간)");
                }
                Effect::LoseHealth { amount } => {
                    builder
                        .static_text("체력을 ")
                        .with_health_loss(format!("{amount:.0}"))
                        .static_text(" 잃습니다");
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("체력을 ")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("만큼 잃습니다");
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("골드를 ")
                        .with_gold_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("만큼 잃습니다. 골드가 부족하면 체력을 ")
                        .with_health_loss(format!(
                            "{:.0}~{:.0}",
                            min_amount / 10.0,
                            max_amount / 10.0
                        ))
                        .static_text("만큼 잃습니다");
                }
                Effect::LoseHealthExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("계약 만료 시 체력을 ")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("만큼 잃습니다");
                }
                Effect::LoseGoldExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("계약 만료 시 골드를 ")
                        .with_gold_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("만큼 잃습니다. 골드가 부족하면 체력을 ")
                        .with_health_loss(format!(
                            "{:.0}~{:.0}",
                            min_amount / 10.0,
                            max_amount / 10.0
                        ))
                        .static_text("만큼 잃습니다");
                }
                Effect::LoseGold { amount } => {
                    builder
                        .static_text("골드를 ")
                        .with_gold_loss(format!("{amount}"))
                        .static_text(" 잃습니다");
                }
                Effect::GrantUpgrade { .. } => {
                    builder.text("랜덤한 업그레이드를 획득합니다");
                }
                Effect::GrantItem { .. } => {
                    builder.text("아이템을 획득합니다");
                }
                Effect::AddChallengeMonster => {
                    builder.text("다음 라운드에 도전 몬스터가 추가됩니다");
                }
                Effect::IncreaseAllTowersDamage { multiplier } => {
                    builder
                        .static_text("모든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0))
                        .static_text(" 증가합니다");
                }
                Effect::DecreaseAllTowersDamage { multiplier } => {
                    builder
                        .static_text("모든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_percentage_decrease(format!("{:.0}", (1.0 - multiplier) * 100.0))
                        .static_text(" 감소합니다");
                }
                Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
                    builder
                        .static_text("모든 타워의 ")
                        .with_attack_speed_stat("공격속도")
                        .static_text("가 ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0))
                        .static_text(" 증가합니다");
                }
                Effect::IncreaseAllTowersRange { multiplier } => {
                    builder
                        .static_text("모든 타워의 ")
                        .with_attack_range_stat("사정거리")
                        .static_text("가 ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0))
                        .static_text(" 증가합니다");
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    builder
                        .static_text("받는 피해가 ")
                        .with_reduction_percentage(format!("{:.0}", (1.0 - multiplier) * 100.0))
                        .static_text(" 감소합니다");
                }
                Effect::IncreaseIncomingDamage { multiplier } => {
                    builder
                        .static_text("받는 피해가 ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0))
                        .static_text(" 증가합니다");
                }
                Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
                    builder.text(format!(
                        "카드 선택 시 최대 {}장의 카드를 받을 수 있습니다",
                        5 + bonus
                    ));
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
                    builder.text(format!(
                        "카드 선택 시 최대 {}번 리롤할 수 있습니다",
                        1 + bonus
                    ));
                }
                Effect::IncreaseShopMaxRerolls { bonus } => {
                    builder.text(format!(
                        "상점 리롤 시 최대 {}번 리롤할 수 있습니다",
                        1 + bonus
                    ));
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    builder
                        .static_text("골드 획득량이 ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0))
                        .static_text(" 증가합니다");
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => {
                    builder
                        .static_text("골드 획득량이 ")
                        .with_percentage_decrease(format!("{:.0}", reduction_percentage * 100.0))
                        .static_text(" 감소합니다");
                }
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("아이템과 업그레이드를 구매할 수 없습니다");
                }
                Effect::DisableItemUse => {
                    builder.text("아이템을 사용할 수 없습니다");
                }
                Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
                    builder.text(format!("카드 선택 시 최대 슬롯이 {}개 감소합니다", penalty));
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
                    builder.text(format!(
                        "카드 선택 시 최대 리롤 횟수가 {}회 감소합니다",
                        penalty
                    ));
                }
                Effect::DecreaseShopMaxRerolls { penalty } => {
                    builder.text(format!("상점 리롤 시 최대 횟수가 {}회 감소합니다", penalty));
                }
                Effect::AddCardSelectionHandRerollHealthCost { cost } => {
                    builder
                        .static_text("카드 선택 리롤 시 체력을 ")
                        .with_health_loss(format!("{cost}"))
                        .static_text(" 잃습니다");
                }
                Effect::AddShopRerollHealthCost { cost } => {
                    builder
                        .static_text("상점 리롤 시 체력을 ")
                        .with_health_loss(format!("{cost}"))
                        .static_text(" 잃습니다");
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder
                        .static_text("적 체력이 ")
                        .with_percentage_increase(format!("{percentage}"))
                        .static_text(" 증가합니다");
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!(
                        "계약 기간 동안 {} 랭크 타워를 사용할 수 없습니다",
                        rank
                    ));
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!(
                        "계약 기간 동안 {} 수트 타워를 사용할 수 없습니다",
                        suit
                    ));
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
                            builder
                                .static_text("타워 설치 핸드에 ")
                                .text(format!("{} 카드를 {}", tower_name, count));
                            builder.static_text("장 추가합니다");
                        }
                        _ => {
                            builder.text(format!(
                                "타워 설치 핸드에 {} {} {} 타워 카드를 {}장 추가합니다",
                                suit, rank, tower_name, count
                            ));
                        }
                    }
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("보호막을 ")
                        .with_shield_value(format!("{}~{}", min_amount, max_amount))
                        .static_text("만큼 획득합니다");
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("체력을 ")
                        .with_health_value(format!("{}~{}", min_amount, max_amount))
                        .static_text("만큼 회복합니다");
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("골드를 ")
                        .with_gold_value(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("만큼 획득합니다");
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("체력을 ")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("만큼 감소합니다");
                }
            },
        }
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => {
                    builder.text("Heal");
                }
                Effect::Lottery { .. } => {
                    builder.text("Lottery");
                }
                Effect::ExtraReroll => {
                    builder.text("Extra Reroll");
                }
                Effect::ExtraShopReroll => {
                    builder.text("Extra Shop Reroll");
                }
                Effect::Shield { .. } => {
                    builder.text("Shield");
                }
                Effect::EarnGold { .. } => {
                    builder.text("Gold Gain");
                }
                Effect::DamageReduction { .. } => {
                    builder.text("Damage Reduction");
                }
                Effect::UserDamageReduction { .. } => {
                    builder.text("Damage Reduction");
                }
                Effect::LoseHealth { .. } => {
                    builder.text("Lose Health");
                }
                Effect::LoseHealthRange { .. } => {
                    builder.text("Lose Health (Random)");
                }
                Effect::LoseGoldRange { .. } => {
                    builder.text("Lose Gold (Random)");
                }
                Effect::LoseHealthExpire { .. } => {
                    builder.text("Lose Health On Contract End");
                }
                Effect::LoseGoldExpire { .. } => {
                    builder.text("Lose Gold On Contract End");
                }
                Effect::LoseGold { .. } => {
                    builder.text("Lose Gold");
                }
                Effect::GrantUpgrade { .. } => {
                    builder.text("Grant Upgrade");
                }
                Effect::GrantItem { .. } => {
                    builder.text("Grant Item");
                }
                Effect::AddChallengeMonster => {
                    builder.text("Add Challenge Monster");
                }
                Effect::IncreaseAllTowersDamage { .. } => {
                    builder.text("Increase All Towers Damage");
                }
                Effect::DecreaseAllTowersDamage { .. } => {
                    builder.text("Decrease All Towers Damage");
                }
                Effect::IncreaseAllTowersAttackSpeed { .. } => {
                    builder.text("Increase All Towers Attack Speed");
                }
                Effect::IncreaseAllTowersRange { .. } => {
                    builder.text("Increase All Towers Range");
                }
                Effect::DecreaseIncomingDamage { .. } => {
                    builder.text("Decrease Incoming Damage");
                }
                Effect::IncreaseIncomingDamage { .. } => {
                    builder.text("Increase Incoming Damage");
                }
                Effect::IncreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("Increase Card Selection Max Slots");
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("Increase Card Selection Max Rerolls");
                }
                Effect::IncreaseShopMaxRerolls { .. } => {
                    builder.text("Increase Shop Max Rerolls");
                }
                Effect::IncreaseGoldGain { .. } => {
                    builder.text("Increase Gold Gain");
                }
                Effect::DecreaseGoldGainPercent { .. } => {
                    builder.text("Decrease Gold Gain");
                }
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("Disable Item/Upgrade Purchases");
                }
                Effect::DisableItemUse => {
                    builder.text("Disable Item Use");
                }
                Effect::DecreaseCardSelectionHandMaxSlots { .. } => {
                    builder.text("Decrease Card Selection Max Slots");
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { .. } => {
                    builder.text("Decrease Card Selection Max Rerolls");
                }
                Effect::DecreaseShopMaxRerolls { .. } => {
                    builder.text("Decrease Shop Max Rerolls");
                }
                Effect::AddCardSelectionHandRerollHealthCost { .. } => {
                    builder.text("Card Selection Reroll Health Cost");
                }
                Effect::AddShopRerollHealthCost { .. } => {
                    builder.text("Shop Reroll Health Cost");
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("Enemy Health +{}%", percentage));
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("Disable {} Rank Towers", rank));
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("Disable {} Suit Towers", suit));
                }
                Effect::AddTowerCardToPlacementHand { .. } => {
                    builder.text("Add Tower Card");
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Gain Shield (")
                        .with_shield_value(format!("{}~{}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Heal Health (")
                        .with_health_value(format!("{}~{}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Gain Gold (")
                        .with_gold_value(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose Health (")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose Gold (")
                        .with_gold_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text("), if insufficient, lose health (")
                        .with_health_loss(format!(
                            "{:.0}~{:.0}",
                            min_amount / 10.0,
                            max_amount / 10.0
                        ))
                        .static_text(")");
                }
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    builder
                        .static_text("Restores ")
                        .with_heal_icon(format!("{amount:.0}"))
                        .static_text(" health");
                }
                Effect::Shield { amount } => {
                    builder
                        .static_text("Gain a shield that absorbs ")
                        .with_shield_value(format!("{amount:.0}"))
                        .static_text(" damage");
                }
                Effect::ExtraReroll => {
                    builder.text("Gain an extra reroll");
                }
                Effect::ExtraShopReroll => {
                    builder.text("Gain an extra shop reroll");
                }
                Effect::EarnGold { amount } => {
                    builder
                        .static_text("Gain ")
                        .with_gold_icon(format!("{amount}"))
                        .static_text(" gold");
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => {
                    builder
                        .text(format!("{:.0}% chance to gain ", probability * 100.0))
                        .with_gold_icon(format!("{amount:.0}"))
                        .static_text(" gold");
                }
                Effect::DamageReduction {
                    damage_multiply,
                    duration,
                } => {
                    builder
                        .static_text("Reduces damage taken by ")
                        .with_reduction_percentage(format!(
                            "{:.0}",
                            (1.0 - damage_multiply) * 100.0
                        ))
                        .static_text(" for ")
                        .with_time_duration(format!("{:.1}s", duration.as_secs_f32()));
                }
                Effect::UserDamageReduction { multiply, duration } => {
                    builder
                        .static_text("Reduces damage taken by ")
                        .with_reduction_percentage(format!("{:.0}", (1.0 - multiply) * 100.0))
                        .static_text(" for ")
                        .with_time_duration(format!("{:.1}s", duration.as_secs_f32()));
                }
                Effect::LoseHealth { amount } => {
                    builder
                        .static_text("Lose ")
                        .with_health_loss(format!("{amount:.0}"))
                        .static_text(" health");
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose ")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" health");
                }
                Effect::LoseGoldRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose ")
                        .with_gold_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" gold, if insufficient, lose ")
                        .with_health_loss(format!(
                            "{:.0}~{:.0}",
                            min_amount / 10.0,
                            max_amount / 10.0
                        ))
                        .static_text(" health");
                }
                Effect::LoseHealthExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose ")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" health when contract expires");
                }
                Effect::LoseGoldExpire {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose ")
                        .with_gold_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(" gold when contract expires, if insufficient, lose ")
                        .with_health_loss(format!(
                            "{:.0}~{:.0}",
                            min_amount / 10.0,
                            max_amount / 10.0
                        ))
                        .static_text(" health");
                }
                Effect::LoseGold { amount } => {
                    builder
                        .static_text("Lose ")
                        .with_gold_loss(format!("{amount}"))
                        .static_text(" gold");
                }
                Effect::GrantUpgrade { .. } => {
                    builder.text("Gain a random upgrade");
                }
                Effect::GrantItem { .. } => {
                    builder.text("Gain an item");
                }
                Effect::AddChallengeMonster => {
                    builder.text("Add a challenge monster next round");
                }
                Effect::IncreaseAllTowersDamage { multiplier } => {
                    builder
                        .static_text("Increase all towers' ")
                        .with_attack_damage_stat("damage")
                        .static_text(" by ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0));
                }
                Effect::DecreaseAllTowersDamage { multiplier } => {
                    builder
                        .static_text("Decrease all towers' ")
                        .with_attack_damage_stat("damage")
                        .static_text(" by ")
                        .with_percentage_decrease(format!("{:.0}", (1.0 - multiplier) * 100.0));
                }
                Effect::IncreaseAllTowersAttackSpeed { multiplier } => {
                    builder
                        .static_text("Increase all towers' ")
                        .with_attack_speed_stat("attack speed")
                        .static_text(" by ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0));
                }
                Effect::IncreaseAllTowersRange { multiplier } => {
                    builder
                        .static_text("Increase all towers' ")
                        .with_attack_range_stat("range")
                        .static_text(" by ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0));
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    builder
                        .static_text("Reduce incoming damage by ")
                        .with_reduction_percentage(format!("{:.0}", (1.0 - multiplier) * 100.0));
                }
                Effect::IncreaseIncomingDamage { multiplier } => {
                    builder
                        .static_text("Increase incoming damage by ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0));
                }
                Effect::IncreaseCardSelectionHandMaxSlots { bonus } => {
                    builder.text(format!(
                        "Can receive up to {} cards when selecting cards",
                        5 + bonus
                    ));
                }
                Effect::IncreaseCardSelectionHandMaxRerolls { bonus } => {
                    builder.text(format!(
                        "Can reroll up to {} times when selecting cards",
                        1 + bonus
                    ));
                }
                Effect::IncreaseShopMaxRerolls { bonus } => {
                    builder.text(format!("Can reroll shop up to {} times", 1 + bonus));
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    builder
                        .static_text("Increase gold gain by ")
                        .with_percentage_increase(format!("{:.0}", (multiplier - 1.0) * 100.0));
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => {
                    builder
                        .static_text("Decrease gold gain by ")
                        .with_percentage_decrease(format!("{:.0}", reduction_percentage * 100.0));
                }
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("Cannot purchase items and upgrades");
                }
                Effect::DisableItemUse => {
                    builder.text("Cannot use items");
                }
                Effect::DecreaseCardSelectionHandMaxSlots { penalty } => {
                    builder.text(format!(
                        "Reduce maximum card selection slots by {}",
                        penalty
                    ));
                }
                Effect::DecreaseCardSelectionHandMaxRerolls { penalty } => {
                    builder.text(format!(
                        "Reduce maximum card selection rerolls by {}",
                        penalty
                    ));
                }
                Effect::DecreaseShopMaxRerolls { penalty } => {
                    builder.text(format!("Reduce maximum shop rerolls by {}", penalty));
                }
                Effect::AddCardSelectionHandRerollHealthCost { cost } => {
                    builder
                        .static_text("Lose ")
                        .with_health_loss(format!("{cost}"))
                        .static_text(" health when rerolling card selection");
                }
                Effect::AddShopRerollHealthCost { cost } => {
                    builder
                        .static_text("Lose ")
                        .with_health_loss(format!("{cost}"))
                        .static_text(" health when rerolling shop");
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder
                        .static_text("Increase enemy health by ")
                        .with_percentage_increase(format!("{percentage}"));
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("Cannot use {} rank towers during contract", rank));
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("Cannot use {} suit towers during contract", suit));
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
                            builder.text(format!(
                                "Add {} {} cards to placement hand",
                                count, tower_name
                            ));
                        }
                        _ => {
                            builder.text(format!(
                                "Add {} {} {} {} tower cards to placement hand",
                                count, suit, rank, tower_name
                            ));
                        }
                    }
                }
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Gain shield (")
                        .with_shield_value(format!("{}~{}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Heal health (")
                        .with_health_value(format!("{}~{}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Gain gold (")
                        .with_gold_value(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(")");
                }
                Effect::LoseHealthRange {
                    min_amount,
                    max_amount,
                } => {
                    builder
                        .static_text("Lose health (")
                        .with_health_loss(format!("{:.0}~{:.0}", min_amount, max_amount))
                        .static_text(")");
                }
            },
        }
    }
}

/// Effect 실행 에러 메시지 다국어 지원
#[derive(Clone, State)]
pub struct EffectExecutionErrorText(pub crate::game_state::effect::EffectExecutionError);

impl LocalizedText for EffectExecutionErrorText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl EffectExecutionErrorText {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        use crate::game_state::effect::EffectExecutionError;
        match &self.0 {
            EffectExecutionError::ItemUseDisabled => {
                builder.text("아이템을 사용할 수 없습니다");
            }
            EffectExecutionError::InvalidFlow { required } => {
                builder
                    .text("잘못된 단계입니다 (필요: ")
                    .text(required)
                    .text(")");
            }
        }
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        use crate::game_state::effect::EffectExecutionError;
        match &self.0 {
            EffectExecutionError::ItemUseDisabled => {
                builder.text("Cannot use items");
            }
            EffectExecutionError::InvalidFlow { required } => {
                builder
                    .text("Invalid game flow (required: ")
                    .text(required)
                    .text(")");
            }
        }
    }
}
