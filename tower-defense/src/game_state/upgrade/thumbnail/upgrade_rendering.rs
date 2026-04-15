use crate::{
    asset::image::thumbnail as thumbnail_image, game_state::upgrade::UpgradeKind,
    thumbnail::render_sticker_image_with_shadow,
};
use namui::*;

const UPGRADE_STICKER_THUMBNAIL_STROKE: Px = px(6.0);

fn sticker_thumbnail(image: Image, width_height: Wh<Px>) -> RenderingTree {
    render_sticker_image_with_shadow(image, width_height, UPGRADE_STICKER_THUMBNAIL_STROKE, true)
}

impl UpgradeKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            UpgradeKind::Cat => sticker_thumbnail(thumbnail_image::CAT, width_height),
            UpgradeKind::CainSword { .. } => {
                sticker_thumbnail(thumbnail_image::CAIN_SWORD, width_height)
            }
            UpgradeKind::LongSword { .. } => {
                sticker_thumbnail(thumbnail_image::LONG_SWORD, width_height)
            }
            UpgradeKind::Mace { .. } => sticker_thumbnail(thumbnail_image::MACE, width_height),
            UpgradeKind::ClubSword { .. } => {
                sticker_thumbnail(thumbnail_image::CLUB_SWORD, width_height)
            }
            UpgradeKind::Backpack => sticker_thumbnail(thumbnail_image::BACKPACK, width_height),
            UpgradeKind::DiceBundle => {
                sticker_thumbnail(thumbnail_image::DICE_BUNDLE, width_height)
            }
            UpgradeKind::Tricycle { .. } => {
                sticker_thumbnail(thumbnail_image::TRICYCLE, width_height)
            }
            UpgradeKind::EnergyDrink => {
                sticker_thumbnail(thumbnail_image::ENERGY_DRINK, width_height)
            }
            UpgradeKind::PerfectPottery { .. } => {
                sticker_thumbnail(thumbnail_image::PERFECT_POTTERY, width_height)
            }
            UpgradeKind::SingleChopstick { .. } => {
                sticker_thumbnail(thumbnail_image::SINGLE_CHOPSTICK, width_height)
            }
            UpgradeKind::PairChopsticks { .. } => {
                sticker_thumbnail(thumbnail_image::PAIR_CHOPSTICK, width_height)
            }
            UpgradeKind::FountainPen { .. } => {
                sticker_thumbnail(thumbnail_image::FOUNTAIN_PEN, width_height)
            }
            UpgradeKind::Brush { .. } => sticker_thumbnail(thumbnail_image::BRUSH, width_height),
            UpgradeKind::FourLeafClover => {
                sticker_thumbnail(thumbnail_image::FOUR_LEAF_CLOVER, width_height)
            }
            UpgradeKind::Rabbit => sticker_thumbnail(thumbnail_image::RABBIT, width_height),
            UpgradeKind::BlackWhite => {
                sticker_thumbnail(thumbnail_image::BLACK_WHITE, width_height)
            }
            UpgradeKind::Eraser => sticker_thumbnail(thumbnail_image::ERASER, width_height),
            UpgradeKind::BrokenPottery { .. } => {
                sticker_thumbnail(thumbnail_image::BROKEN_POTTERY, width_height)
            }
        }
    }
}
