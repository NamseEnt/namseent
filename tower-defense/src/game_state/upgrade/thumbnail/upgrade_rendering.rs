use crate::{
    asset::image::thumbnail as thumbnail_image,
    game_state::upgrade::UpgradeKind,
    thumbnail::{render_sticker_image, STICKER_THUMBNAIL_STROKE},
};
use namui::*;

impl UpgradeKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            UpgradeKind::Cat => render_sticker_image(thumbnail_image::CAT, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::CainSword { .. } => render_sticker_image(thumbnail_image::CAIN_SWORD, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::LongSword { .. } => render_sticker_image(thumbnail_image::LONG_SWORD, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::Mace { .. } => render_sticker_image(thumbnail_image::MACE, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::ClubSword { .. } => render_sticker_image(thumbnail_image::CLUB_SWORD, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::Backpack => render_sticker_image(thumbnail_image::BACKPACK, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::DiceBundle => render_sticker_image(thumbnail_image::DICE_BUNDLE, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::Tricycle { .. } => render_sticker_image(thumbnail_image::TRICYCLE, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::EnergyDrink => render_sticker_image(thumbnail_image::ENERGY_DRINK, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::PerfectPottery { .. } => render_sticker_image(thumbnail_image::PERFECT_POTTERY, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::SingleChopstick { .. } => render_sticker_image(thumbnail_image::SINGLE_CHOPSTICK, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::PairChopsticks { .. } => render_sticker_image(thumbnail_image::PAIR_CHOPSTICK, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::FountainPen { .. } => render_sticker_image(thumbnail_image::FOUNTAIN_PEN, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::Brush { .. } => render_sticker_image(thumbnail_image::BRUSH, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::FourLeafClover => render_sticker_image(thumbnail_image::FOUR_LEAF_CLOVER, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::Rabbit => render_sticker_image(thumbnail_image::RABBIT, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::BlackWhite => render_sticker_image(thumbnail_image::BLACK_WHITE, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::Eraser => render_sticker_image(thumbnail_image::ERASER, width_height, STICKER_THUMBNAIL_STROKE),
            UpgradeKind::BrokenPottery { .. } => render_sticker_image(thumbnail_image::BROKEN_POTTERY, width_height, STICKER_THUMBNAIL_STROKE),
        }
    }
}
