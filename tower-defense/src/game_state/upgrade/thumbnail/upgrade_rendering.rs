use crate::{
    asset::image::thumbnail as thumbnail_image,
    game_state::upgrade::UpgradeKind,
    thumbnail::{render_placeholder_thumbnail, render_sticker_image_with_shadow},
};
use namui::*;

const UPGRADE_STICKER_THUMBNAIL_STROKE: Px = px(6.0);

fn sticker_thumbnail(image: Image, width_height: Wh<Px>) -> RenderingTree {
    render_sticker_image_with_shadow(image, width_height, UPGRADE_STICKER_THUMBNAIL_STROKE, true)
}

impl UpgradeKind {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            UpgradeKind::Cat(_) => sticker_thumbnail(thumbnail_image::CAT, width_height),
            UpgradeKind::Staff(_) => sticker_thumbnail(thumbnail_image::STAFF, width_height),
            UpgradeKind::LongSword(_) => {
                sticker_thumbnail(thumbnail_image::LONG_SWORD, width_height)
            }
            UpgradeKind::Mace(_) => sticker_thumbnail(thumbnail_image::MACE, width_height),
            UpgradeKind::ClubSword(_) => {
                sticker_thumbnail(thumbnail_image::CLUB_SWORD, width_height)
            }
            UpgradeKind::Backpack(_) => {
                sticker_thumbnail(thumbnail_image::BACKPACK, width_height)
            }
            UpgradeKind::DiceBundle(_) => {
                sticker_thumbnail(thumbnail_image::DICE_BUNDLE, width_height)
            }
            UpgradeKind::Tricycle(_) => {
                sticker_thumbnail(thumbnail_image::TRICYCLE, width_height)
            }
            UpgradeKind::EnergyDrink(_) => {
                sticker_thumbnail(thumbnail_image::ENERGY_DRINK, width_height)
            }
            UpgradeKind::PerfectPottery(_) => {
                sticker_thumbnail(thumbnail_image::PERFECT_POTTERY, width_height)
            }
            UpgradeKind::SingleChopstick(_) => {
                sticker_thumbnail(thumbnail_image::SINGLE_CHOPSTICK, width_height)
            }
            UpgradeKind::PairChopsticks(_) => {
                sticker_thumbnail(thumbnail_image::PAIR_CHOPSTICK, width_height)
            }
            UpgradeKind::FountainPen(_) => {
                sticker_thumbnail(thumbnail_image::FOUNTAIN_PEN, width_height)
            }
            UpgradeKind::Brush(_) => sticker_thumbnail(thumbnail_image::BRUSH, width_height),
            UpgradeKind::FourLeafClover(_) => {
                sticker_thumbnail(thumbnail_image::FOUR_LEAF_CLOVER, width_height)
            }
            UpgradeKind::Rabbit(_) => sticker_thumbnail(thumbnail_image::RABBIT, width_height),
            UpgradeKind::BlackWhite(_) => {
                sticker_thumbnail(thumbnail_image::BLACK_WHITE, width_height)
            }
            UpgradeKind::Trophy(_)
            | UpgradeKind::Crock(_)
            | UpgradeKind::DemolitionHammer(_)
            | UpgradeKind::Metronome(_)
            | UpgradeKind::Tape(_)
            | UpgradeKind::NameTag(_)
            | UpgradeKind::ShoppingBag(_)
            | UpgradeKind::Resolution(_)
            | UpgradeKind::Mirror(_)
            | UpgradeKind::IceCream(_)
            | UpgradeKind::Spanner(_)
            | UpgradeKind::Pea(_)
            | UpgradeKind::SlotMachine(_)
            | UpgradeKind::PiggyBank(_)
            | UpgradeKind::Camera(_)
            | UpgradeKind::GiftBox(_)
            | UpgradeKind::Fang(_)
            | UpgradeKind::Popcorn(_)
            | UpgradeKind::MembershipCard(_) => {
                render_placeholder_thumbnail(width_height, UPGRADE_STICKER_THUMBNAIL_STROKE, true)
            }
            UpgradeKind::Eraser(_) => sticker_thumbnail(thumbnail_image::ERASER, width_height),
            UpgradeKind::BrokenPottery(_) => {
                sticker_thumbnail(thumbnail_image::BROKEN_POTTERY, width_height)
            }
        }
    }
}
