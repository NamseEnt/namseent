use crate::{
    asset::image::thumbnail as thumbnail_image,
    game_state::upgrade::Upgrade,
    thumbnail::{render_placeholder_thumbnail, render_sticker_image_with_shadow},
};
use namui::*;

const UPGRADE_STICKER_THUMBNAIL_STROKE: Px = px(6.0);

fn sticker_thumbnail(image: Image, width_height: Wh<Px>) -> RenderingTree {
    render_sticker_image_with_shadow(image, width_height, UPGRADE_STICKER_THUMBNAIL_STROKE, true)
}

impl Upgrade {
    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        match self {
            Upgrade::Cat(..) => sticker_thumbnail(thumbnail_image::CAT, width_height),
            Upgrade::Staff(..) => sticker_thumbnail(thumbnail_image::STAFF, width_height),
            Upgrade::LongSword(..) => sticker_thumbnail(thumbnail_image::LONG_SWORD, width_height),
            Upgrade::Mace(..) => sticker_thumbnail(thumbnail_image::MACE, width_height),
            Upgrade::ClubSword(..) => sticker_thumbnail(thumbnail_image::CLUB_SWORD, width_height),
            Upgrade::Backpack(..) => sticker_thumbnail(thumbnail_image::BACKPACK, width_height),
            Upgrade::DiceBundle(..) => sticker_thumbnail(thumbnail_image::DICE_BUNDLE, width_height),
            Upgrade::Tricycle(..) => sticker_thumbnail(thumbnail_image::TRICYCLE, width_height),
            Upgrade::EnergyDrink(..) => sticker_thumbnail(thumbnail_image::ENERGY_DRINK, width_height),
            Upgrade::PerfectPottery(..) => sticker_thumbnail(thumbnail_image::PERFECT_POTTERY, width_height),
            Upgrade::SingleChopstick(..) => sticker_thumbnail(thumbnail_image::SINGLE_CHOPSTICK, width_height),
            Upgrade::PairChopsticks(..) => sticker_thumbnail(thumbnail_image::PAIR_CHOPSTICK, width_height),
            Upgrade::FountainPen(..) => sticker_thumbnail(thumbnail_image::FOUNTAIN_PEN, width_height),
            Upgrade::Brush(..) => sticker_thumbnail(thumbnail_image::BRUSH, width_height),
            Upgrade::FourLeafClover(..) => sticker_thumbnail(thumbnail_image::FOUR_LEAF_CLOVER, width_height),
            Upgrade::Rabbit(..) => sticker_thumbnail(thumbnail_image::RABBIT, width_height),
            Upgrade::BlackWhite(..) => sticker_thumbnail(thumbnail_image::BLACK_WHITE, width_height),
            Upgrade::Eraser(..) => sticker_thumbnail(thumbnail_image::ERASER, width_height),
            Upgrade::BrokenPottery(..) => sticker_thumbnail(thumbnail_image::BROKEN_POTTERY, width_height),
            Upgrade::Trophy(..) | Upgrade::Crock(..) | Upgrade::DemolitionHammer(..) | Upgrade::Metronome(..) | Upgrade::Tape(..) | Upgrade::NameTag(..) | Upgrade::ShoppingBag(..) | Upgrade::Resolution(..) | Upgrade::Mirror(..) | Upgrade::IceCream(..) | Upgrade::Spanner(..) | Upgrade::Pea(..) | Upgrade::SlotMachine(..) | Upgrade::PiggyBank(..) | Upgrade::Camera(..) | Upgrade::GiftBox(..) | Upgrade::Fang(..) | Upgrade::Popcorn(..) | Upgrade::MembershipCard(..) => {
                render_placeholder_thumbnail(width_height, UPGRADE_STICKER_THUMBNAIL_STROKE, true)
            }
        }
    }
}
