use crate::l10n::upgrade::UpgradeKindText;

impl UpgradeKindText<'_> {
    pub fn to_english(&self) -> String {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => "Gold Income Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, .. } => format!("{rank} Card Attack Damage Multiply"),
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, .. } => format!("{:?} Card Attack Damage Multiply", suit),
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, .. } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("{tower_name} Attack Damage Multiplier")
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => "Shop Slot Expansion".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => "Reroll Count Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { .. } => "Low Card Tower Attack Damage Multiply".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => "Shop Item Price Discount".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => "Shop Refresh Count Increase".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { .. } => "No Reroll Tower Attack Damage Multiply".to_string(),
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, .. } => {
                    if *even { "Even Card Attack Damage Multiplier" } else { "Odd Card Attack Damage Multiplier" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, .. } => {
                    if *face { "Face Card Attack Damage Multiplier" } else { "Number Card Attack Damage Multiplier" }.to_string()
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => "Shorten Straight Flush to 4 Cards".to_string(),
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => "Skip Rank for Straight".to_string(),
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => "Treat All Suits as Same".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { .. } => "Reroll Tower Attack Damage Multiply".to_string(),
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::GoldEarnPlus => "Increases gold earned when defeating monsters.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RankAttackDamageMultiply { rank, damage_multiplier } => {
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {rank} cards.")
                },
                crate::game_state::upgrade::UpgradeKind::SuitAttackDamageMultiply { suit, damage_multiplier } => {
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {:?} cards.", suit)
                },
                crate::game_state::upgrade::UpgradeKind::HandAttackDamageMultiply { tower_kind, damage_multiplier } => {
                    let tower_name = Self::get_english_tower_name(tower_kind);
                    format!("Attack Damage increases by x{damage_multiplier:.1} for {tower_name} towers.")
                },
                crate::game_state::upgrade::UpgradeKind::ShopSlotExpansion => "Adds 1 slot available for purchase in the shop.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollCountPlus => "Increases the number of rerolls available each round by 1.".to_string(),
                crate::game_state::upgrade::UpgradeKind::LowCardTowerDamageMultiply { damage_multiplier } => {
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with 3 or fewer cards.")
                },
                crate::game_state::upgrade::UpgradeKind::ShopItemPriceMinus => "Shop item prices are discounted.".to_string(),
                crate::game_state::upgrade::UpgradeKind::ShopRefreshPlus => "Shop refresh count increases by 1.".to_string(),
                crate::game_state::upgrade::UpgradeKind::NoRerollTowerAttackDamageMultiply { damage_multiplier } => {
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made without rerolling.")
                },
                crate::game_state::upgrade::UpgradeKind::EvenOddTowerAttackDamageMultiply { even, damage_multiplier } => {
                    let card_type = if *even { "even" } else { "odd" };
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {card_type} cards.")
                },
                crate::game_state::upgrade::UpgradeKind::FaceNumberCardTowerAttackDamageMultiply { face, damage_multiplier } => {
                    let card_type = if *face { "face" } else { "number" };
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made with {card_type} cards.")
                },
                crate::game_state::upgrade::UpgradeKind::ShortenStraightFlushTo4Cards => "Allows making straight flush with 4 cards.".to_string(),
                crate::game_state::upgrade::UpgradeKind::SkipRankForStraight => "Allows skipping one rank when making a straight.".to_string(),
                crate::game_state::upgrade::UpgradeKind::TreatSuitsAsSame => "Treats all suits as the same.".to_string(),
                crate::game_state::upgrade::UpgradeKind::RerollTowerAttackDamageMultiply { damage_multiplier } => {
                    format!("Attack Damage increases by x{damage_multiplier:.1} for towers made after rerolling.")
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
