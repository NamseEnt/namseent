use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Cat { .. } => {
                    builder.static_text("Cat");
                }
                crate::game_state::upgrade::UpgradeKind::Staff { .. } => {
                    builder.static_text("Staff");
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
                crate::game_state::upgrade::UpgradeKind::Backpack { .. } => {
                    builder.static_text("Backpack");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle { .. } => {
                    builder.static_text("Dice Bundle");
                }
                crate::game_state::upgrade::UpgradeKind::Tricycle { .. } => {
                    builder.static_text("Tricycle");
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink { .. } => {
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
                crate::game_state::upgrade::UpgradeKind::Eraser { .. } => {
                    builder.static_text("Eraser");
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => {
                    builder.static_text("Broken Pottery");
                }
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Cat { add } => {
                    builder
                        .static_text("Gain ")
                        .with_icon_bold(IconKind::Gold, format!("+{add}"))
                        .static_text(" on monster kills");
                }
                crate::game_state::upgrade::UpgradeKind::Staff { damage_multiplier } => {
                    builder
                        .static_text("Diamond tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::LongSword { damage_multiplier } => {
                    builder
                        .static_text("Spade tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::Mace { damage_multiplier } => {
                    builder
                        .static_text("Heart tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::ClubSword { damage_multiplier } => {
                    builder
                        .static_text("Club tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::Backpack { add } => {
                    builder
                        .static_text("Shop slot ")
                        .with_icon_bold(IconKind::Shop, format!("+{add}"));
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle { add } => {
                    builder
                        .static_text("Dice ")
                        .with_icon_bold(IconKind::Refresh, format!("+{add}"));
                }
                crate::game_state::upgrade::UpgradeKind::Tricycle { damage_multiplier } => {
                    builder
                        .static_text("3-card tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { damage_multiplier } => {
                    builder
                        .static_text("No-reroll tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { damage_multiplier } => {
                    builder
                        .static_text("Odd-card tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { damage_multiplier } => {
                    builder
                        .static_text("Even-card tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::FountainPen { damage_multiplier } => {
                    builder
                        .static_text("Number-card tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::Brush { damage_multiplier } => {
                    builder
                        .static_text("Face-card tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { damage_multiplier } => {
                    builder
                        .static_text("Rerolled tower ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink { add } => {
                    builder
                        .static_text("Shop price ")
                        .with_icon_bold(IconKind::Gold, format!("-{add}"));
                }
                crate::game_state::upgrade::UpgradeKind::Eraser { add } => {
                    builder
                        .static_text("Remove ")
                        .with_positive_effect(format!("{add} rank"))
                        .static_text(" from the deck");
                }
                crate::game_state::upgrade::UpgradeKind::FourLeafClover => {
                    builder.static_text("Enable straight flush with 4 cards");
                }
                crate::game_state::upgrade::UpgradeKind::Rabbit => {
                    builder.static_text("Skip one rank in a straight");
                }
                crate::game_state::upgrade::UpgradeKind::BlackWhite => {
                    builder.static_text("Treat all suits as one");
                }
            },
        }
    }
}
