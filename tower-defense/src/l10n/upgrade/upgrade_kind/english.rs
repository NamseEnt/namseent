use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Magnet => {
                    builder.static_text("Magnet");
                }
                crate::game_state::upgrade::UpgradeKind::CainSword { .. } => {
                    builder.static_text("Cain Sword");
                }
                crate::game_state::upgrade::UpgradeKind::LongSword { .. } => {
                    builder.static_text("Long Sword");
                }
                crate::game_state::upgrade::UpgradeKind::Mace { .. } => {
                    builder.static_text("Mace");
                }
                crate::game_state::upgrade::UpgradeKind::ClubSword { .. } => {
                    builder.static_text("Club");
                }
                crate::game_state::upgrade::UpgradeKind::Backpack => {
                    builder.static_text("Backpack");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle => {
                    builder.static_text("Dice Bundle");
                }
                crate::game_state::upgrade::UpgradeKind::Spoon { .. } => {
                    builder.static_text("Spoon");
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink => {
                    builder.static_text("Energy Drink");
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { .. } => {
                    builder.static_text("Perfect Pottery");
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { .. } => {
                    builder.static_text("Single Chopstick");
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { .. } => {
                    builder.static_text("Pair Chopsticks");
                }
                crate::game_state::upgrade::UpgradeKind::FountainPen { .. } => {
                    builder.static_text("Fountain Pen");
                }
                crate::game_state::upgrade::UpgradeKind::Brush { .. } => {
                    builder.static_text("Brush");
                }
                crate::game_state::upgrade::UpgradeKind::FourLeafClover => {
                    builder.static_text("Four Leaf Clover");
                }
                crate::game_state::upgrade::UpgradeKind::Rabbit => {
                    builder.static_text("Rabbit");
                }
                crate::game_state::upgrade::UpgradeKind::BlackWhite => {
                    builder.static_text("Black & White");
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => {
                    builder.static_text("Broken Pottery");
                }
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Magnet => {
                    builder
                        .static_text("Increases ")
                        .with_gold_icon("gold")
                        .static_text(" earned when defeating monsters.");
                }
                crate::game_state::upgrade::UpgradeKind::CainSword { damage_multiplier }
                | crate::game_state::upgrade::UpgradeKind::LongSword { damage_multiplier }
                | crate::game_state::upgrade::UpgradeKind::Mace { damage_multiplier }
                | crate::game_state::upgrade::UpgradeKind::ClubSword { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built with specific suit cards by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::Backpack => {
                    builder
                        .static_text("Adds ")
                        .with_positive_effect("1 slot")
                        .static_text(" available for shop purchases.");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle => {
                    builder
                        .static_text("Increases the number of dice available each round by ")
                        .with_positive_effect("1")
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::Spoon { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built with 3 or fewer cards by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built without reroll by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built with odd cards by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built with even cards by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::FountainPen { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built with number cards by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::Brush { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of towers built with face cards by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { damage_multiplier } => {
                    builder
                        .static_text("Increases attack damage of rerolled towers by ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(".");
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink => {
                    builder.static_text("Reduces shop item prices.");
                }
                crate::game_state::upgrade::UpgradeKind::FourLeafClover => {
                    builder.static_text("Allows straight flush with 4 cards.");
                }
                crate::game_state::upgrade::UpgradeKind::Rabbit => {
                    builder.static_text("Allows skipping one rank when making a straight.");
                }
                crate::game_state::upgrade::UpgradeKind::BlackWhite => {
                    builder.static_text("Treats all suits as the same.");
                }
            },
        }
    }
}
