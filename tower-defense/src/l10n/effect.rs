use super::{Language, Locale, LocalizedText, rich_text_helpers::*};
use crate::game_state::effect::Effect;

#[derive(Clone)]
pub enum EffectText {
    Name(Effect),
    Description(Effect),
}

impl LocalizedText for EffectText {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl EffectText {
    pub(super) fn to_korean(&self) -> String {
        match self {
            EffectText::Name(effect) => match effect {
                Effect::Heal { .. } => "치유".to_string(),
                Effect::Lottery { .. } => "복권".to_string(),
                Effect::ExtraReroll => "추가 리롤".to_string(),
                Effect::Shield { .. } => "방어막".to_string(),
                Effect::EarnGold { .. } => "골드 획득".to_string(),
                Effect::DamageReduction { .. } => "피해 감소".to_string(),
                Effect::UserDamageReduction { .. } => "피해 감소".to_string(),
                Effect::LoseHealth { .. } => "체력 감소".to_string(),
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
                Effect::DecreaseGoldGainPercentDuringContract { .. } => {
                    "골드 획득량 감소".to_string()
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
                Effect::DecreaseGoldGainPercentDuringContract {
                    reduction_percentage,
                } => {
                    format!(
                        "골드 획득량이 {:.0}% 감소합니다",
                        reduction_percentage * 100.0
                    )
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
                Effect::Shield { .. } => "Shield".to_string(),
                Effect::EarnGold { .. } => "Gold Gain".to_string(),
                Effect::DamageReduction { .. } => "Damage Reduction".to_string(),
                Effect::UserDamageReduction { .. } => "Damage Reduction".to_string(),
                Effect::LoseHealth { .. } => "Lose Health".to_string(),
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
                Effect::DecreaseGoldGainPercentDuringContract { .. } => {
                    "Decrease Gold Gain".to_string()
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
                Effect::DecreaseGoldGainPercentDuringContract {
                    reduction_percentage,
                } => {
                    format!("Decrease gold gain by {:.0}%", reduction_percentage * 100.0)
                }
            },
        }
    }
}
