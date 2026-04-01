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
                Effect::Heal { .. } => builder.text("치유"),
                Effect::Shield { .. } => builder.text("방어막"),
                Effect::ExtraDice => builder.text("추가 주사위"),
                Effect::EarnGold { .. } => builder.text("골드 획득"),
                Effect::Lottery { .. } => builder.text("복권"),
                Effect::DamageReduction { .. } => builder.text("피해 감소"),
                Effect::UserDamageReduction { .. } => builder.text("피해 감소"),
                Effect::LoseHealth { .. } => builder.text("체력 감소"),
                Effect::LoseGold { .. } => builder.text("골드 감소"),
                Effect::GrantUpgrade { .. } => builder.text("업그레이드 획득"),
                Effect::GrantItem { .. } => builder.text("아이템 획득"),
                Effect::IncreaseAllTowersDamage { .. } => builder.text("모든 타워 공격력 증가"),
                Effect::DecreaseAllTowersDamage { .. } => builder.text("모든 타워 공격력 감소"),
                Effect::IncreaseIncomingDamage { .. } => builder.text("받는 피해 증가"),
                Effect::DecreaseIncomingDamage { .. } => builder.text("받는 피해 감소"),
                Effect::IncreaseGoldGain { .. } => builder.text("골드 획득량 증가"),
                Effect::DecreaseGoldGainPercent { .. } => builder.text("골드 획득량 감소"),
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("아이템/업그레이드 구매 불가")
                }
                Effect::DisableItemUse => builder.text("아이템 사용 불가"),
                Effect::IncreaseMaxHandSlots { .. } => builder.text("카드 선택 최대 슬롯 증가"),
                Effect::DecreaseMaxHandSlots { .. } => builder.text("카드 선택 최대 슬롯 감소"),
                Effect::IncreaseMaxRerolls { .. } => builder.text("최대 리롤 증가"),
                Effect::DecreaseMaxRerolls { .. } => builder.text("최대 리롤 감소"),
                Effect::IncreaseEnemyHealthPercent { .. } => builder.text("적 체력 증가"),
                Effect::DecreaseEnemyHealthPercent { .. } => builder.text("적 체력 감소"),
                Effect::IncreaseEnemySpeed { .. } => builder.text("적 이동속도 증가"),
                Effect::DecreaseEnemySpeed { .. } => builder.text("적 이동속도 감소"),
                Effect::RankTowerDisable { .. } => builder.text("특정 랭크 타워 비활성화"),
                Effect::SuitTowerDisable { .. } => builder.text("특정 수트 타워 비활성화"),
                Effect::AddTowerCardToPlacementHand { .. } => builder.text("타워 카드 추가"),
                Effect::GainShield { .. } => builder.text("보호막 획득"),
                Effect::HealHealth { .. } => builder.text("체력 회복"),
                Effect::GainGold { .. } => builder.text("골드 획득"),
                _ => builder.text(""),
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => builder.text(format!("체력을 {:.0} 회복합니다", amount)),
                Effect::Shield { amount } => {
                    builder.text(format!("{:.0} 피해를 흡수하는 방어막", amount))
                }
                Effect::ExtraDice => builder.text("추가 주사위 1회"),
                Effect::EarnGold { amount } => builder.text(format!("{} 골드 획득", amount)),
                Effect::Lottery {
                    amount,
                    probability,
                } => builder.text(format!(
                    "{:.0}% 확률로 {} 골드 획득",
                    probability * 100.0,
                    amount
                )),
                Effect::DamageReduction { .. } => builder.text("받는 피해 감소"),
                Effect::UserDamageReduction { .. } => builder.text("받는 피해 감소"),
                Effect::LoseHealth { amount } => builder.text(format!("{} 체력 감소", amount)),
                Effect::LoseGold { amount } => builder.text(format!("{} 골드 감소", amount)),
                Effect::GrantUpgrade { .. } => builder.text("업그레이드 획득"),
                Effect::GrantItem { .. } => builder.text("아이템 획득"),
                Effect::IncreaseAllTowersDamage { multiplier } => builder.text(format!(
                    "모든 타워 공격력 +{:.0}%",
                    (multiplier - 1.0) * 100.0
                )),
                Effect::DecreaseAllTowersDamage { multiplier } => builder.text(format!(
                    "모든 타워 공격력 -{:.0}%",
                    (1.0 - multiplier) * 100.0
                )),
                Effect::IncreaseIncomingDamage { multiplier } => {
                    builder.text(format!("받는 피해 +{:.0}%", (multiplier - 1.0) * 100.0))
                }
                Effect::DecreaseIncomingDamage { multiplier } => {
                    builder.text(format!("받는 피해 -{:.0}%", (1.0 - multiplier) * 100.0))
                }
                Effect::IncreaseGoldGain { multiplier } => {
                    builder.text(format!("골드 획득량 +{:.0}%", (multiplier - 1.0) * 100.0))
                }
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => builder.text(format!("골드 획득량 -{:.0}%", reduction_percentage * 100.0)),
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("아이템/업그레이드 구매 불가")
                }
                Effect::DisableItemUse => builder.text("아이템 사용 불가"),
                Effect::IncreaseMaxHandSlots { bonus } => {
                    builder.text(format!("카드 선택 슬롯 +{}", bonus))
                }
                Effect::DecreaseMaxHandSlots { penalty } => {
                    builder.text(format!("카드 선택 슬롯 -{}", penalty))
                }
                Effect::IncreaseMaxRerolls { bonus } => builder.text(format!("리롤 +{}", bonus)),
                Effect::DecreaseMaxRerolls { penalty } => {
                    builder.text(format!("리롤 -{}", penalty))
                }
                Effect::IncreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("적 체력 +{:.0}%", percentage))
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("적 체력 -{:.0}%", percentage))
                }
                Effect::IncreaseEnemySpeed { multiplier } => {
                    builder.text(format!("적 이동속도 +{:.0}%", (multiplier - 1.0) * 100.0))
                }
                Effect::DecreaseEnemySpeed { multiplier } => {
                    builder.text(format!("적 이동속도 -{:.0}%", (1.0 - multiplier) * 100.0))
                }
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("{} 랭크 타워 비활성화", rank))
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("{} 수트 타워 비활성화", suit))
                }
                Effect::AddTowerCardToPlacementHand {
                    tower_kind,
                    suit,
                    rank,
                    count,
                } => builder.text(format!(
                    "타워 설치 핸드에 {} {} {} 카드를 {}장 추가",
                    suit,
                    rank,
                    tower_kind.to_text().to_korean(),
                    count
                )),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => builder.text(format!("보호막 {}~{} 획득", min_amount, max_amount)),
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => builder.text(format!("체력 {}~{} 회복", min_amount, max_amount)),
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => builder.text(format!("골드 {}~{} 획득", min_amount, max_amount)),
                _ => builder.text(""),
            },
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => builder.text("Heal"),
                Effect::Shield { .. } => builder.text("Shield"),
                Effect::ExtraDice => builder.text("Extra Dice"),
                Effect::EarnGold { .. } => builder.text("Earn Gold"),
                Effect::Lottery { .. } => builder.text("Lottery"),
                Effect::DamageReduction { .. } => builder.text("Damage Reduction"),
                Effect::UserDamageReduction { .. } => builder.text("Damage Reduction"),
                Effect::LoseHealth { .. } => builder.text("Lose Health"),
                Effect::LoseGold { .. } => builder.text("Lose Gold"),
                Effect::GrantUpgrade { .. } => builder.text("Grant Upgrade"),
                Effect::GrantItem { .. } => builder.text("Grant Item"),
                Effect::IncreaseAllTowersDamage { .. } => {
                    builder.text("Increase All Towers Damage")
                }
                Effect::DecreaseAllTowersDamage { .. } => {
                    builder.text("Decrease All Towers Damage")
                }
                Effect::IncreaseIncomingDamage { .. } => builder.text("Increase Incoming Damage"),
                Effect::DecreaseIncomingDamage { .. } => builder.text("Decrease Incoming Damage"),
                Effect::IncreaseGoldGain { .. } => builder.text("Increase Gold Gain"),
                Effect::DecreaseGoldGainPercent { .. } => builder.text("Decrease Gold Gain"),
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("Disable Item/Upgrade Purchases")
                }
                Effect::DisableItemUse => builder.text("Disable Item Use"),
                Effect::IncreaseMaxHandSlots { .. } => builder.text("Increase Max Hand Slots"),
                Effect::DecreaseMaxHandSlots { .. } => builder.text("Decrease Max Hand Slots"),
                Effect::IncreaseMaxRerolls { .. } => builder.text("Increase Max Rerolls"),
                Effect::DecreaseMaxRerolls { .. } => builder.text("Decrease Max Rerolls"),
                Effect::IncreaseEnemyHealthPercent { .. } => {
                    builder.text("Increase Enemy Health Percent")
                }
                Effect::DecreaseEnemyHealthPercent { .. } => {
                    builder.text("Decrease Enemy Health Percent")
                }
                Effect::IncreaseEnemySpeed { .. } => builder.text("Increase Enemy Speed"),
                Effect::DecreaseEnemySpeed { .. } => builder.text("Decrease Enemy Speed"),
                Effect::RankTowerDisable { .. } => builder.text("Rank Tower Disable"),
                Effect::SuitTowerDisable { .. } => builder.text("Suit Tower Disable"),
                Effect::AddTowerCardToPlacementHand { .. } => builder.text("Add Tower Card"),
                Effect::GainShield { .. } => builder.text("Gain Shield"),
                Effect::HealHealth { .. } => builder.text("Heal Health"),
                Effect::GainGold { .. } => builder.text("Gain Gold"),
                _ => builder.text(""),
            },
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => builder.text(format!("Heal {:.0} health", amount)),
                Effect::Shield { amount } => builder.text(format!("Gain {:.0} shield", amount)),
                Effect::ExtraDice => builder.text("Gain extra dice"),
                Effect::EarnGold { amount } => builder.text(format!("Gain {} gold", amount)),
                Effect::Lottery {
                    amount,
                    probability,
                } => builder.text(format!(
                    "{}% chance to get {} gold",
                    probability * 100.0,
                    amount
                )),
                Effect::DamageReduction { .. } => builder.text("Reduce damage"),
                Effect::UserDamageReduction { .. } => builder.text("Reduce damage"),
                Effect::LoseHealth { amount } => builder.text(format!("Lose {:.0} health", amount)),
                Effect::LoseGold { amount } => builder.text(format!("Lose {} gold", amount)),
                Effect::GrantUpgrade { .. } => builder.text("Gain upgrade"),
                Effect::GrantItem { .. } => builder.text("Gain item"),
                Effect::IncreaseAllTowersDamage { multiplier } => builder.text(format!(
                    "Increase tower damage by {:.0}%",
                    (multiplier - 1.0) * 100.0
                )),
                Effect::DecreaseAllTowersDamage { multiplier } => builder.text(format!(
                    "Decrease tower damage by {:.0}%",
                    (1.0 - multiplier) * 100.0
                )),
                Effect::IncreaseIncomingDamage { multiplier } => builder.text(format!(
                    "Increase incoming damage by {:.0}%",
                    (multiplier - 1.0) * 100.0
                )),
                Effect::DecreaseIncomingDamage { multiplier } => builder.text(format!(
                    "Decrease incoming damage by {:.0}%",
                    (1.0 - multiplier) * 100.0
                )),
                Effect::IncreaseGoldGain { multiplier } => builder.text(format!(
                    "Increase gold gain by {:.0}%",
                    (multiplier - 1.0) * 100.0
                )),
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => builder.text(format!(
                    "Decrease gold gain by {:.0}%",
                    reduction_percentage * 100.0
                )),
                Effect::DisableItemAndUpgradePurchases => {
                    builder.text("Item/Upgrade purchase disabled")
                }
                Effect::DisableItemUse => builder.text("Item use disabled"),
                Effect::IncreaseMaxHandSlots { bonus } => {
                    builder.text(format!("+{} max hand slots", bonus))
                }
                Effect::DecreaseMaxHandSlots { penalty } => {
                    builder.text(format!("-{} max hand slots", penalty))
                }
                Effect::IncreaseMaxRerolls { bonus } => builder.text(format!("+{} rerolls", bonus)),
                Effect::DecreaseMaxRerolls { penalty } => {
                    builder.text(format!("-{} rerolls", penalty))
                }
                Effect::IncreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("Increase enemy health by {}%", percentage))
                }
                Effect::DecreaseEnemyHealthPercent { percentage } => {
                    builder.text(format!("Decrease enemy health by {}%", percentage))
                }
                Effect::IncreaseEnemySpeed { multiplier } => builder.text(format!(
                    "Increase enemy speed by {:.0}%",
                    (multiplier - 1.0) * 100.0
                )),
                Effect::DecreaseEnemySpeed { multiplier } => builder.text(format!(
                    "Decrease enemy speed by {:.0}%",
                    (1.0 - multiplier) * 100.0
                )),
                Effect::RankTowerDisable { rank } => {
                    builder.text(format!("Disable rank {} towers", rank))
                }
                Effect::SuitTowerDisable { suit } => {
                    builder.text(format!("Disable suit {} towers", suit))
                }
                Effect::AddTowerCardToPlacementHand {
                    tower_kind,
                    suit,
                    rank,
                    count,
                } => builder.text(format!(
                    "Add {} {} {} tower cards {}",
                    count,
                    rank,
                    suit,
                    tower_kind.to_text().to_english()
                )),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => builder.text(format!("Gain shield {}~{}", min_amount, max_amount)),
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => builder.text(format!("Heal {}~{}", min_amount, max_amount)),
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => builder.text(format!("Gain {}~{} gold", min_amount, max_amount)),
                _ => builder.text(""),
            },
        };
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
