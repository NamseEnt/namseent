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
}
