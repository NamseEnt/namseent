use super::{Language, Locale, LocalizedText};
use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::{game_state::effect::Effect, theme::typography::TypographyBuilder, *};

#[allow(unreachable_patterns)]
#[derive(Clone, State)]
pub enum EffectText {
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
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => {
                    builder.with_icon_bold(IconKind::Health, format!("+{:.0}", amount))
                }
                Effect::Shield { amount } => {
                    builder.with_icon_bold(IconKind::Shield, format!("{:.0}", amount))
                }
                Effect::EarnGold { amount } => {
                    builder.with_icon_bold(IconKind::Gold, format!("+{}", amount))
                }
                Effect::Lottery {
                    amount,
                    probability,
                } => builder
                    .text(format!("{:.0}% ", probability * 100.0))
                    .with_icon_bold(IconKind::Gold, format!("{}", amount)),
                Effect::DamageReduction {
                    damage_multiply, ..
                } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("-{:.0}%", (1.0 - damage_multiply) * 100.0),
                    )
                    .static_text(" 피해"),
                Effect::LoseHealth { amount } => {
                    builder.with_icon_bold(IconKind::Health, format!("-{:.0}", amount))
                }
                Effect::LoseGold { amount } => {
                    builder.with_icon_bold(IconKind::Gold, format!("-{}", amount))
                }
                Effect::GrantUpgrade { .. } => builder.text("업그레이드"),
                Effect::GrantItem { .. } => builder.text("아이템"),
                Effect::IncreaseAllTowersDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" 타워"),
                Effect::DecreaseAllTowersDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("-{:.0}%", (1.0 - multiplier) * 100.0),
                    )
                    .static_text(" 타워"),
                Effect::IncreaseIncomingDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" 받는 피해"),
                Effect::DecreaseIncomingDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("-{:.0}%", (1.0 - multiplier) * 100.0),
                    )
                    .static_text(" 받는 피해"),
                Effect::IncreaseGoldGain { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Gold,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" 골드"),
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => builder
                    .with_icon_bold(
                        IconKind::Gold,
                        format!("-{:.0}%", reduction_percentage * 100.0),
                    )
                    .static_text(" 골드"),
                Effect::DisableItemAndUpgradePurchases => builder.text("구매 불가"),
                Effect::DisableItemUse => builder.text("사용 불가"),
                Effect::IncreaseMaxHandSlots { bonus } => builder
                    .with_icon_bold(IconKind::Card, format!("+{}", bonus))
                    .static_text(" 슬롯"),
                Effect::DecreaseMaxHandSlots { penalty } => builder
                    .with_icon_bold(IconKind::Card, format!("-{}", penalty))
                    .static_text(" 슬롯"),
                Effect::IncreaseMaxRerolls { bonus } => builder
                    .with_icon_bold(IconKind::Refresh, format!("+{}", bonus))
                    .static_text(" 리롤"),
                Effect::DecreaseMaxRerolls { penalty } => builder
                    .with_icon_bold(IconKind::Refresh, format!("-{}", penalty))
                    .static_text(" 리롤"),
                Effect::IncreaseEnemyHealthPercent { percentage } => builder
                    .with_icon_bold(IconKind::Health, format!("+{:.0}%", percentage))
                    .static_text(" 적 체력"),
                Effect::DecreaseEnemyHealthPercent { percentage } => builder
                    .with_icon_bold(IconKind::Health, format!("-{:.0}%", percentage))
                    .static_text(" 적 체력"),
                Effect::IncreaseEnemySpeed { multiplier } => builder
                    .with_icon_bold(
                        IconKind::MoveSpeed,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" 적속도"),
                Effect::DecreaseEnemySpeed { multiplier } => builder
                    .with_icon_bold(
                        IconKind::MoveSpeed,
                        format!("-{:.0}%", (1.0 - multiplier) * 100.0),
                    )
                    .static_text(" 적속도"),
                Effect::RankTowerDisable { rank } => {
                    builder.card_rank(rank).static_text(" 랭크 타워")
                }
                Effect::SuitTowerDisable { suit } => builder.card_suit(suit).static_text(" 타워"),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => builder
                    .with_icon_bold(IconKind::Shield, format!("{}~{}", min_amount, max_amount))
                    .static_text(" 보호막"),
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => builder
                    .with_icon_bold(IconKind::Health, format!("{}~{}", min_amount, max_amount))
                    .static_text(" 회복"),
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => builder
                    .with_icon_bold(IconKind::Gold, format!("{}~{}", min_amount, max_amount))
                    .static_text(" 골드"),
                _ => builder.text(""),
            },
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            EffectText::Description(effect) => match effect {
                Effect::Heal { amount } => builder
                    .with_icon_bold(IconKind::Health, format!("+{:.0}", amount))
                    .static_text(" HP"),
                Effect::Shield { amount } => builder
                    .with_icon_bold(IconKind::Shield, format!("+{:.0}", amount))
                    .static_text(" shield"),
                Effect::EarnGold { amount } => builder
                    .with_icon_bold(IconKind::Gold, format!("{}", amount))
                    .static_text(" gold"),
                Effect::Lottery {
                    amount,
                    probability,
                } => builder
                    .text(format!("{:.0}% chance to get ", probability * 100.0))
                    .with_icon_bold(IconKind::Gold, format!("{}", amount)),
                Effect::DamageReduction {
                    damage_multiply, ..
                } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("-{:.0}%", (1.0 - damage_multiply) * 100.0),
                    )
                    .static_text(" damage"),
                Effect::LoseHealth { amount } => builder
                    .with_icon_bold(IconKind::Health, format!("-{:.0}", amount))
                    .static_text(" HP"),
                Effect::LoseGold { amount } => builder
                    .with_icon_bold(IconKind::Gold, format!("-{}", amount))
                    .static_text(" gold"),
                Effect::GrantUpgrade { .. } => builder.text("Upgrade"),
                Effect::GrantItem { .. } => builder.text("Item"),
                Effect::IncreaseAllTowersDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" tower damage"),
                Effect::DecreaseAllTowersDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("-{:.0}%", (1.0 - multiplier) * 100.0),
                    )
                    .static_text(" tower damage"),
                Effect::IncreaseIncomingDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" incoming damage"),
                Effect::DecreaseIncomingDamage { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Damage,
                        format!("-{:.0}%", (1.0 - multiplier) * 100.0),
                    )
                    .static_text(" incoming damage"),
                Effect::IncreaseGoldGain { multiplier } => builder
                    .with_icon_bold(
                        IconKind::Gold,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" gold"),
                Effect::DecreaseGoldGainPercent {
                    reduction_percentage,
                } => builder
                    .with_icon_bold(
                        IconKind::Gold,
                        format!("-{:.0}%", reduction_percentage * 100.0),
                    )
                    .static_text(" gold"),
                Effect::DisableItemAndUpgradePurchases => builder.text("Buy disabled"),
                Effect::DisableItemUse => builder.text("Use disabled"),
                Effect::IncreaseMaxHandSlots { bonus } => builder
                    .with_icon_bold(IconKind::Card, format!("+{}", bonus))
                    .static_text(" slots"),
                Effect::DecreaseMaxHandSlots { penalty } => builder
                    .with_icon_bold(IconKind::Card, format!("-{}", penalty))
                    .static_text(" slots"),
                Effect::IncreaseMaxRerolls { bonus } => builder
                    .with_icon_bold(IconKind::Refresh, format!("+{}", bonus))
                    .static_text(" rerolls"),
                Effect::DecreaseMaxRerolls { penalty } => builder
                    .with_icon_bold(IconKind::Refresh, format!("-{}", penalty))
                    .static_text(" rerolls"),
                Effect::IncreaseEnemyHealthPercent { percentage } => builder
                    .with_icon_bold(IconKind::Health, format!("+{:.0}%", percentage))
                    .static_text(" enemy HP"),
                Effect::DecreaseEnemyHealthPercent { percentage } => builder
                    .with_icon_bold(IconKind::Health, format!("-{:.0}%", percentage))
                    .static_text(" enemy HP"),
                Effect::IncreaseEnemySpeed { multiplier } => builder
                    .with_icon_bold(
                        IconKind::MoveSpeed,
                        format!("+{:.0}%", (multiplier - 1.0) * 100.0),
                    )
                    .static_text(" speed"),
                Effect::DecreaseEnemySpeed { multiplier } => builder
                    .with_icon_bold(
                        IconKind::MoveSpeed,
                        format!("-{:.0}%", (1.0 - multiplier) * 100.0),
                    )
                    .static_text(" speed"),
                Effect::RankTowerDisable { rank } => {
                    builder.card_rank(rank).static_text(" rank tower")
                }
                Effect::SuitTowerDisable { suit } => builder.card_suit(suit).static_text(" tower"),
                Effect::GainShield {
                    min_amount,
                    max_amount,
                } => builder
                    .with_icon_bold(IconKind::Shield, format!("{}~{}", min_amount, max_amount))
                    .static_text(" shield"),
                Effect::HealHealth {
                    min_amount,
                    max_amount,
                } => builder
                    .with_icon_bold(IconKind::Health, format!("{}~{}", min_amount, max_amount))
                    .static_text(" heal"),
                Effect::GainGold {
                    min_amount,
                    max_amount,
                } => builder
                    .with_icon_bold(IconKind::Gold, format!("{}~{}", min_amount, max_amount))
                    .static_text(" gold"),
                _ => builder.text(""),
            },
        };
    }
}
