use crate::{
    asset::image::thumbnail as thumbnail_image, game_state::upgrade::Upgrade,
    thumbnail::render_sticker_image_with_shadow,
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
            Upgrade::DiceBundle(..) => {
                sticker_thumbnail(thumbnail_image::DICE_BUNDLE, width_height)
            }
            Upgrade::Tricycle(..) => sticker_thumbnail(thumbnail_image::TRICYCLE, width_height),
            Upgrade::EnergyDrink(..) => {
                sticker_thumbnail(thumbnail_image::ENERGY_DRINK, width_height)
            }
            Upgrade::PerfectPottery(..) => {
                sticker_thumbnail(thumbnail_image::PERFECT_POTTERY, width_height)
            }
            Upgrade::SingleChopstick(..) => {
                sticker_thumbnail(thumbnail_image::SINGLE_CHOPSTICK, width_height)
            }
            Upgrade::PairChopsticks(..) => {
                sticker_thumbnail(thumbnail_image::PAIR_CHOPSTICK, width_height)
            }
            Upgrade::FountainPen(..) => {
                sticker_thumbnail(thumbnail_image::FOUNTAIN_PEN, width_height)
            }
            Upgrade::Brush(..) => sticker_thumbnail(thumbnail_image::BRUSH, width_height),
            Upgrade::FourLeafClover(..) => {
                sticker_thumbnail(thumbnail_image::FOUR_LEAF_CLOVER, width_height)
            }
            Upgrade::Rabbit(..) => sticker_thumbnail(thumbnail_image::RABBIT, width_height),
            Upgrade::BlackWhite(..) => {
                sticker_thumbnail(thumbnail_image::BLACK_WHITE, width_height)
            }
            Upgrade::Eraser(..) => sticker_thumbnail(thumbnail_image::ERASER, width_height),
            Upgrade::BrokenPottery(..) => {
                sticker_thumbnail(thumbnail_image::BROKEN_POTTERY, width_height)
            }
            Upgrade::Trophy(..) => sticker_thumbnail(thumbnail_image::TROPHY, width_height),
            Upgrade::Crock(..) => sticker_thumbnail(thumbnail_image::CROCK, width_height),
            Upgrade::DemolitionHammer(..) => {
                sticker_thumbnail(thumbnail_image::DEMOLITION_HAMMER, width_height)
            }
            Upgrade::Metronome(..) => sticker_thumbnail(thumbnail_image::METRONOME, width_height),
            Upgrade::Tape(..) => sticker_thumbnail(thumbnail_image::TAPE, width_height),
            Upgrade::NameTag(..) => sticker_thumbnail(thumbnail_image::NAME_TAG, width_height),
            Upgrade::ShoppingBag(..) => {
                sticker_thumbnail(thumbnail_image::SHOPPING_BAG, width_height)
            }
            Upgrade::Resolution(..) => sticker_thumbnail(thumbnail_image::RESOLUTION, width_height),
            Upgrade::Mirror(..) => sticker_thumbnail(thumbnail_image::MIRROR, width_height),
            Upgrade::IceCream(..) => sticker_thumbnail(thumbnail_image::ICE_CREAM, width_height),
            Upgrade::Spanner(..) => sticker_thumbnail(thumbnail_image::SPANNER, width_height),
            Upgrade::Pea(..) => sticker_thumbnail(thumbnail_image::PEA, width_height),
            Upgrade::SlotMachine(..) => {
                sticker_thumbnail(thumbnail_image::SLOT_MACHINE, width_height)
            }
            Upgrade::PiggyBank(..) => sticker_thumbnail(thumbnail_image::PIGGY_BANK, width_height),
            Upgrade::Camera(..) => sticker_thumbnail(thumbnail_image::CAMERA, width_height),
            Upgrade::GiftBox(..) => sticker_thumbnail(thumbnail_image::GIFT_BOX, width_height),
            Upgrade::Fang(..) => sticker_thumbnail(thumbnail_image::FANG, width_height),
            Upgrade::Popcorn(..) => sticker_thumbnail(thumbnail_image::POPCORN, width_height),
            Upgrade::MembershipCard(..) => {
                sticker_thumbnail(thumbnail_image::MEMBERSHIP_CARD, width_height)
            }
        }
    }
}
