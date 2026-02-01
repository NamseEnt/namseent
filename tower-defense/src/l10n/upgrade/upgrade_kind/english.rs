use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => {
                    builder.static_text("Gold Income Increase");
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, .. } => {
                    builder
                        .with_card_rank(format!("{rank}"))
                        .static_text(" Card ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiply");
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, .. } => {
                    builder
                        .with_suit_color(format!("{:?}", suit), *suit)
                        .static_text(" Card ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiply");
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    builder
                        .text(tower_name)
                        .static_text(" ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiplier");
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => {
                    builder.static_text("Shop Slot Expansion");
                },
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => {
                    builder.static_text("Reroll Count Increase");
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { .. } => {
                    builder
                        .static_text("Low Card Tower ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiply");
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => {
                    builder.static_text("Shop Item Price Discount");
                },
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => {
                    builder.static_text("Shop Refresh Count Increase");
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => {
                    builder
                        .static_text("No Reroll Tower ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiply");
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                    let card_type = if *even { "Even" } else { "Odd" };
                    builder
                        .text(card_type)
                        .static_text(" Card ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiplier");
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                    let card_type = if *face { "Face" } else { "Number" };
                    builder
                        .text(card_type)
                        .static_text(" Card ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiplier");
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => {
                    builder.static_text("Shorten Straight Flush to 4 Cards");
                },
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => {
                    builder.static_text("Skip Rank for Straight");
                },
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => {
                    builder.static_text("Treat All Suits as Same");
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { .. } => {
                    builder
                        .static_text("Reroll Tower ")
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" Multiply");
                },
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => {
                    builder
                        .static_text("Increases ")
                        .with_gold_icon("gold")
                        .static_text(" earned when defeating monsters.");
                },
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made with ")
                        .with_card_rank(format!("{rank}"))
                        .static_text(" cards.");
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made with ")
                        .with_suit_color(format!("{:?}", suit), *suit)
                        .static_text(" cards.");
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for ")
                        .text(tower_name)
                        .static_text(" towers.");
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => {
                    builder
                        .static_text("Adds ")
                        .with_positive_effect("1 slot")
                        .static_text(" available for purchase in the shop.");
                },
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => {
                    builder
                        .static_text("Increases the number of rerolls available each round by ")
                        .with_positive_effect("1")
                        .static_text(".");
                },
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made with 3 or fewer cards.");
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => {
                    builder.static_text("Shop item prices are discounted.");
                },
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => {
                    builder
                        .static_text("Shop refresh count increases by ")
                        .with_positive_effect("1")
                        .static_text(".");
                },
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made without rerolling.");
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                    let card_type = if *even { "even" } else { "odd" };
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made with ")
                        .text(card_type)
                        .static_text(" cards.");
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                    let card_type = if *face { "face" } else { "number" };
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made with ")
                        .text(card_type)
                        .static_text(" cards.");
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => {
                    builder.static_text("Allows making straight flush with 4 cards.");
                },
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => {
                    builder.static_text("Allows skipping one rank when making a straight.");
                },
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => {
                    builder.static_text("Treats all suits as the same.");
                },
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                    builder
                        .with_attack_damage_stat("Attack Damage")
                        .static_text(" increases by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" for towers made after rerolling.");
                },
            }
        }
    }

    fn get_english_tower_name(tower_kind: &crate::game_state::tower::TowerKind) -> &'static str {
        match tower_kind {
            crate::game_state::tower::TowerKind::Barricade => "Barricade",
            crate::game_state::tower::TowerKind::High => "High Card",
            crate::game_state::tower::TowerKind::OnePair => "One Pair",
            crate::game_state::tower::TowerKind::TwoPair => "Two Pair",
            crate::game_state::tower::TowerKind::ThreeOfAKind => "Three of a Kind",
            crate::game_state::tower::TowerKind::Straight => "Straight",
            crate::game_state::tower::TowerKind::Flush => "Flush",
            crate::game_state::tower::TowerKind::FullHouse => "Full House",
            crate::game_state::tower::TowerKind::FourOfAKind => "Four of a Kind",
            crate::game_state::tower::TowerKind::StraightFlush => "Straight Flush",
            crate::game_state::tower::TowerKind::RoyalFlush => "Royal Flush",
        }
    }
}
