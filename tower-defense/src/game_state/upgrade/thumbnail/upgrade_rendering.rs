use crate::{
    asset::image::thumbnail as thumbnail_image, game_state::upgrade::UpgradeKind,
    thumbnail::ThumbnailComposer,
};
use namui::*;

impl UpgradeKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            UpgradeKind::Cat => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::CAT)
                .build(),
            UpgradeKind::CainSword { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::CAIN_SWORD)
                .build(),
            UpgradeKind::LongSword { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::LONG_SWORD)
                .build(),
            UpgradeKind::Mace { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::MACE)
                .build(),
            UpgradeKind::ClubSword { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::CLUB_SWORD)
                .build(),
            UpgradeKind::Backpack => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::BACKPACK)
                .build(),
            UpgradeKind::DiceBundle => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::DICE_BUNDLE)
                .build(),
            UpgradeKind::Tricycle { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::TRICYCLE)
                .build(),
            UpgradeKind::EnergyDrink => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::ENERGY_DRINK)
                .build(),
            UpgradeKind::PerfectPottery { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::PERFECT_POTTERY)
                .build(),
            UpgradeKind::SingleChopstick { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::SINGLE_CHOPSTICK)
                .build(),
            UpgradeKind::PairChopsticks { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::PAIR_CHOPSTICK)
                .build(),
            UpgradeKind::FountainPen { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::FOUNTAIN_PEN)
                .build(),
            UpgradeKind::Brush { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::BRUSH)
                .build(),
            UpgradeKind::FourLeafClover => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::FOUR_LEAF_CLOVER)
                .build(),
            UpgradeKind::Rabbit => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::RABBIT)
                .build(),
            UpgradeKind::BlackWhite => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::BLACK_WHITE)
                .build(),
            UpgradeKind::Eraser => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::ERASER)
                .build(),
            UpgradeKind::BrokenPottery { .. } => ThumbnailComposer::new(width_height)
                .with_image_base(thumbnail_image::BROKEN_POTTERY)
                .build(),
        }
    }

    pub fn thumbnail_image(&self) -> Image {
        match self {
            UpgradeKind::Cat => thumbnail_image::CAT,
            UpgradeKind::CainSword { .. } => thumbnail_image::CAIN_SWORD,
            UpgradeKind::LongSword { .. } => thumbnail_image::LONG_SWORD,
            UpgradeKind::Mace { .. } => thumbnail_image::MACE,
            UpgradeKind::ClubSword { .. } => thumbnail_image::CLUB_SWORD,
            UpgradeKind::Backpack => thumbnail_image::BACKPACK,
            UpgradeKind::DiceBundle => thumbnail_image::DICE_BUNDLE,
            UpgradeKind::Tricycle { .. } => thumbnail_image::TRICYCLE,
            UpgradeKind::EnergyDrink => thumbnail_image::ENERGY_DRINK,
            UpgradeKind::PerfectPottery { .. } => thumbnail_image::PERFECT_POTTERY,
            UpgradeKind::SingleChopstick { .. } => thumbnail_image::SINGLE_CHOPSTICK,
            UpgradeKind::PairChopsticks { .. } => thumbnail_image::PAIR_CHOPSTICK,
            UpgradeKind::FountainPen { .. } => thumbnail_image::FOUNTAIN_PEN,
            UpgradeKind::Brush { .. } => thumbnail_image::BRUSH,
            UpgradeKind::FourLeafClover => thumbnail_image::FOUR_LEAF_CLOVER,
            UpgradeKind::Rabbit => thumbnail_image::RABBIT,
            UpgradeKind::BlackWhite => thumbnail_image::BLACK_WHITE,
            UpgradeKind::Eraser => thumbnail_image::ERASER,
            UpgradeKind::BrokenPottery { .. } => thumbnail_image::BROKEN_POTTERY,
        }
    }
}
